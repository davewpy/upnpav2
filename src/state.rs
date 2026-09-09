// ===========================================================================
// UPnP AV 2.0 State Management — StateSchema, StateVariable, StateStore
// ===========================================================================
//
// This module provides the runtime state management layer for UPnP AV 2.0
// services (AVTransport, RenderingControl, ConnectionManager).
//
// Components:
// - StateSchema: Compile-time definition (name, type, eventing, default)
// - StateVariable: Runtime variable with current value and change tracking
// - StateStore: Service-level registry of state variables
//
// Design principles:
// 1. StateSchema is the compile-time contract per state variable
// 2. StateVariable tracks current value and has_changes for GENA eventing
// 3. StateStore manages all variables for a service with validation
// 4. Writing the same value does NOT set has_changes (per UPnP spec)

use std::collections::HashMap;

use crate::types::upnp::{DataType, Error, StateValue};

// ---------------------------------------------------------------------------
// StateVariableName — trait for state variable name enums
// ---------------------------------------------------------------------------

/// Trait for state variable name enums.
///
/// Implemented by each service's `StateVariableName` enum to provide:
/// - `as_str()`: UPnP wire format name (e.g., "TransportState")
/// - `is_evented()`: Whether this variable sends events via LastChange
/// - `is_instance_scoped()`: Whether this variable is per-InstanceID
/// - `data_type()`: UPnP data type for validation
///
/// This trait enables compile-time type safety in `StateStore<S: StateVariableName>`.
pub trait StateVariableName: std::fmt::Display + Clone + Copy {
    /// Returns the UPnP wire format name (e.g., "TransportState").
    fn as_str(&self) -> &'static str;

    /// Returns true if this variable sends events via LastChange.
    fn is_evented(&self) -> bool;

    /// Returns true if this variable is scoped per InstanceID.
    /// InstanceID=0 is global/post-mix, InstanceID>0 is per-stream.
    fn is_instance_scoped(&self) -> bool;

    /// Returns the UPnP data type for validation.
    fn data_type(&self) -> DataType;
}

// ---------------------------------------------------------------------------
// StateSchema — compile-time definition for a state variable
// ---------------------------------------------------------------------------

/// Definition of a state variable — name, type, eventing, default value.
///
/// This is the compile-time contract that defines:
/// - Name (typed enum variant)
/// - Data type (UPnP wire type)
/// - Whether it's evented (via LastChange)
/// - Default value
/// - Allowed value range (min, max, step)
/// - Allowed values (fixed set)
/// - Whether it's an A_ARG_TYPE variable (type definition, not a real state var)
///
/// Each service registers its state variables at initialization time.
#[derive(Debug, Clone)]
pub struct StateSchema<S> {
    /// State variable name (typed enum variant).
    pub name: S,
    /// UPnP data type (string, ui4, boolean, etc.).
    pub data_type: DataType,
    /// Whether this variable sends events (via LastChange).
    pub send_events: bool,
    /// Default value.
    pub default: StateValue,
    /// SCPD metadata (optional, used for XML generation).
    pub allowed_values: Option<Vec<String>>,
    pub allowed_value_range: Option<(String, String, String)>,
    /// If true, this is an A_ARG_TYPE variable (type definition, not a real state var).
    /// Emitted in SCPD but without defaultValue.
    pub argument_type: bool,
}

impl<S: Default> Default for StateSchema<S> {
    fn default() -> Self {
        Self {
            name: S::default(),
            data_type: DataType::String,
            send_events: false,
            default: StateValue::String(String::new()),
            allowed_values: None,
            allowed_value_range: None,
            argument_type: false,
        }
    }
}

/// Helper to create a StateSchema with optional SCPD fields.
/// Most registrations don't need allowed_values or allowed_value_range.
pub fn state_def<S: std::fmt::Display + Clone>(
    name: S,
    data_type: DataType,
    send_events: bool,
    default: StateValue,
) -> StateSchema<S> {
    StateSchema {
        name,
        data_type,
        send_events,
        default,
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    }
}

// ---------------------------------------------------------------------------
// StateVariable — runtime state variable with current value
// ---------------------------------------------------------------------------

