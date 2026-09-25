/// UPnP service version (1, 2, or 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceVersion {
    V1 = 1,
    V2 = 2,
    V3 = 3,
}

/// UPnP service definition — name, base namespace, and version.
/// UPnP device type (per UDA 2.0 spec).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Device {
    /// MediaRenderer — renders media content
    MediaRenderer,
    /// MediaServer — stores and serves media content
    MediaServer,
    /// MediaController — controls MediaRenderer/MediaServer
    MediaController,
    /// Gateway — bridges different network types
    Gateway,
    /// Basic — minimal UPnP device
    Basic,
    /// ControlPoint — UPnP control point
    ControlPoint,
}

impl Device {
    /// Full device type URN: urn:schemas-upnp-org:device:<type>:1
    pub fn urn(&self) -> String {
        let name = match self {
            Self::MediaRenderer => "MediaRenderer",
            Self::MediaServer => "MediaServer",
            Self::MediaController => "MediaController",
            Self::Gateway => "Gateway",
            Self::Basic => "Basic",
            Self::ControlPoint => "ControlPoint",
        };
        format!("urn:schemas-upnp-org:device:{}:1", name)
    }
}

/// UPnP service definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Services {
    /// AVTransport service
    AVTransport(ServiceVersion),
    /// ConnectionManager service
    ConnectionManager(ServiceVersion),
    /// RenderingControl service
    RenderingControl(ServiceVersion),
}

impl Services {
    /// Service name (e.g., "AVTransport")
    pub const fn name(&self) -> &'static str {
        match self {
            Self::AVTransport(_) => "AVTransport",
            Self::ConnectionManager(_) => "ConnectionManager",
            Self::RenderingControl(_) => "RenderingControl",
        }
    }

    /// Base namespace without version (e.g., "urn:schemas-upnp-org:service:AVTransport").
    ///
    /// This is a `const fn` — no allocation, suitable for use in static contexts.
    pub const fn base_namespace(&self) -> &'static str {
        match self {
            Self::AVTransport(_) => "urn:schemas-upnp-org:service:AVTransport",
            Self::ConnectionManager(_) => "urn:schemas-upnp-org:service:ConnectionManager",
            Self::RenderingControl(_) => "urn:schemas-upnp-org:service:RenderingControl",
        }
    }

    /// Service version
    pub const fn version(&self) -> ServiceVersion {
        match self {
            Self::AVTransport(v) | Self::ConnectionManager(v) | Self::RenderingControl(v) => *v,
        }
    }

    /// Full namespace URN: `<base_namespace>:<version>` (e.g., `"urn:schemas-upnp-org:service:AVTransport:3"`).
    ///
    /// Allocates a `String` because the version suffix is dynamic.
    /// If you only need the base namespace, use `base_namespace()` instead.
    pub fn full_namespace(&self) -> String {
        format!("{}:{}", self.base_namespace(), self.version() as u32)
    }

    /// HTTP control URL for this service (e.g., "/upnp/control/AVTransport").
    pub fn url_control(&self) -> &'static str {
        match self {
            Self::AVTransport(_) => "/upnp/control/AVTransport",
            Self::ConnectionManager(_) => "/upnp/control/ConnectionManager",
            Self::RenderingControl(_) => "/upnp/control/RenderingControl",
        }
    }

    /// SCPD URL for this service (e.g., "/upnp/AVTransport.xml").
    pub fn scpd_url(&self) -> &'static str {
        match self {
            Self::AVTransport(_) => "/upnp/AVTransport.xml",
            Self::ConnectionManager(_) => "/upnp/ConnectionManager.xml",
            Self::RenderingControl(_) => "/upnp/RenderingControl.xml",
        }
    }

    /// Event subscription URL for this service (e.g., "/upnp/event/AVTransport").
    pub fn event_url(&self) -> &'static str {
        match self {
            Self::AVTransport(_) => "/upnp/event/AVTransport",
            Self::ConnectionManager(_) => "/upnp/event/ConnectionManager",
            Self::RenderingControl(_) => "/upnp/event/RenderingControl",
        }
    }

    /// Lookup service by name
    pub fn service_from_name(name: &str) -> Option<Services> {
        match name {
            "AVTransport" => Some(Services::AVTransport(ServiceVersion::V3)),
            "ConnectionManager" => Some(Services::ConnectionManager(ServiceVersion::V3)),
            "RenderingControl" => Some(Services::RenderingControl(ServiceVersion::V3)),
            _ => None,
        }
    }

    /// Lookup service by full namespace URN (e.g., "urn:schemas-upnp-org:service:AVTransport:1")
    pub fn namespace_from_string(urn: &str) -> Option<Services> {
        let (service_urn, ver) = urn.rsplit_once(':')?;
        let service_version = match ver {
            "1" => ServiceVersion::V1,
            "2" => ServiceVersion::V2,
            "3" => ServiceVersion::V3,
            _ => return None,
        };
        match service_urn {
            "urn:schemas-upnp-org:service:AVTransport" => {
                Some(Services::AVTransport(service_version))
            }
            "urn:schemas-upnp-org:service:ConnectionManager" => {
                Some(Services::ConnectionManager(service_version))
            }
            "urn:schemas-upnp-org:service:RenderingControl" => {
                Some(Services::RenderingControl(service_version))
            }
            _ => None,
        }
    }
}

