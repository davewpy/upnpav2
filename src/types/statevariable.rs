// ===========================================================================
// UPnP AV 2.0 — State Variable Schema + Runtime Types
// ===========================================================================
//
// Two-level model:
//   Level 1 — Schema (static, declarative): StateVariableSchema carries metadata
//              from the UPnP spec tables as struct fields.
//   Level 2 — Runtime (dynamic, execution-time): StateVariableValue holds current
//              value + change tracking per instance.
//
// Per-service name enums (StateVariableName) are defined in services/*/static.rs.
// They implement Display for string lookup but NO trait — metadata is in the schema.

use crate::types::upnp::DataType;

// ---------------------------------------------------------------------------
// StateVariableSchema — declarative metadata from UPnP spec tables
// ---------------------------------------------------------------------------

/// Static definition of one state variable (per UPnP spec §4.1 tables).
///
/// This is data, not behavior. Each field corresponds to a column in the
/// UPnP spec's state variable table: name, eventing status, instance scoping,
/// data type, and default value.
#[derive(Debug, Clone)]
pub struct StateVariableSchema {
    /// The UPnP wire format name (e.g., "TransportState").
    pub name: &'static str,
    /// Whether this variable sends events directly via GENA NOTIFY.
    /// Per spec: LastChange (AVT), AllowedDefaultTransformSettings + DefaultTransformSettings (RC),
    /// DeviceClockInfoUpdates (CM).
    pub is_evented: bool,
    /// Whether this variable is collated into the LastChange XML payload.
    /// All non-position, non-A_ARG_TYPE vars with is_evented=—, via_lastchange=YES.
    pub via_lastchange: bool,
    /// Whether this variable is scoped per InstanceID (InstanceID > 0).
    /// InstanceID=0 is global/post-mix; InstanceID>0 is per-stream.
    pub is_instance_scoped: bool,
    /// UPnP wire type name for SCPD XML generation (e.g., "string", "ui4", "boolean").
    pub data_type_name: &'static str,
    /// Default value as a string literal. Converted to DataType at runtime via
    /// `DataType::from_default_str(data_type_name, default)`.
    /// Use `None` for variables with no meaningful default (e.g., URIs).
    pub default: Option<&'static str>,
    /// SCPD metadata (optional, used for XML generation).
    pub allowed_values: Option<&'static [&'static str]>,
    pub allowed_value_range: Option<(String, String, String)>,
    /// If true, this is an A_ARG_TYPE variable (type definition, not a real state var).
    /// Emitted in SCPD but without defaultValue.
    pub argument_type: bool,
}

impl StateVariableSchema {
    /// Create a basic schema with no allowed values or range.
    pub fn new(name: &'static str, default: Option<&'static str>) -> Self {
        Self {
            name,
            is_evented: false,
            via_lastchange: false,
            is_instance_scoped: false,
            data_type_name: "string",
            default,
            allowed_values: None,
            allowed_value_range: None,
            argument_type: false,
        }
    }
}

// ---------------------------------------------------------------------------
// StateVariableType — runtime state variable with current value
// ---------------------------------------------------------------------------

/// Runtime state variable with current value tracking.
///
/// Each state variable holds its current value and change tracking.
/// The `evented` property is defined in `StateSchema` and looked up
/// from `StateStore.schema()` when needed.
///
/// Writing the same value does NOT set `has_changes` per UPnP spec.
pub struct StateVariableType {
    /// Current value (typed via DataType enum variant).
    pub current_value: DataType,
    /// Whether this variable has pending changes since last event.
    pub has_changes: bool,
}

impl Clone for StateVariableType {
    fn clone(&self) -> Self {
        Self {
            current_value: self.current_value.clone(),
            has_changes: self.has_changes,
        }
    }
}

impl StateVariableType {
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