/// Runtime state variable with current value tracking.
///
/// Each state variable holds its current value and change tracking.
/// The `evented` property is defined in `StateSchema` and looked up
/// from `StateStore.schema()` when needed.
///
/// Writing the same value does NOT set `has_changes` per UPnP spec.
pub struct StateVariable {
    /// Current value.
    pub current_value: StateValue,
    /// Whether this variable has pending changes since last event.
    pub has_changes: bool,
}

impl Clone for StateVariable {
    fn clone(&self) -> Self {
        Self {
            current_value: self.current_value.clone(),
            has_changes: self.has_changes,
        }
    }
}

impl StateVariable {
    /// Create a new state variable with its default value.
    pub fn new(default: StateValue) -> Self {
        Self {
            current_value: default,
            has_changes: false,
        }
    }

    /// Set the current value with type validation.
    ///
    /// Sets `has_changes = true` if the value actually changed.
    /// Writing the same value does NOT set `has_changes` per UPnP spec.
    pub fn set(&mut self, value: StateValue) -> Result<(), Error> {
        if self.current_value != value {
            self.current_value = value;
            self.has_changes = true;
        }
        Ok(())
    }

    /// Mark this variable as having no pending changes (called after eventing).
    pub fn clear_changes(&mut self) {
        self.has_changes = false;
    }
}

// ---------------------------------------------------------------------------
// StateStore — runtime registry of state variables for a service
// ---------------------------------------------------------------------------

/// Runtime registry of state variables for a service.
///
/// Maps state variable names (typed enum) to their runtime values. Used by
/// action implementations to read/write state, and by GENA/SCPD handlers.
///
/// Key features:
/// - Type validation on every set operation (via StateVariableName trait)
/// - Batch operations with atomic apply
/// - Change tracking for GENA eventing
/// - SCPD name generation
///
/// Constraint: `S` must implement `StateVariableName` for compile-time
/// type safety — each state variable name knows its data type, eventing
/// status, and instance-scoping at compile time.
pub struct StateStore<S: StateVariableName> {
    schema: HashMap<String, StateSchema<S>>,
    /// Global state variables (non-instance-scoped or InstanceID=0).
    variables: HashMap<String, StateVariable>,
    /// Per-InstanceID state variables (InstanceID > 0).
    /// Outer key = InstanceID, inner key = state variable name.
    instance_states: HashMap<u32, HashMap<String, StateVariable>>,
}

impl<S: StateVariableName> StateStore<S> {
    /// Create a new empty state store.
    pub fn new() -> Self {
        Self {
            schema: HashMap::new(),
            variables: HashMap::new(),
            instance_states: HashMap::new(),
        }
    }

    /// Register a state variable definition.
    pub fn register(&mut self, def: StateSchema<S>) {
        let name = format!("{}", def.name);
        let default = def.default.clone();
        self.schema.insert(name.clone(), def);
        self.variables.insert(name, StateVariable::new(default));
    }

    /// Register multiple state variable definitions.
    pub fn register_batch(&mut self, defs: Vec<StateSchema<S>>) {
        for def in defs {
            self.register(def);
        }
    }

    /// Get the current value of a state variable.
    pub fn get(&self, name: S) -> Result<&StateValue, Error> {
        let key = name.as_str().to_string();
        self.variables
            .get(&key)
            .map(|v| &v.current_value)
            .ok_or(Error::ArgumentValueInvalid)
    }

    /// Get a clone of the current value of a state variable.
    ///
    /// Use this when the `StateStore` is behind a `MutexGuard` to avoid
    /// returning references that would outlive the guard.
    pub fn get_owned(&self, name: S) -> Result<StateValue, Error> {
        let key = name.as_str().to_string();
        self.variables
            .get(&key)
            .map(|v| v.current_value.clone())
            .ok_or(Error::ArgumentValueInvalid)
    }

    /// Get the definition of a state variable by name (string).
    pub fn schema(&self, name: &str) -> Option<&StateSchema<S>> {
        self.schema.get(name)
    }

    /// Get a mutable reference to a state variable.
    pub fn get_mut(&mut self, name: S) -> Option<&mut StateVariable> {
        let key = name.as_str().to_string();
        self.variables.get_mut(&key)
    }