/// UPnP standard data types (UPnP-av-2.0 spec §4.2).
///
/// Unified enum: each variant carries both the type tag AND its value.
/// At compile-time, the variant name is the type; at runtime, the inner field is the value.
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Ui1(u8),       // ui1
    Ui2(u16),      // ui2
    Ui4(u32),      // ui4
    I1(i8),        // i1
    I2(i16),       // i2
    I4(i32),       // i4
    I8(i64),       // i8
    Float(f32),    // r4 / float
    Double(f64),   // r8
    Decimal(String),
    Char(char),
    String(String),
    Date(String),
    DateTime(String),
    Boolean(bool),
    Base64(String),
    HexBinary(String),
    Uri(String),
    Uuid(String),
}

impl DataType {
    /// Returns the UPnP wire-format type name for SCPD XML generation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ui1(_) => "ui1",
            Self::Ui2(_) => "ui2",
            Self::Ui4(_) => "ui4",
            Self::I1(_) => "i1",
            Self::I2(_) => "i2",
            Self::I4(_) => "i4",
            Self::I8(_) => "i8",
            Self::Float(_) => "r4",
            Self::Double(_) => "r8",
            Self::Decimal(_) => "number",
            Self::Char(_) => "char",
            Self::String(_) => "string",
            Self::Date(_) => "date",
            Self::DateTime(_) => "dateTime",
            Self::Boolean(_) => "boolean",
            Self::Base64(_) => "bin.base64",
            Self::HexBinary(_) => "bin.hex",
            Self::Uri(_) => "uri",
            Self::Uuid(_) => "uuid",
        }
    }
}

/// Generic UPnP Device Architecture error codes (codes 400-699).
///
/// These are defined by the UPnP Device Architecture specification and apply
/// across all services (AVTransport, RenderingControl, ConnectionManager, etc.).
/// Service-specific error codes (700-799) are defined in their respective modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Error {
    /// No action by that name in service.
    InvalidAction = 401,
    /// Arguments are wrong (order, type, count).
    InvalidArgs = 402,
    /// General failure; state prevents execution.
    ActionFailed = 501,
    /// Argument value is not valid.
    ArgumentValueInvalid = 600,
    /// Value outside of allowed range.
    ArgumentValueOutOfRange = 601,
    /// Optional action not supported.
    OptionalActionNotImplemented = 602,
    /// Device cannot complete action.
    OutOfMemory = 603,
    /// User action needed (e.g., media slot empty).
    HumanInterventionRequired = 604,
    /// String exceeds maximum length.
    StringArgumentTooLong = 605,
}

impl Error {
    /// Returns the numeric error code.
    pub const fn code(&self) -> u16 {
        *self as u16
    }

    /// Returns the human-readable error description.
    pub const fn description(&self) -> &'static str {
        match self {
            Self::InvalidAction => "No action by that name in service",
            Self::InvalidArgs => "Arguments are wrong (order, type, count)",
            Self::ActionFailed => "General failure; state prevents execution",
            Self::ArgumentValueInvalid => "Argument value is not valid",
            Self::ArgumentValueOutOfRange => "Value outside of allowed range",
            Self::OptionalActionNotImplemented => "Optional action not supported",
            Self::OutOfMemory => "Device cannot complete action",
            Self::HumanInterventionRequired => "User action needed (for example, media slot empty)",
            Self::StringArgumentTooLong => "String exceeds maximum length",
        }
    }

    /// Lookup an error by its numeric code.
    pub const fn from_code(code: u16) -> Option<Self> {
        match code {
            401 => Some(Self::InvalidAction),
            402 => Some(Self::InvalidArgs),
            501 => Some(Self::ActionFailed),
            600 => Some(Self::ArgumentValueInvalid),
            601 => Some(Self::ArgumentValueOutOfRange),
            602 => Some(Self::OptionalActionNotImplemented),
            603 => Some(Self::OutOfMemory),
            604 => Some(Self::HumanInterventionRequired),
            605 => Some(Self::StringArgumentTooLong),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UPnP Error {}: {}", self.code(), self.description())
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone)]
pub enum ArgumentDirection {
    IN,
    OUT,
}

