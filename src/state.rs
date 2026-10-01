// ===========================================================================
// UPnP AV 2.0 State Management — StateStore only
// ===========================================================================
//
// This module provides the runtime state registry for UPnP AV 2.0 services.
// Schema and variable primitives live in `types::statevariable`.
//
// Design principles:
// - StateStore manages all variables for a service with validation
// - Writing the same value does NOT set has_changes (per UPnP spec)

use std::collections::HashMap;
use std::marker::PhantomData;

use crate::types::statevariable::{StateVariableSchema, StateVariableType};
use crate::types::upnp::{DataType, Error};

// ===========================================================================
// StateStore — runtime registry of state variables for a service
// ===========================================================================

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
/// Generic over S (per-service StateVariableName enum) for type-safe keying.
/// Schema metadata is stored as data in StateVariableSchema, not as trait methods.
pub struct StateStore<S: std::fmt::Display + Clone + Copy> {
    _marker: PhantomData<S>,
    schema: HashMap<String, StateVariableSchema>,
    /// Global state variables (non-instance-scoped or InstanceID=0).
    variables: HashMap<String, StateVariableType>,
    /// Per-InstanceID state variables (InstanceID > 0).
    /// Outer key = InstanceID, inner key = state variable name.
    instance_states: HashMap<u32, HashMap<String, StateVariableType>>,
}

impl<S: std::fmt::Display + Clone + Copy> StateStore<S> {
    /// Create a new empty state store.
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
            schema: HashMap::new(),
            variables: HashMap::new(),
            instance_states: HashMap::new(),
        }
    }

    /// Register a state variable definition.
    pub fn register(&mut self, def: StateVariableSchema) {
        let default = DataType::from_default_str(def.data_type_name, def.default);
        let name = def.name.to_string();
        self.schema.insert(name.clone(), def);
        self.variables.insert(name, StateVariableType::new(default));
    }

    /// Register multiple state variable definitions.
    pub fn register_batch(&mut self, defs: Vec<StateVariableSchema>) {
        for def in defs {
            self.register(def);
        }
    }

    /// Get the current value of a state variable.
    pub fn get(&self, name: S) -> Result<&DataType, Error> {
        let key = format!("{}", name);
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
        let key = format!("{}", name);
        self.variables
            .get(&key)
            .map(|v| v.current_value.clone())
            .ok_or(Error::ArgumentValueInvalid)
    }

    /// Get the definition of a state variable by name (string).
    pub fn schema(&self, name: &str) -> Option<&StateVariableSchema> {
        self.schema.get(name)
    }

    /// Get a mutable reference to a state variable.
    pub fn get_mut(&mut self, name: S) -> Option<&mut StateVariableType> {
        let key = format!("{}", name);
        self.variables.get_mut(&key)
    }

    /// Get a clone of a state variable by name.
    ///
    /// Use this when the `StateStore` is behind a `MutexGuard` to avoid
    /// returning references that would outlive the guard.
    pub fn get_mut_owned(&self, name: S) -> Option<StateVariableType> {
        let key = format!("{}", name);
        self.variables.get(&key).cloned()
    }

    /// Set a state variable value.
    ///
    /// Type safety is compile-time: Rust enum variants enforce correct types.
    pub fn set(&mut self, name: S, value: DataType) {
        let key = format!("{}", name);
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
    /// Includes variables where `is_evented` is true OR `via_lastchange` is true.
    pub fn collect_evented(&self) -> Vec<(String, String)> {
        self.variables
            .iter()
            .filter(|(name, _)| {
                // Variables with either direct eventing or via LastChange
                self.schema
                    .get(name.as_str())
                    .map(|s| s.is_evented || s.via_lastchange)
                    .unwrap_or(false)
            })
            .map(|(name, v)| (name.clone(), v.current_value.as_value_str()))
            .collect()
    }

    /// Get all evented variables that have pending changes.
    /// Uses `is_evented || via_lastchange` from schema fields.
    pub fn collect_changed_evented(&self) -> Vec<(String, String)> {
        self.variables
            .iter()
            .filter(|(name, v)| {
                self.schema.get(name.as_str()).map_or(false, |s| {
                    (s.is_evented || s.via_lastchange) && v.has_changes
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
    pub fn schema_mut(&mut self, name: &str) -> Option<&mut StateVariableSchema> {
        self.schema.get_mut(name)
    }

    // =========================================================================
    // Instance Scoping Methods
    // =========================================================================

    /// Get the current value of an instance-scoped state variable for a specific InstanceID.
    ///
    /// Falls back to global (InstanceID=0) value if the instance doesn't exist yet.
    pub fn get_instance(&self, name: S, instance_id: u32) -> Result<&DataType, Error> {
        let key = format!("{}", name);

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
    pub fn get_instance_mut(
        &mut self,
        name: S,
        instance_id: u32,
    ) -> Option<&mut StateVariableType> {
        let key = format!("{}", name);

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
        for schema in self.schema.values() {
            if schema.is_instance_scoped {
                let default = DataType::from_default_str(schema.data_type_name, schema.default);
                instance_vars.insert(schema.name.to_string(), StateVariableType::new(default));
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
        let key = format!("{}", name);

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

        for schema in self.schema.values() {
            if !(schema.is_evented || schema.via_lastchange) {
                continue;
            }

            let value = if schema.is_instance_scoped {
                self.instance_states
                    .get(&instance_id)
                    .and_then(|vars| vars.get(schema.name))
                    .or_else(|| self.variables.get(schema.name))
            } else {
                self.variables.get(schema.name)
            };

            if let Some(var) = value {
                result.push((schema.name.to_string(), var.current_value.as_value_str()));
            }
        }

        result
    }

    /// Get all evented variables that have pending changes for a specific instance.
    pub fn collect_changed_evented_for_instance(&self, instance_id: u32) -> Vec<(String, String)> {
        let mut result = Vec::new();

        for schema in self.schema.values() {
            if !(schema.is_evented || schema.via_lastchange) {
                continue;
            }

            let var = if schema.is_instance_scoped {
                self.instance_states
                    .get(&instance_id)
                    .and_then(|vars| vars.get(schema.name))
                    .or_else(|| self.variables.get(schema.name))
            } else {
                self.variables.get(schema.name)
            };

            if let Some(var) = var {
                if var.has_changes {
                    result.push((schema.name.to_string(), var.current_value.as_value_str()));
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