    /// Get a clone of a state variable by name.
    ///
    /// Use this when the `StateStore` is behind a `MutexGuard` to avoid
    /// returning references that would outlive the guard.
    pub fn get_mut_owned(&self, name: S) -> Option<StateVariable> {
        let key = name.as_str().to_string();
        self.variables.get(&key).cloned()
    }

    /// Set a state variable value with type validation.
    ///
    /// Uses `StateVariableName::data_type()` for compile-time type lookup
    /// instead of runtime schema lookup.
    pub fn set(&mut self, name: S, value: StateValue) -> Result<(), Error> {
        let key = name.as_str().to_string();
        // Validate against trait's data_type (compile-time)
        value.validate(name.data_type())?;

        if let Some(var) = self.variables.get_mut(&key) {
            var.set(value)
        } else {
            Err(Error::ArgumentValueInvalid)
        }
    }

    /// Set multiple state variables atomically.
    ///
    /// All changes are recorded. If any set fails, none are applied.
    pub fn set_batch(&mut self, pairs: Vec<(S, StateValue)>) -> Result<(), Error> {
        // Validate all first (using trait's data_type)
        for (name, value) in &pairs {
            value.validate(name.data_type())?;
        }

        // Apply all
        for (name, value) in pairs {
            self.set(name, value)?;
        }
        Ok(())
    }

    /// Get all evented state variables with their current values.
    ///
    /// Returns a list of (name, value) pairs suitable for GENA eventing.
    /// Uses `StateVariableName::is_evented()` (compile-time).
    pub fn collect_evented(&self) -> Vec<(String, String)> {
        self.variables
            .iter()
            .filter(|(name, _)| {
                // Look up the name enum variant to check is_evented()
                self.schema
                    .get(*name)
                    .map(|s| s.name.is_evented())
                    .unwrap_or(false)
            })
            .map(|(name, v)| (name.clone(), v.current_value.as_str()))
            .collect()
    }

    /// Get all evented variables that have pending changes.
    /// Uses `StateVariableName::is_evented()` (compile-time).
    pub fn collect_changed_evented(&self) -> Vec<(String, String)> {
        self.variables
            .iter()
            .filter(|(name, v)| {
                self.schema
                    .get(*name)
                    .map_or(false, |s| s.name.is_evented() && v.has_changes)
            })
            .map(|(name, v)| (name.clone(), v.current_value.as_str()))
            .collect()
    }

    /// Clear has_changes on all variables (called after eventing).
    pub fn clear_all_changes(&mut self) {
        for var in self.variables.values_mut() {
            var.clear_changes();
        }
    }

    /// Get all variable names (for SCPD generation).
    pub fn names(&self) -> Vec<String> {
        self.variables.keys().cloned().collect()
    }

    // =========================================================================
    // Instance Scoping Methods
    // =========================================================================

    /// Get the current value of an instance-scoped state variable for a specific InstanceID.
    ///
    /// Falls back to global (InstanceID=0) value if the instance doesn't exist yet.
    pub fn get_instance(&self, name: S, instance_id: u32) -> Result<&StateValue, Error> {
        let key = name.as_str().to_string();

        // First check the specific instance
        if let Some(instance_vars) = self.instance_states.get(&instance_id) {
            if let Some(var) = instance_vars.get(&key) {
                return Ok(&var.current_value);
            }
        }

        // Fall back to global (InstanceID=0)
        if let Some(var) = self.variables.get(&key) {
            return Ok(&var.current_value);
        }

        Err(Error::ArgumentValueInvalid)
    }

    /// Get a mutable reference to an instance-scoped state variable.
    ///
    /// Creates the instance if it doesn't exist, initializing all instance-scoped
    /// variables with their default values from the schema.
    pub fn get_instance_mut(&mut self, name: S, instance_id: u32) -> Option<&mut StateVariable> {
        let key = name.as_str().to_string();

        // Ensure instance exists
        if !self.instance_states.contains_key(&instance_id) {
            self.create_instance(instance_id);
        }

        self.instance_states
            .get_mut(&instance_id)
            .and_then(|vars| vars.get_mut(&key))
    }

    /// Create a new instance with all instance-scoped state variables initialized to defaults.
    pub fn create_instance(&mut self, instance_id: u32) {
        if self.instance_states.contains_key(&instance_id) {
            return; // Instance already exists
        }

        let mut instance_vars = HashMap::new();

        // Initialize all instance-scoped variables with their defaults
        for (name, schema) in &self.schema {
            if schema.name.is_instance_scoped() {
                instance_vars.insert(name.clone(), StateVariable::new(schema.default.clone()));
            }
        }

        self.instance_states.insert(instance_id, instance_vars);
    }

