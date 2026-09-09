// ===========================================================================
// UPnP ConnectionManager Domain Enums
// ===========================================================================

use crate::types::upnp::ServiceVersion;

/// ConnectionManager service version
pub const V3: ServiceVersion = ServiceVersion::V3;

/// Connection status per UPnP-av-ConnectionManager-v3 spec §4.2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionStatus {
    Ok,
    ContentFormatMismatch,
    InsufficientBandwidth,
    UnreliableChannel,
    Unknown,
}

impl ConnectionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::ContentFormatMismatch => "ContentFormatMismatch",
            Self::InsufficientBandwidth => "InsufficientBandwidth",
            Self::UnreliableChannel => "UnreliableChannel",
            Self::Unknown => "Unknown",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "OK" => Some(Self::Ok),
            "ContentFormatMismatch" => Some(Self::ContentFormatMismatch),
            "InsufficientBandwidth" => Some(Self::InsufficientBandwidth),
            "UnreliableChannel" => Some(Self::UnreliableChannel),
            "Unknown" => Some(Self::Unknown),
            _ => None,
        }
    }
}

/// Connection direction per UPnP-av-ConnectionManager-v3 spec §4.2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Input,
    Output,
}

impl Direction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Input => "Input",
            Self::Output => "Output",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Input" => Some(Self::Input),
            "Output" => Some(Self::Output),
            _ => None,
        }
    }
}

// ===========================================================================
// UPnP ConnectionManager Service Actions
// ===========================================================================

/// ConnectionManager service actions (UPnP AV 2.0 spec §4.2.12)
///
/// 7 actions total: 4 Required (R), 3 Optional (O)

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionName {
    // Required actions
    GetProtocolInfo,
    GetCurrentConnectionIDs,
    // Optional actions
    PrepareForConnection,
    ConnectionComplete,
    GetCurrentConnectionInfo,
    GetRendererItemInfo,
    GetFeatureList,
}

/// ConnectionManager argument names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArgumentName {
    Source,
    Sink,
    RemoteProtocolInfo,
    PeerConnectionManager,
    PeerConnectionID,
    Direction,
    ConnectionID,
    AVTransportID,
    RcsID,
    ConnectionIDs,
    ProtocolInfo,
    ItemInfoFilter,
    ItemMetadataList,
    ItemRenderingInfoList,
    FeatureList,
}

impl ActionName {
    /// Returns the UPnP action name string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GetProtocolInfo => "GetProtocolInfo",
            Self::PrepareForConnection => "PrepareForConnection",
            Self::ConnectionComplete => "ConnectionComplete",
            Self::GetCurrentConnectionIDs => "GetCurrentConnectionIDs",
            Self::GetCurrentConnectionInfo => "GetCurrentConnectionInfo",
            Self::GetRendererItemInfo => "GetRendererItemInfo",
            Self::GetFeatureList => "GetFeatureList",
        }
    }

    /// Returns true if this is a required action.
    pub fn is_required(&self) -> bool {
        matches!(
            self,
            Self::GetProtocolInfo
                | Self::GetCurrentConnectionIDs
                | Self::GetCurrentConnectionInfo
                | Self::GetFeatureList
        )
    }

    /// Returns true if this is an optional action.
    pub fn is_optional(&self) -> bool {
        matches!(
            self,
            Self::PrepareForConnection | Self::ConnectionComplete | Self::GetRendererItemInfo
        )
    }
}

impl ArgumentName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Source => "Source",
            Self::Sink => "Sink",
            Self::RemoteProtocolInfo => "RemoteProtocolInfo",
            Self::PeerConnectionManager => "PeerConnectionManager",
            Self::PeerConnectionID => "PeerConnectionID",
            Self::Direction => "Direction",
            Self::ConnectionID => "ConnectionID",
            Self::AVTransportID => "AVTransportID",
            Self::RcsID => "RcsID",
            Self::ConnectionIDs => "ConnectionIDs",
            Self::ProtocolInfo => "ProtocolInfo",
            Self::ItemInfoFilter => "ItemInfoFilter",
            Self::ItemMetadataList => "ItemMetadataList",
            Self::ItemRenderingInfoList => "ItemRenderingInfoList",
            Self::FeatureList => "FeatureList",
        }
    }
}

