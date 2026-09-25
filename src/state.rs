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

use crate::types::upnp::{DataType, Error};

// ---------------------------------------------------------------------------
// StateVariableName — trait for state variable name enums
// ---------------------------------------------------------------------------

/// Trait for state variable name enums.
///
/// Implemented by each service's `StateVariableName` enum to provide:
/// - `as_str()`: UPnP wire format name (e.g., "TransportState")
/// - `is_evented()`: Whether this variable sends events (direct or indirect) per UPnP spec
///   (LastChange, AllowedDefaultTransformSettings, DefaultTransformSettings, DeviceClockInfoUpdates)
/// - `via_lastchange()`: Whether this variable is indirectly evented via LastChange XML payload
/// - `is_instance_scoped()`: Whether this variable is per-InstanceID
/// - `data_type_name()`: UPnP wire type name (e.g., "string", "ui4") for SCPD XML generation
///
/// Per UPnP AV 2.0 spec, state variables have two eventing dimensions:
/// - `is_evented` column: YES = sends events (direct or indirect), NO = never evented
/// - `via_lastchange` column: — = direct NOTIFY, YES = collated into LastChange XML, NO = not evented
///
/// This trait enables compile-time type safety in `StateStore<S: StateVariableName>`.
pub trait StateVariableName: std::fmt::Display + Clone + Copy {
    /// Returns the UPnP wire format name (e.g., "TransportState").
    fn as_str(&self) -> &'static str;

    /// Returns true if this variable is evented per UPnP spec (is_evented=YES column).
    /// Includes both direct NOTIFY vars (LastChange, AllowedDefaultTransformSettings,
    /// DefaultTransformSettings, DeviceClockInfoUpdates) and via_lastchange vars.
    fn is_evented(&self) -> bool;

    /// Returns true if this variable is indirectly evented via LastChange XML payload.
    /// Per UPnP AV 2.0 spec: all non-position state vars with is_evented=—, via_lastchange=YES
    /// are collated into the LastChange event document and sent as one NOTIFY.
    fn via_lastchange(&self) -> bool;

    /// Returns true if this variable is scoped per InstanceID.
    /// InstanceID=0 is global/post-mix, InstanceID>0 is per-stream.
    fn is_instance_scoped(&self) -> bool;

    /// Returns the UPnP wire type name for SCPD XML generation (e.g., "string", "ui4", "boolean").
    /// The actual value used in the DataType enum is irrelevant — only the variant matters.
    fn data_type_name(&self) -> &'static str;
}

// ---------------------------------------------------------------------------
// StateSchema — compile-time definition for a state variable
// ---------------------------------------------------------------------------

/// Definition of a state variable — name, default value, SCPD metadata.
///
/// This is the compile-time contract that defines:
/// - Name (typed enum variant)
/// - Default value
/// - Allowed value range (min, max, step)
/// - Allowed values (fixed set)
/// - Whether it's an A_ARG_TYPE variable (type definition, not a real state var)
///
/// Eventing behavior is defined in the `StateVariableName` trait methods:
/// - `is_evented()`: direct GENA NOTIFY (LastChange, etc.)
/// - `via_lastchange()`: collated into LastChange XML
///
/// Each service registers its state variables at initialization time.
#[derive(Debug, Clone)]
pub struct StateSchema<S> {
    /// State variable name (typed enum variant).
    pub name: S,
    /// Default value.
    pub default: DataType,
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
            default: DataType::String(String::new()),
            allowed_values: None,
            allowed_value_range: None,
            argument_type: false,
        }
    }
}