#[derive(Debug, Clone)]
pub struct ArgumentDefinition<Arg, S> {
    pub name: Arg,
    pub direction: ArgumentDirection,
    pub related_state_var: Option<S>,
}

#[derive(Debug, Clone)]
pub struct ActionDefinition<A, Arg, S> {
    pub name: A,
    pub in_args: Vec<ArgumentDefinition<Arg, S>>,
    pub out_args: Vec<ArgumentDefinition<Arg, S>>,
}

// ---------------------------------------------------------------------------
// DataType — unified type + value enum
// ---------------------------------------------------------------------------

/// Unified UPnP data type and value.
///
/// Each variant encodes both the UPnP wire type (variant name) and its runtime value (inner field).
/// This replaces the previous two-enum design where `DataType` was a type tag and `StateValue`
/// carried values — they had a 1:1 mapping, so consolidation eliminates redundancy.
///
/// Type-tag usage (SCPD XML generation): pass an empty/default value.
/// Value usage (state store): pass the actual value.
pub use DataType as StateValue;

impl DataType {
    /// Convert to UPnP string representation for SOAP/LastChange.
    pub fn as_value_str(&self) -> String {
        match self {
            Self::Ui1(v) => v.to_string(),
            Self::Ui2(v) => v.to_string(),
            Self::Ui4(v) => v.to_string(),
            Self::I1(v) => v.to_string(),
            Self::I2(v) => v.to_string(),
            Self::I4(v) => v.to_string(),
            Self::I8(v) => v.to_string(),
            Self::Float(v) => format!("{:.6}", v),
            Self::Double(v) => format!("{:.15}", v),
            Self::Decimal(v) => v.clone(),
            Self::Char(v) => v.to_string(),
            Self::String(v) => v.clone(),
            Self::Date(v) => v.clone(),
            Self::DateTime(v) => v.clone(),
            Self::Boolean(v) => {
                if *v {
                    "1".to_string()
                } else {
                    "0".to_string()
                }
            }
            Self::Base64(v) => v.clone(),
            Self::HexBinary(v) => v.clone(),
            Self::Uri(v) => v.clone(),
            Self::Uuid(v) => v.clone(),
        }
    }
}

impl From<u8> for DataType {
    fn from(v: u8) -> Self {
        Self::Ui1(v)
    }
}
impl From<u16> for DataType {
    fn from(v: u16) -> Self {
        Self::Ui2(v)
    }
}
impl From<u32> for DataType {
    fn from(v: u32) -> Self {
        Self::Ui4(v)
    }
}
impl From<i8> for DataType {
    fn from(v: i8) -> Self {
        Self::I1(v)
    }
}
impl From<i16> for DataType {
    fn from(v: i16) -> Self {
        Self::I2(v)
    }
}
impl From<i32> for DataType {
    fn from(v: i32) -> Self {
        Self::I4(v)
    }
}
impl From<i64> for DataType {
    fn from(v: i64) -> Self {
        Self::I8(v)
    }
}
impl From<f32> for DataType {
    fn from(v: f32) -> Self {
        Self::Float(v)
    }
}
impl From<f64> for DataType {
    fn from(v: f64) -> Self {
        Self::Double(v)
    }
}
impl From<bool> for DataType {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}
impl From<String> for DataType {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}
impl From<&str> for DataType {
    fn from(v: &str) -> Self {
        Self::String(v.to_string())
    }
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_value_str())
    }
}

// Re-export state management types from state module
pub use crate::state::{StateSchema, StateStore, StateVariable, state_def};

// ---------------------------------------------------------------------------
// ActionArgs — typed key-value store for SOAP arguments
// ---------------------------------------------------------------------------