/// UPnP ConnectionManager service error codes.
///
/// Per UPnP-av-ConnectionManager-v3-Service specification.
/// Error codes 400-699 are defined by the UPnP Device Architecture.
/// Error codes 700-799 are service-specific.
/// Error codes 800-899 are reserved for vendor extensions (not permitted for standard actions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Error {
    // === Protocol/Direction Errors ===
    /// The connection cannot be established because the protocol info argument is incompatible.
    /// Used by: PrepareForConnection()
    IncompatibleProtocolInfo = 701,

    /// The connection cannot be established because the directions of the involved ConnectionManagers (source/sink) are incompatible.
    /// Used by: PrepareForConnection()
    IncompatibleDirections = 702,

    // === Resource Errors ===
    /// The connection cannot be established because there are insufficient network resources (bandwidth, channels, etc.).
    /// Used by: PrepareForConnection()
    InsufficientNetworkResources = 703,

    /// The connection cannot be established because of local restrictions in the device.
    /// This might happen, for example, when physical resources on the device are already in use by other connections.
    /// Used by: PrepareForConnection()
    LocalRestrictions = 704,

    /// The connection cannot be established because the client is not permitted to access the specified ConnectionManager.
    /// Used by: PrepareForConnection()
    AccessDenied = 705,

    // === Connection Reference Errors ===
    /// The connection reference argument does not refer to a valid connection established by this service.
    /// Used by: ConnectionComplete(), GetCurrentConnectionInfo()
    InvalidConnectionReference = 706,

    // === Network Errors ===
    /// The connection cannot be established because the ConnectionManagers are not part of the same physical network.
    /// Used by: PrepareForConnection()
    NotInNetwork = 707,

    // === Capacity Errors ===
    /// The connection cannot be established because the specified ConnectionManager has instantiated the maximum number of simultaneous connections it has room for in its internal data structures.
    /// Closing one connection will resolve the issue.
    /// Used by: PrepareForConnection()
    ConnectionTableOverflow = 708,

    /// The connection cannot be established because the device does not have sufficient internal processing resources to handle the new connection.
    /// Closing one or more connections on this device may resolve the issue.
    /// Used by: PrepareForConnection()
    InternalProcessingResourcesExceeded = 709,

    /// The connection cannot be established because the device does not have sufficient internal memory resources to handle the new connection.
    /// Closing one or more connections on this device may resolve the issue.
    /// Used by: PrepareForConnection()
    InternalMemoryResourcesExceeded = 710,

    /// The connection cannot be established because the device does not have sufficient internal storage system capabilities to handle the new connection.
    /// Closing one or more connections on this device may resolve the issue.
    /// Used by: PrepareForConnection()
    InternalStorageSystemCapabilitiesExceeded = 711,
}

impl Error {
    /// Returns the numeric error code.
    pub const fn code(&self) -> u16 {
        *self as u16
    }