/// Helper to create a StateSchema with optional SCPD fields.
/// Most registrations don't need allowed_values or allowed_value_range.
pub fn state_def<S: std::fmt::Display + Clone>(name: S, default: DataType) -> StateSchema<S> {
    StateSchema {
        name,
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
    /// Current value (typed via DataType enum variant).
    pub current_value: DataType,
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
    pub fn new(default: DataType) -> Self {
        Self {
            current_value: default,
            has_changes: false,
        }
    }

    /// Set the current value.
    ///
    /// Sets `has_changes = true` if the value actually changed.
    /// Writing the same value does NOT set `has_changes` per UPnP spec.
    pub fn set(&mut self, value: DataType) {
        if self.current_value != value {
            self.current_value = value;
            self.has_changes = true;
        }
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
/// - Type safety via Rust enum variants (compile-time)
/// - Batch operations with atomic apply
/// - Change tracking for GENA eventing
/// - SCPD name generation
///
/// Constraint: `S` must implement `StateVariableName` for compile-time
/// type safety — each state variable name knows its wire type name, eventing
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
    pub fn get(&self, name: S) -> Result<&DataType, Error> {
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
    pub fn get_owned(&self, name: S) -> Result<DataType, Error> {
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

    /// Set a state variable value.
    ///
    /// Type safety is compile-time: Rust enum variants enforce correct types.
    pub fn set(&mut self, name: S, value: DataType) {
        let key = name.as_str().to_string();
        if let Some(var) = self.variables.get_mut(&key) {
            var.set(value)
        }
    }

    /// Set multiple state variables atomically.
    ///
    /// All changes are recorded. Type safety is compile-time via Rust enum variants.
    pub fn set_batch(&mut self, pairs: Vec<(S, DataType)>) {
        for (name, value) in pairs {
            self.set(name, value);
        }
    }

    /// Get all evented state variables with their current values.
    ///
    /// Returns a list of (name, value) pairs suitable for GENA eventing.
    /// Includes variables where `is_evented()` is true OR `via_lastchange()` is true.
    pub fn collect_evented(&self) -> Vec<(String, String)> {
        self.variables
            .iter()
            .filter(|(name, _)| {
                // Variables with either direct eventing or via LastChange
                self.schema
                    .get(*name)
                    .map(|s| s.name.is_evented() || s.name.via_lastchange())
                    .unwrap_or(false)
            })
            .map(|(name, v)| (name.clone(), v.current_value.as_value_str()))
            .collect()
    }

    /// Get all evented variables that have pending changes.
    /// Uses `is_evented() || via_lastchange()` (compile-time).
    pub fn collect_changed_evented(&self) -> Vec<(String, String)> {
        self.variables
            .iter()
            .filter(|(name, v)| {
                self.schema.get(*name).map_or(false, |s| {
                    (s.name.is_evented() || s.name.via_lastchange()) && v.has_changes
                })
            })
            .map(|(name, v)| (name.clone(), v.current_value.as_value_str()))
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

    /// Get mutable access to a state variable's schema definition.
    pub fn schema_mut(&mut self, name: &str) -> Option<&mut StateSchema<S>> {
        self.schema.get_mut(name)
    }

    // =========================================================================
    // Instance Scoping Methods
    // =========================================================================

    /// Get the current value of an instance-scoped state variable for a specific InstanceID.
    ///
    /// Falls back to global (InstanceID=0) value if the instance doesn't exist yet.
    pub fn get_instance(&self, name: S, instance_id: u32) -> Result<&DataType, Error> {
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

    /// Set an instance-scoped state variable value.
    ///
    /// Creates the instance if it doesn't exist.
    /// Type safety is compile-time: Rust enum variants enforce correct types.
    pub fn set_instance(&mut self, name: S, instance_id: u32, value: DataType) {
        let key = name.as_str().to_string();

        // Ensure instance exists
        if !self.instance_states.contains_key(&instance_id) {
            self.create_instance(instance_id);
        }

        if let Some(instance_vars) = self.instance_states.get_mut(&instance_id) {
            if let Some(var) = instance_vars.get_mut(&key) {
                var.set(value)
            }
        }
    }

    /// Set multiple instance-scoped state variables atomically.
    ///
    /// All changes are recorded. Type safety is compile-time via Rust enum variants.
    pub fn set_instance_batch(&mut self, instance_id: u32, pairs: Vec<(S, DataType)>) {
        // Ensure instance exists
        if !self.instance_states.contains_key(&instance_id) {
            self.create_instance(instance_id);
        }

        // Apply all
        for (name, value) in pairs {
            self.set_instance(name, instance_id, value);
        }
    }

    /// Get all evented state variables for a specific instance.
    ///
    /// Returns a list of (name, value) pairs suitable for GENA eventing.
    /// For instance-scoped variables, returns values from the specified instance.
    /// Falls back to global values for non-instance-scoped variables.
    pub fn collect_evented_for_instance(&self, instance_id: u32) -> Vec<(String, String)> {
        let mut result = Vec::new();

        for (name, schema) in &self.schema {
            if !(schema.name.is_evented() || schema.name.via_lastchange()) {
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
                result.push((name.clone(), var.current_value.as_value_str()));
            }
        }

        result
    }

    /// Get all evented variables that have pending changes for a specific instance.
    pub fn collect_changed_evented_for_instance(&self, instance_id: u32) -> Vec<(String, String)> {
        let mut result = Vec::new();

        for (name, schema) in &self.schema {
            if !(schema.name.is_evented() || schema.name.via_lastchange()) {
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
                    result.push((name.clone(), var.current_value.as_value_str()));
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