/// Key-value store for SOAP action arguments.
///
/// Uses `String` values for all types — serialization/deserialization happens
/// at the SOAP boundary. The application layer converts to/from Rust types.
#[derive(Debug, Clone, Default)]
pub struct ActionArgs {
    pairs: Vec<(String, String)>,
}

impl ActionArgs {
    pub fn new() -> Self {
        Self { pairs: Vec::new() }
    }

    pub fn from_pairs(pairs: Vec<(String, String)>) -> Self {
        Self { pairs }
    }

    /// Get a value by argument name.
    pub fn get(&self, name: &str) -> Option<&str> {
        self.pairs
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }

    /// Set a key-value pair.
    pub fn set(&mut self, name: String, value: String) {
        if let Some((_, v)) = self.pairs.iter_mut().find(|(k, _)| k == &name) {
            *v = value;
        } else {
            self.pairs.push((name, value));
        }
    }

    /// Get all pairs.
    pub fn pairs(&self) -> &[(String, String)] {
        &self.pairs
    }

    /// Convert to owned pairs.
    pub fn into_pairs(self) -> Vec<(String, String)> {
        self.pairs
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Action trait — runtime execution interface
// ---------------------------------------------------------------------------

/// Trait for UPnP action implementations.
///
/// Each concrete action (Play, SetVolume, GetBrightness, etc.) implements this trait.
/// The action carries its own schema metadata (name, in/out args) so that
/// `ActionMap` and SCPD generation derive everything from the registered action.
/// No manual tracking needed — the action IS the source of truth.
pub trait Action: Send + Sync {
    /// Returns the UPnP action name (e.g. "Play", "SetVolume", "GetTransportInfo").
    fn name(&self) -> &'static str;

    /// Returns the IN argument definitions for this action.
    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>];

    /// Returns the OUT argument definitions for this action.
    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>];

    /// Execute the action with the given arguments.
    ///
    /// Returns OUT arguments on success, or a UPnP error code on failure.
    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error>;
}

// ---------------------------------------------------------------------------
// ActionMap — runtime registry of actions for a service
// ---------------------------------------------------------------------------

/// Runtime registry of actions for a service.
///
/// Maps action names to their implementations. Used by `SoapHandler` to
/// dispatch incoming SOAP requests to the correct action.
pub struct ActionMap {
    namespace: Services,
    actions: Vec<(String, Box<dyn Action>)>,
    action_defs: Vec<ActionDefinition<String, String, String>>,
}

impl ActionMap {
    pub fn new(namespace: Services) -> Self {
        Self {
            namespace,
            actions: Vec::new(),
            action_defs: Vec::new(),
        }
    }

    /// Register an action.
    ///
    /// Schema metadata (name, in/out args) is extracted from the action itself
    /// via the `Action` trait methods. No separate schema parameters needed.
    pub fn register(&mut self, action: Box<dyn Action>) {
        let name = action.name().to_string();
        let in_args: Vec<ArgumentDefinition<String, String>> = action
            .in_args()
            .iter()
            .map(|a| ArgumentDefinition {
                name: a.name.to_string(),
                direction: a.direction.clone(),
                related_state_var: a.related_state_var.map(String::from),
            })
            .collect();
        let out_args: Vec<ArgumentDefinition<String, String>> = action
            .out_args()
            .iter()
            .map(|a| ArgumentDefinition {
                name: a.name.to_string(),
                direction: a.direction.clone(),
                related_state_var: a.related_state_var.map(String::from),
            })
            .collect();
        self.actions.push((name.clone(), action));
        self.action_defs.push(ActionDefinition {
            name,
            in_args,
            out_args,
        });
    }

    /// Get an action by name.
    pub fn get(&self, name: &str) -> Option<&dyn Action> {
        self.actions
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, a)| a.as_ref())
    }

    /// Get all action names (for SCPD generation).
    pub fn action_names(&self) -> Vec<String> {
        self.actions.iter().map(|(n, _)| n.clone()).collect()
    }

    /// Get all action definitions (for SCPD generation).
    pub fn action_definitions(&self) -> &[ActionDefinition<String, String, String>] {
        &self.action_defs
    }

    /// Get the service namespace (full URN string).
    pub fn namespace(&self) -> String {
        self.namespace.full_namespace()
    }

    /// Get the service name from the namespace.
    pub fn service_name(&self) -> &str {
        self.namespace.name()
    }
}