    /// Returns the human-readable error description.
    pub const fn description(&self) -> &'static str {
        match self {
            Self::IncompatibleProtocolInfo => {
                "The connection cannot be established because the protocol info argument is incompatible"
            }
            Self::IncompatibleDirections => {
                "The connection cannot be established because the directions of the involved ConnectionManagers (source/sink) are incompatible"
            }
            Self::InsufficientNetworkResources => {
                "The connection cannot be established because there are insufficient network resources (bandwidth, channels, etc.)"
            }
            Self::LocalRestrictions => {
                "The connection cannot be established because of local restrictions in the device (for example, when physical resources on the device are already in use by other connections)"
            }
            Self::AccessDenied => {
                "The connection cannot be established because the client is not permitted to access the specified ConnectionManager"
            }
            Self::InvalidConnectionReference => {
                "The connection reference argument does not refer to a valid connection established by this service"
            }
            Self::NotInNetwork => {
                "The connection cannot be established because the ConnectionManagers are not part of the same physical network"
            }
            Self::ConnectionTableOverflow => {
                "The connection cannot be established because the specified ConnectionManager has instantiated the maximum number of simultaneous connections it has room for in its internal data structures"
            }
            Self::InternalProcessingResourcesExceeded => {
                "The connection cannot be established because the device does not have sufficient internal processing resources to handle the new connection"
            }
            Self::InternalMemoryResourcesExceeded => {
                "The connection cannot be established because the device does not have sufficient internal memory resources to handle the new connection"
            }
            Self::InternalStorageSystemCapabilitiesExceeded => {
                "The connection cannot be established because the device does not have sufficient internal storage system capabilities to handle the new connection"
            }
        }
    }

    /// Lookup an error by its numeric code.
    pub const fn from_code(code: u16) -> Option<Self> {
        match code {
            701 => Some(Self::IncompatibleProtocolInfo),
            702 => Some(Self::IncompatibleDirections),
            703 => Some(Self::InsufficientNetworkResources),
            704 => Some(Self::LocalRestrictions),
            705 => Some(Self::AccessDenied),
            706 => Some(Self::InvalidConnectionReference),
            707 => Some(Self::NotInNetwork),
            708 => Some(Self::ConnectionTableOverflow),
            709 => Some(Self::InternalProcessingResourcesExceeded),
            710 => Some(Self::InternalMemoryResourcesExceeded),
            711 => Some(Self::InternalStorageSystemCapabilitiesExceeded),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ConnectionManager Error {}: {}",
            self.code(),
            self.description()
        )
    }
}

impl std::error::Error for Error {}

// ===========================================================================
// UPnP ConnectionManager State Variable Names
// ===========================================================================

/// ConnectionManager state variable names per UPnP-av-ConnectionManager-v3 spec §4.2.
///
/// 6 core state variables + 11 A_ARG_TYPE_* pseudo-state-variables.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StateVariableName {
    // Core state variables
    #[default]
    SourceProtocolInfo,
    SinkProtocolInfo,
    CurrentConnectionIDs,
    FeatureList,
    ClockUpdateID,
    DeviceClockInfoUpdates,
    // A_ARG_TYPE variables — type definitions for action arguments
    A_ARG_TYPE_ConnectionStatus,
    A_ARG_TYPE_ConnectionManager,
    A_ARG_TYPE_Direction,
    A_ARG_TYPE_ProtocolInfo,
    A_ARG_TYPE_ConnectionID,
    A_ARG_TYPE_AVTransportID,
    A_ARG_TYPE_RcsID,
    A_ARG_TYPE_ItemInfoFilter,
    A_ARG_TYPE_Result,
    A_ARG_TYPE_RenderingInfoList,
}

impl std::fmt::Display for StateVariableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceProtocolInfo => write!(f, "SourceProtocolInfo"),
            Self::SinkProtocolInfo => write!(f, "SinkProtocolInfo"),
            Self::CurrentConnectionIDs => write!(f, "CurrentConnectionIDs"),
            Self::FeatureList => write!(f, "FeatureList"),
            Self::ClockUpdateID => write!(f, "ClockUpdateID"),
            Self::DeviceClockInfoUpdates => write!(f, "DeviceClockInfoUpdates"),
            Self::A_ARG_TYPE_ConnectionStatus => write!(f, "A_ARG_TYPE_ConnectionStatus"),
            Self::A_ARG_TYPE_ConnectionManager => write!(f, "A_ARG_TYPE_ConnectionManager"),
            Self::A_ARG_TYPE_Direction => write!(f, "A_ARG_TYPE_Direction"),
            Self::A_ARG_TYPE_ProtocolInfo => write!(f, "A_ARG_TYPE_ProtocolInfo"),
            Self::A_ARG_TYPE_ConnectionID => write!(f, "A_ARG_TYPE_ConnectionID"),
            Self::A_ARG_TYPE_AVTransportID => write!(f, "A_ARG_TYPE_AVTransportID"),
            Self::A_ARG_TYPE_RcsID => write!(f, "A_ARG_TYPE_RcsID"),
            Self::A_ARG_TYPE_ItemInfoFilter => write!(f, "A_ARG_TYPE_ItemInfoFilter"),
            Self::A_ARG_TYPE_Result => write!(f, "A_ARG_TYPE_Result"),
            Self::A_ARG_TYPE_RenderingInfoList => write!(f, "A_ARG_TYPE_RenderingInfoList"),
        }
    }
}