    /// Remove an instance and all its state variables.
    pub fn remove_instance(&mut self, instance_id: u32) {
        self.instance_states.remove(&instance_id);
    }

    /// Set an instance-scoped state variable value with type validation.
    ///
    /// Creates the instance if it doesn't exist.
    pub fn set_instance(
        &mut self,
        name: S,
        instance_id: u32,
        value: StateValue,
    ) -> Result<(), Error> {
        let key = name.as_str().to_string();

        // Validate against trait's data_type (compile-time)
        value.validate(name.data_type())?;

        // Ensure instance exists
        if !self.instance_states.contains_key(&instance_id) {
            self.create_instance(instance_id);
        }

        if let Some(instance_vars) = self.instance_states.get_mut(&instance_id) {
            if let Some(var) = instance_vars.get_mut(&key) {
                var.set(value)
            } else {
                Err(Error::ArgumentValueInvalid)
            }
        } else {
            Err(Error::ArgumentValueInvalid)
        }
    }

    /// Set multiple instance-scoped state variables atomically.
    ///
    /// All changes are recorded. If any set fails, none are applied.
    pub fn set_instance_batch(
        &mut self,
        instance_id: u32,
        pairs: Vec<(S, StateValue)>,
    ) -> Result<(), Error> {
        // Validate all first (using trait's data_type)
        for (name, value) in &pairs {
            value.validate(name.data_type())?;
        }

        // Ensure instance exists
        if !self.instance_states.contains_key(&instance_id) {
            self.create_instance(instance_id);
        }

        // Apply all
        for (name, value) in pairs {
            self.set_instance(name, instance_id, value)?;
        }
        Ok(())
    }

    /// Get all evented state variables for a specific instance.
    ///
    /// Returns a list of (name, value) pairs suitable for GENA eventing.
    /// For instance-scoped variables, returns values from the specified instance.
    /// Falls back to global values for non-instance-scoped variables.
    pub fn collect_evented_for_instance(&self, instance_id: u32) -> Vec<(String, String)> {
        let mut result = Vec::new();

        for (name, schema) in &self.schema {
            if !schema.name.is_evented() {
                continue;
            }

            let value = if schema.name.is_instance_scoped() {
                self.instance_states
                    .get(&instance_id)
                    .and_then(|vars| vars.get(name))
                    .or_else(|| self.variables.get(name))
            } else {
                self.variables.get(name)
            };

            if let Some(var) = value {
                result.push((name.clone(), var.current_value.as_str()));
            }
        }

        result
    }

    /// Get all evented variables that have pending changes for a specific instance.
    pub fn collect_changed_evented_for_instance(&self, instance_id: u32) -> Vec<(String, String)> {
        let mut result = Vec::new();

        for (name, schema) in &self.schema {
            if !schema.name.is_evented() {
                continue;
            }

            let var = if schema.name.is_instance_scoped() {
                self.instance_states
                    .get(&instance_id)
                    .and_then(|vars| vars.get(name))
                    .or_else(|| self.variables.get(name))
            } else {
                self.variables.get(name)
            };

            if let Some(var) = var {
                if var.has_changes {
                    result.push((name.clone(), var.current_value.as_str()));
                }
            }
        }

        result
    }

    /// Clear has_changes on all variables for a specific instance (called after eventing).
    pub fn clear_instance_changes(&mut self, instance_id: u32) {
        if let Some(instance_vars) = self.instance_states.get_mut(&instance_id) {
            for var in instance_vars.values_mut() {
                var.clear_changes();
            }
        }
    }

    /// Get all instance IDs that have been created.
    pub fn instance_ids(&self) -> Vec<u32> {
        self.instance_states.keys().cloned().collect()
    }

    /// Check if an instance exists.
    pub fn has_instance(&self, instance_id: u32) -> bool {
        self.instance_states.contains_key(&instance_id)
    }

    /// Get the number of active instances.
    pub fn instance_count(&self) -> usize {
        self.instance_states.len()
    }
}