impl crate::state::StateVariableName for StateVariableName {
    fn as_str(&self) -> &'static str {
        match self {
            Self::SourceProtocolInfo => "SourceProtocolInfo",
            Self::SinkProtocolInfo => "SinkProtocolInfo",
            Self::CurrentConnectionIDs => "CurrentConnectionIDs",
            Self::FeatureList => "FeatureList",
            Self::ClockUpdateID => "ClockUpdateID",
            Self::DeviceClockInfoUpdates => "DeviceClockInfoUpdates",
            Self::A_ARG_TYPE_ConnectionStatus => "A_ARG_TYPE_ConnectionStatus",
            Self::A_ARG_TYPE_ConnectionManager => "A_ARG_TYPE_ConnectionManager",
            Self::A_ARG_TYPE_Direction => "A_ARG_TYPE_Direction",
            Self::A_ARG_TYPE_ProtocolInfo => "A_ARG_TYPE_ProtocolInfo",
            Self::A_ARG_TYPE_ConnectionID => "A_ARG_TYPE_ConnectionID",
            Self::A_ARG_TYPE_AVTransportID => "A_ARG_TYPE_AVTransportID",
            Self::A_ARG_TYPE_RcsID => "A_ARG_TYPE_RcsID",
            Self::A_ARG_TYPE_ItemInfoFilter => "A_ARG_TYPE_ItemInfoFilter",
            Self::A_ARG_TYPE_Result => "A_ARG_TYPE_Result",
            Self::A_ARG_TYPE_RenderingInfoList => "A_ARG_TYPE_RenderingInfoList",
        }
    }

    fn is_evented(&self) -> bool {
        // SourceProtocolInfo, SinkProtocolInfo, CurrentConnectionIDs, DeviceClockInfoUpdates are evented
        matches!(
            self,
            Self::SourceProtocolInfo
                | Self::SinkProtocolInfo
                | Self::CurrentConnectionIDs
                | Self::DeviceClockInfoUpdates
        )
    }

    fn is_instance_scoped(&self) -> bool {
        // ConnectionManager state variables are NOT instance-scoped (global service state)
        false
    }

    fn data_type(&self) -> crate::types::upnp::DataType {
        match self {
            Self::SourceProtocolInfo => crate::types::upnp::DataType::String,
            Self::SinkProtocolInfo => crate::types::upnp::DataType::String,
            Self::CurrentConnectionIDs => crate::types::upnp::DataType::String,
            Self::FeatureList => crate::types::upnp::DataType::String,
            Self::ClockUpdateID => crate::types::upnp::DataType::UnsignedInt,
            Self::DeviceClockInfoUpdates => crate::types::upnp::DataType::String,
            Self::A_ARG_TYPE_ConnectionStatus => crate::types::upnp::DataType::String,
            Self::A_ARG_TYPE_ConnectionManager => crate::types::upnp::DataType::String,
            Self::A_ARG_TYPE_Direction => crate::types::upnp::DataType::String,
            Self::A_ARG_TYPE_ProtocolInfo => crate::types::upnp::DataType::String,
            Self::A_ARG_TYPE_ConnectionID => crate::types::upnp::DataType::UnsignedInt,
            Self::A_ARG_TYPE_AVTransportID => crate::types::upnp::DataType::UnsignedInt,
            Self::A_ARG_TYPE_RcsID => crate::types::upnp::DataType::UnsignedInt,
            Self::A_ARG_TYPE_ItemInfoFilter => crate::types::upnp::DataType::String,
            Self::A_ARG_TYPE_Result => crate::types::upnp::DataType::String,
            Self::A_ARG_TYPE_RenderingInfoList => crate::types::upnp::DataType::String,
        }
    }
}
