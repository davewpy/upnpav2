// ===========================================================================
// UPnP RenderingControl Domain Enums
// ===========================================================================

use crate::types::upnp::ServiceVersion;

/// RenderingControl service version
pub const V3: ServiceVersion = ServiceVersion::V3;

/// Audio channel per UPnP-av-RenderingControl-v3 spec §3.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Channel {
    Master,
    LF,
    RF,
    CF,
    LFE,
    LS,
    RS,
    LFC,
    RFC,
    SD,
    SL,
    SR,
    T,
    B,
    BC,
    BL,
    BR,
}

impl Channel {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Master => "Master",
            Self::LF => "LF",
            Self::RF => "RF",
            Self::CF => "CF",
            Self::LFE => "LFE",
            Self::LS => "LS",
            Self::RS => "RS",
            Self::LFC => "LFC",
            Self::RFC => "RFC",
            Self::SD => "SD",
            Self::SL => "SL",
            Self::SR => "SR",
            Self::T => "T",
            Self::B => "B",
            Self::BC => "BC",
            Self::BL => "BL",
            Self::BR => "BR",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Master" => Some(Self::Master),
            "LF" => Some(Self::LF),
            "RF" => Some(Self::RF),
            "CF" => Some(Self::CF),
            "LFE" => Some(Self::LFE),
            "LS" => Some(Self::LS),
            "RS" => Some(Self::RS),
            "LFC" => Some(Self::LFC),
            "RFC" => Some(Self::RFC),
            "SD" => Some(Self::SD),
            "SL" => Some(Self::SL),
            "SR" => Some(Self::SR),
            "T" => Some(Self::T),
            "B" => Some(Self::B),
            "BC" => Some(Self::BC),
            "BL" => Some(Self::BL),
            "BR" => Some(Self::BR),
            _ => None,
        }
    }
}

// ===========================================================================
// UPnP RenderingControl Service Actions
// ===========================================================================

// RenderingControl service actions (UPnP AV 2.0 spec §4.2.11)
///
/// 44 actions total: 2 Required (R), 28 Optional (O), 14 Conditionally Required (CR)

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionName {
    // Required actions
    ListPresets,
    SelectPreset,
    // Optional actions
    GetBrightness,
    SetBrightness,
    GetContrast,
    SetContrast,
    GetSharpness,
    SetSharpness,
    GetRedVideoGain,
    SetRedVideoGain,
    GetGreenVideoGain,
    SetGreenVideoGain,
    GetBlueVideoGain,
    SetBlueVideoGain,
    GetRedVideoBlackLevel,
    SetRedVideoBlackLevel,
    GetGreenVideoBlackLevel,
    SetGreenVideoBlackLevel,
    GetBlueVideoBlackLevel,
    SetBlueVideoBlackLevel,
    GetColorTemperature,
    SetColorTemperature,
    GetHorizontalKeystone,
    SetHorizontalKeystone,
    GetVerticalKeystone,
    SetVerticalKeystone,
    GetMute,
    SetMute,
    GetVolume,
    SetVolume,
    GetVolumeDB,
    SetVolumeDB,
    GetLoudness,
    SetLoudness,
    GetStateVariables,
    SetStateVariables,
    // Conditionally required actions
    GetVolumeDBRange,
    GetAllowedTransforms,
    GetTransforms,
    SetTransforms,
    GetAllowedDefaultTransforms,
    GetDefaultTransforms,
    SetDefaultTransforms,
    GetAllAvailableTransforms,
}

/// RenderingControl argument names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArgumentName {
    InstanceID,
    Channel,
    DesiredVolume,
    PresetName,
    CurrentPresetNameList,
    CurrentBrightness,
    DesiredBrightness,
    CurrentContrast,
    DesiredContrast,
    CurrentSharpness,
    DesiredSharpness,
    CurrentRedVideoGain,
    DesiredRedVideoGain,
    CurrentGreenVideoGain,
    DesiredGreenVideoGain,
    CurrentBlueVideoGain,
    DesiredBlueVideoGain,
    CurrentRedVideoBlackLevel,
    DesiredRedVideoBlackLevel,
    CurrentGreenVideoBlackLevel,
    DesiredGreenVideoBlackLevel,
    CurrentBlueVideoBlackLevel,
    DesiredBlueVideoBlackLevel,
    CurrentColorTemperature,
    DesiredColorTemperature,
    CurrentHorizontalKeystone,
    DesiredHorizontalKeystone,
    CurrentVerticalKeystone,
    DesiredVerticalKeystone,
    CurrentMute,
    DesiredMute,
    CurrentVolume,
    CurrentVolumeDB,
    DesiredVolumeDB,
    CurrentLoudness,
    DesiredLoudness,
    CurrentVolumeDBRange,
    CurrentAllowedTransforms,
    DesiredTransforms,
    CurrentAllowedDefaultTransforms,
    CurrentDefaultTransforms,
    AllAvailableTransforms,
}

/// RenderingControl state variable names.
///
/// 21 standard state variables + 8 A_ARG_TYPE_* pseudo-state-variables.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StateVariableName {
    #[default]
    LastChange,
    PresetNameList,
    Brightness,
    Contrast,
    Sharpness,
    RedVideoGain,
    GreenVideoGain,
    BlueVideoGain,
    RedVideoBlackLevel,
    GreenVideoBlackLevel,
    BlueVideoBlackLevel,
    ColorTemperature,
    HorizontalKeystone,
    VerticalKeystone,
    Mute,
    Volume,
    VolumeDB,
    Loudness,
    AllowedTransformSettings,
    TransformSettings,
    AllowedDefaultTransformSettings,
    DefaultTransformSettings,
    // A_ARG_TYPE variables — type definitions for action arguments
    A_ARG_TYPE_InstanceID,
    A_ARG_TYPE_Channel,
    A_ARG_TYPE_PresetName,
    A_ARG_TYPE_DeviceUDN,
    A_ARG_TYPE_ServiceType,
    A_ARG_TYPE_ServiceID,
    A_ARG_TYPE_StateVariableValuePairs,
    A_ARG_TYPE_StateVariableList,
}

impl std::fmt::Display for StateVariableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateVariableName::LastChange => write!(f, "LastChange"),
            StateVariableName::PresetNameList => write!(f, "PresetNameList"),
            StateVariableName::Brightness => write!(f, "Brightness"),
            StateVariableName::Contrast => write!(f, "Contrast"),
            StateVariableName::Sharpness => write!(f, "Sharpness"),
            StateVariableName::RedVideoGain => write!(f, "RedVideoGain"),
            StateVariableName::GreenVideoGain => write!(f, "GreenVideoGain"),
            StateVariableName::BlueVideoGain => write!(f, "BlueVideoGain"),
            StateVariableName::RedVideoBlackLevel => write!(f, "RedVideoBlackLevel"),
            StateVariableName::GreenVideoBlackLevel => write!(f, "GreenVideoBlackLevel"),
            StateVariableName::BlueVideoBlackLevel => write!(f, "BlueVideoBlackLevel"),
            StateVariableName::ColorTemperature => write!(f, "ColorTemperature"),
            StateVariableName::HorizontalKeystone => write!(f, "HorizontalKeystone"),
            StateVariableName::VerticalKeystone => write!(f, "VerticalKeystone"),
            StateVariableName::Mute => write!(f, "Mute"),
            StateVariableName::Volume => write!(f, "Volume"),
            StateVariableName::VolumeDB => write!(f, "VolumeDB"),
            StateVariableName::Loudness => write!(f, "Loudness"),
            StateVariableName::AllowedTransformSettings => write!(f, "AllowedTransformSettings"),
            StateVariableName::TransformSettings => write!(f, "TransformSettings"),
            StateVariableName::AllowedDefaultTransformSettings => {
                write!(f, "AllowedDefaultTransformSettings")
            }
            StateVariableName::DefaultTransformSettings => write!(f, "DefaultTransformSettings"),
            StateVariableName::A_ARG_TYPE_InstanceID => write!(f, "A_ARG_TYPE_InstanceID"),
            StateVariableName::A_ARG_TYPE_Channel => write!(f, "A_ARG_TYPE_Channel"),
            StateVariableName::A_ARG_TYPE_PresetName => write!(f, "A_ARG_TYPE_PresetName"),
            StateVariableName::A_ARG_TYPE_DeviceUDN => write!(f, "A_ARG_TYPE_DeviceUDN"),
            StateVariableName::A_ARG_TYPE_ServiceType => write!(f, "A_ARG_TYPE_ServiceType"),
            StateVariableName::A_ARG_TYPE_ServiceID => write!(f, "A_ARG_TYPE_ServiceID"),
            StateVariableName::A_ARG_TYPE_StateVariableValuePairs => {
                write!(f, "A_ARG_TYPE_StateVariableValuePairs")
            }
            StateVariableName::A_ARG_TYPE_StateVariableList => {
                write!(f, "A_ARG_TYPE_StateVariableList")
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Static schema definitions — declarative metadata from UPnP spec tables
// ---------------------------------------------------------------------------
use crate::types::statevariable::StateVariableSchema;

/// All RenderingControl state variable schemas.
///
/// This is the single source of truth for eventing, instance scoping,
/// data types, and defaults — directly from the UPnP spec §4.1 tables.
pub static STATE_VARIABLE_SCHEMAS: &[StateVariableSchema] = &[
    // Eventing
    StateVariableSchema {
        name: "LastChange",
        is_evented: true,
        via_lastchange: false,
        is_instance_scoped: false,
        data_type_name: "string",
        default: Some(""),
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    // Non-instance-scoped
    StateVariableSchema {
        name: "PresetNameList",
        is_evented: false,
        via_lastchange: true,
        is_instance_scoped: false,
        data_type_name: "string",
        default: Some(""),
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    // Instance-scoped audio/video state variables
    StateVariableSchema {
        name: "Brightness",
        is_evented: false,
        via_lastchange: true,
        is_instance_scoped: true,
        data_type_name: "ui2",
        default: Some("0"),
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    StateVariableSchema {
        name: "Contrast",
        is_evented: false,
        via_lastchange: true,
        is_instance_scoped: true,
        data_type_name: "ui2",
        default: Some("0"),
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    StateVariableSchema {
        name: "Sharpness",
        is_evented: false,
        via_lastchange: true,
        is_instance_scoped: true,
        data_type_name: "ui2",
        default: Some("0"),
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    StateVariableSchema {
        name: "RedVideoGain",
        is_evented: false,
        via_lastchange: true,
        is_instance_scoped: true,
        data_type_name: "ui2",
        default: Some("0"),
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    StateVariableSchema {
        name: "GreenVideoGain",
        is_evented: false,
        via_lastchange: true,
        is_instance_scoped: true,
        data_type_name: "ui2",
        default: Some("0"),
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    StateVariableSchema {
        name: "BlueVideoGain",
        is_evented: false,
        via_lastchange: true,
        is_instance_scoped: true,
        data_type_name: "ui2",
        default: Some("0"),
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    StateVariableSchema {
        name: "RedVideoBlackLevel",
        is_evented: false,
        via_lastchange: true,
        is_instance_scoped: true,
        data_type_name: "ui2",
        default: Some("0"),
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    StateVariableSchema {
        name: "GreenVideoBlackLevel",
        is_evented: false,
        via_lastchange: true,
        is_instance_scoped: true,
        data_type_name: "ui2",
        default: Some("0"),
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    StateVariableSchema {
        name: "BlueVideoBlackLevel",
        is_evented: false,
        via_lastchange: true,
        is_instance_scoped: true,
        data_type_name: "ui2",
        default: Some("0"),
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    StateVariableSchema {
        name: "ColorTemperature",
        is_evented: false,
        via_lastchange: true,
        is_instance_scoped: true,
        data_type_name: "ui2",
        default: Some("0"),
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    StateVariableSchema {
        name: "HorizontalKeystone",
        is_evented: false,
        via_lastchange: true,
        is_instance_scoped: true,
        data_type_name: "i2",
        default: Some("0"),
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    StateVariableSchema {
        name: "VerticalKeystone",
        is_evented: false,
        via_lastchange: true,
        is_instance_scoped: true,
        data_type_name: "i2",
        default: Some("0"),
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    StateVariableSchema {
        name: "Mute",
        is_evented: false,
        via_lastchange: true,
        is_instance_scoped: true,
        data_type_name: "boolean",
        default: Some("0"),
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    StateVariableSchema {
        name: "Volume",
        is_evented: false,
        via_lastchange: true,
        is_instance_scoped: true,
        data_type_name: "ui2",
        default: Some("0"),
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    StateVariableSchema {
        name: "VolumeDB",
        is_evented: false,
        via_lastchange: true,
        is_instance_scoped: true,
        data_type_name: "i2",
        default: Some("0"),
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    StateVariableSchema {
        name: "Loudness",
        is_evented: false,
        via_lastchange: true,
        is_instance_scoped: true,
        data_type_name: "boolean",
        default: Some("0"),
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    StateVariableSchema {
        name: "AllowedTransformSettings",
        is_evented: false,
        via_lastchange: true,
        is_instance_scoped: true,
        data_type_name: "string",
        default: None,
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    StateVariableSchema {
        name: "TransformSettings",
        is_evented: false,
        via_lastchange: true,
        is_instance_scoped: true,
        data_type_name: "string",
        default: None,
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    // Direct NOTIFY variables
    StateVariableSchema {
        name: "AllowedDefaultTransformSettings",
        is_evented: true,
        via_lastchange: false,
        is_instance_scoped: true,
        data_type_name: "string",
        default: None,
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    StateVariableSchema {
        name: "DefaultTransformSettings",
        is_evented: true,
        via_lastchange: false,
        is_instance_scoped: true,
        data_type_name: "string",
        default: None,
        allowed_values: None,
        allowed_value_range: None,
        argument_type: false,
    },
    // A_ARG_TYPE variables — type definitions (not real state vars)
    StateVariableSchema {
        name: "A_ARG_TYPE_InstanceID",
        is_evented: false,
        via_lastchange: false,
        is_instance_scoped: false,
        data_type_name: "ui4",
        default: Some("0"),
        allowed_values: None,
        allowed_value_range: None,
        argument_type: true,
    },
    StateVariableSchema {
        name: "A_ARG_TYPE_Channel",
        is_evented: false,
        via_lastchange: false,
        is_instance_scoped: false,
        data_type_name: "string",
        default: None,
        allowed_values: None,
        allowed_value_range: None,
        argument_type: true,
    },
    StateVariableSchema {
        name: "A_ARG_TYPE_PresetName",
        is_evented: false,
        via_lastchange: false,
        is_instance_scoped: false,
        data_type_name: "string",
        default: None,
        allowed_values: None,
        allowed_value_range: None,
        argument_type: true,
    },
    StateVariableSchema {
        name: "A_ARG_TYPE_DeviceUDN",
        is_evented: false,
        via_lastchange: false,
        is_instance_scoped: false,
        data_type_name: "string",
        default: None,
        allowed_values: None,
        allowed_value_range: None,
        argument_type: true,
    },
    StateVariableSchema {
        name: "A_ARG_TYPE_ServiceType",
        is_evented: false,
        via_lastchange: false,
        is_instance_scoped: false,
        data_type_name: "string",
        default: None,
        allowed_values: None,
        allowed_value_range: None,
        argument_type: true,
    },
    StateVariableSchema {
        name: "A_ARG_TYPE_ServiceID",
        is_evented: false,
        via_lastchange: false,
        is_instance_scoped: false,
        data_type_name: "string",
        default: None,
        allowed_values: None,
        allowed_value_range: None,
        argument_type: true,
    },
    StateVariableSchema {
        name: "A_ARG_TYPE_StateVariableValuePairs",
        is_evented: false,
        via_lastchange: false,
        is_instance_scoped: false,
        data_type_name: "string",
        default: None,
        allowed_values: None,
        allowed_value_range: None,
        argument_type: true,
    },
    StateVariableSchema {
        name: "A_ARG_TYPE_StateVariableList",
        is_evented: false,
        via_lastchange: false,
        is_instance_scoped: false,
        data_type_name: "string",
        default: None,
        allowed_values: None,
        allowed_value_range: None,
        argument_type: true,
    },
];

impl ActionName {
    /// Returns the UPnP action name string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ListPresets => "ListPresets",
            Self::SelectPreset => "SelectPreset",
            Self::GetBrightness => "GetBrightness",
            Self::SetBrightness => "SetBrightness",
            Self::GetContrast => "GetContrast",
            Self::SetContrast => "SetContrast",
            Self::GetSharpness => "GetSharpness",
            Self::SetSharpness => "SetSharpness",
            Self::GetRedVideoGain => "GetRedVideoGain",
            Self::SetRedVideoGain => "SetRedVideoGain",
            Self::GetGreenVideoGain => "GetGreenVideoGain",
            Self::SetGreenVideoGain => "SetGreenVideoGain",
            Self::GetBlueVideoGain => "GetBlueVideoGain",
            Self::SetBlueVideoGain => "SetBlueVideoGain",
            Self::GetRedVideoBlackLevel => "GetRedVideoBlackLevel",
            Self::SetRedVideoBlackLevel => "SetRedVideoBlackLevel",
            Self::GetGreenVideoBlackLevel => "GetGreenVideoBlackLevel",
            Self::SetGreenVideoBlackLevel => "SetGreenVideoBlackLevel",
            Self::GetBlueVideoBlackLevel => "GetBlueVideoBlackLevel",
            Self::SetBlueVideoBlackLevel => "SetBlueVideoBlackLevel",
            Self::GetColorTemperature => "GetColorTemperature",
            Self::SetColorTemperature => "SetColorTemperature",
            Self::GetHorizontalKeystone => "GetHorizontalKeystone",
            Self::SetHorizontalKeystone => "SetHorizontalKeystone",
            Self::GetVerticalKeystone => "GetVerticalKeystone",
            Self::SetVerticalKeystone => "SetVerticalKeystone",
            Self::GetMute => "GetMute",
            Self::SetMute => "SetMute",
            Self::GetVolume => "GetVolume",
            Self::SetVolume => "SetVolume",
            Self::GetVolumeDB => "GetVolumeDB",
            Self::SetVolumeDB => "SetVolumeDB",
            Self::GetVolumeDBRange => "GetVolumeDBRange",
            Self::GetLoudness => "GetLoudness",
            Self::SetLoudness => "SetLoudness",
            Self::GetStateVariables => "GetStateVariables",
            Self::SetStateVariables => "SetStateVariables",
            Self::GetAllowedTransforms => "GetAllowedTransforms",
            Self::GetTransforms => "GetTransforms",
            Self::SetTransforms => "SetTransforms",
            Self::GetAllowedDefaultTransforms => "GetAllowedDefaultTransforms",
            Self::GetDefaultTransforms => "GetDefaultTransforms",
            Self::SetDefaultTransforms => "SetDefaultTransforms",
            Self::GetAllAvailableTransforms => "GetAllAvailableTransforms",
        }
    }

    /// Returns true if this is a required action.
    pub fn is_required(&self) -> bool {
        matches!(self, Self::ListPresets | Self::SelectPreset)
    }

    /// Returns true if this is an optional action.
    pub fn is_optional(&self) -> bool {
        matches!(
            self,
            Self::GetBrightness
                | Self::SetBrightness
                | Self::GetContrast
                | Self::SetContrast
                | Self::GetSharpness
                | Self::SetSharpness
                | Self::GetRedVideoGain
                | Self::SetRedVideoGain
                | Self::GetGreenVideoGain
                | Self::SetGreenVideoGain
                | Self::GetBlueVideoGain
                | Self::SetBlueVideoGain
                | Self::GetRedVideoBlackLevel
                | Self::SetRedVideoBlackLevel
                | Self::GetGreenVideoBlackLevel
                | Self::SetGreenVideoBlackLevel
                | Self::GetBlueVideoBlackLevel
                | Self::SetBlueVideoBlackLevel
                | Self::GetColorTemperature
                | Self::SetColorTemperature
                | Self::GetHorizontalKeystone
                | Self::SetHorizontalKeystone
                | Self::GetVerticalKeystone
                | Self::SetVerticalKeystone
                | Self::GetMute
                | Self::SetMute
                | Self::GetVolume
                | Self::SetVolume
                | Self::GetVolumeDB
                | Self::SetVolumeDB
                | Self::GetLoudness
                | Self::SetLoudness
                | Self::GetStateVariables
                | Self::SetStateVariables
        )
    }

    /// Returns true if this is a conditionally required action.
    pub fn is_conditionally_required(&self) -> bool {
        matches!(
            self,
            Self::GetVolumeDBRange
                | Self::GetAllowedTransforms
                | Self::GetTransforms
                | Self::SetTransforms
                | Self::GetAllowedDefaultTransforms
                | Self::GetDefaultTransforms
                | Self::SetDefaultTransforms
                | Self::GetAllAvailableTransforms
        )
    }
}

impl ArgumentName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InstanceID => "InstanceID",
            Self::Channel => "Channel",
            Self::DesiredVolume => "DesiredVolume",
            Self::PresetName => "PresetName",
            Self::CurrentPresetNameList => "CurrentPresetNameList",
            Self::CurrentBrightness => "CurrentBrightness",
            Self::DesiredBrightness => "DesiredBrightness",
            Self::CurrentContrast => "CurrentContrast",
            Self::DesiredContrast => "DesiredContrast",
            Self::CurrentSharpness => "CurrentSharpness",
            Self::DesiredSharpness => "DesiredSharpness",
            Self::CurrentRedVideoGain => "CurrentRedVideoGain",
            Self::DesiredRedVideoGain => "DesiredRedVideoGain",
            Self::CurrentGreenVideoGain => "CurrentGreenVideoGain",
            Self::DesiredGreenVideoGain => "DesiredGreenVideoGain",
            Self::CurrentBlueVideoGain => "CurrentBlueVideoGain",
            Self::DesiredBlueVideoGain => "DesiredBlueVideoGain",
            Self::CurrentRedVideoBlackLevel => "CurrentRedVideoBlackLevel",
            Self::DesiredRedVideoBlackLevel => "DesiredRedVideoBlackLevel",
            Self::CurrentGreenVideoBlackLevel => "CurrentGreenVideoBlackLevel",
            Self::DesiredGreenVideoBlackLevel => "DesiredGreenVideoBlackLevel",
            Self::CurrentBlueVideoBlackLevel => "CurrentBlueVideoBlackLevel",
            Self::DesiredBlueVideoBlackLevel => "DesiredBlueVideoBlackLevel",
            Self::CurrentColorTemperature => "CurrentColorTemperature",
            Self::DesiredColorTemperature => "DesiredColorTemperature",
            Self::CurrentHorizontalKeystone => "CurrentHorizontalKeystone",
            Self::DesiredHorizontalKeystone => "DesiredHorizontalKeystone",
            Self::CurrentVerticalKeystone => "CurrentVerticalKeystone",
            Self::DesiredVerticalKeystone => "DesiredVerticalKeystone",
            Self::CurrentMute => "CurrentMute",
            Self::DesiredMute => "DesiredMute",
            Self::CurrentVolume => "CurrentVolume",
            Self::CurrentVolumeDB => "CurrentVolumeDB",
            Self::DesiredVolumeDB => "DesiredVolumeDB",
            Self::CurrentLoudness => "CurrentLoudness",
            Self::DesiredLoudness => "DesiredLoudness",
            Self::CurrentVolumeDBRange => "CurrentVolumeDBRange",
            Self::CurrentAllowedTransforms => "CurrentAllowedTransforms",
            Self::DesiredTransforms => "DesiredTransforms",
            Self::CurrentAllowedDefaultTransforms => "CurrentAllowedDefaultTransforms",
            Self::CurrentDefaultTransforms => "CurrentDefaultTransforms",
            Self::AllAvailableTransforms => "AllAvailableTransforms",
        }
    }
}

/// UPnP RenderingControl service error codes.
///
/// Per UPnP-av-RenderingControl-v3-Service specification.
/// Error codes 400-699 are defined by the UPnP Device Architecture.
/// Error codes 700-799 are service-specific.
/// Error codes 800-899 are reserved for vendor extensions (not permitted for standard actions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Error {
    /// The specified name is not a valid preset name.
    /// Used by: SelectPreset()
    InvalidName = 701,
    /// The specified InstanceID is invalid.
    /// Used by: All RenderingControl actions
    InvalidInstanceID = 702,
    /// The specified Channel is invalid.
    /// Used by: GetMute(), SetMute(), GetVolume(), SetVolume(), GetVolumeDB(), SetVolumeDB(), GetVolumeDBRange()
    InvalidChannel = 703,
    /// Some of the variables are invalid.
    /// Used by: GetStateVariables()
    InvalidStateVariableList = 704,
    /// The CSV list is not well formed.
    /// Used by: SetStateVariables()
    IllFormedCsvList = 705,
    /// One of the StateVariableValuePairs contains an invalid value.
    /// Used by: SetStateVariables()
    InvalidStateVariableValue = 706,
    /// The specified MediaRenderer's UDN is different from the UDN value of the MediaRenderer.
    /// Used by: GetStateVariables(), SetStateVariables()
    InvalidMediaRendererUdn = 707,
    /// The specified ServiceType is invalid.
    /// Used by: GetStateVariables(), SetStateVariables()
    InvalidServiceType = 708,
    /// The specified ServiceId is invalid.
    /// Used by: GetStateVariables(), SetStateVariables()
    InvalidServiceId = 709,
    /// StateVariableValuePairs includes variables that are not allowed to be set (e.g., LastChange and/or PresetNameList must not be included).
    /// Used by: SetStateVariables()
    StateVariablesSpecifiedImproperly = 710,
    /// The specified transforms are not in the allowed list of transforms.
    /// Used by: SetStateVariables()
    TransformsNotAllowed = 711,
    /// The specified transforms cannot be applied because the input value for each specified transform is not supported.
    /// Used by: SetStateVariables()
    UnsupportedValuesForTransforms = 712,
    /// The specified transform cannot be applied because an internal error occurred in the rendering device.
    /// Used by: SetStateVariables()
    InternalError = 713,
}

impl Error {
    /// Returns the numeric error code.
    pub const fn code(&self) -> u16 {
        *self as u16
    }

    /// Returns the human-readable error description.
    pub const fn description(&self) -> &'static str {
        match self {
            Self::InvalidName => "The specified name is not a valid preset name",
            Self::InvalidInstanceID => "The specified InstanceID is invalid",
            Self::InvalidChannel => "The specified Channel is invalid",
            Self::InvalidStateVariableList => "Some of the variables are invalid",
            Self::IllFormedCsvList => "The CSV list is not well formed",
            Self::InvalidStateVariableValue => {
                "One of the StateVariableValuePairs contains an invalid value"
            }
            Self::InvalidMediaRendererUdn => {
                "The specified MediaRenderer's UDN is different from the UDN value of the MediaRenderer"
            }
            Self::InvalidServiceType => "The specified ServiceType is invalid",
            Self::InvalidServiceId => "The specified ServiceId is invalid",
            Self::StateVariablesSpecifiedImproperly => {
                "StateVariableValuePairs includes variables that are not allowed to be set (for example, LastChange and/or PresetNameList must not be included)"
            }
            Self::TransformsNotAllowed => {
                "The specified transforms are not in the allowed list of transforms"
            }
            Self::UnsupportedValuesForTransforms => {
                "The specified transforms cannot be applied because the input value for each specified transforms is not supported"
            }
            Self::InternalError => {
                "The specified transform cannot be applied because an internal error occurred in the rendering device"
            }
        }
    }

    /// Lookup an error by its numeric code.
    pub const fn from_code(code: u16) -> Option<Self> {
        match code {
            701 => Some(Self::InvalidName),
            702 => Some(Self::InvalidInstanceID),
            703 => Some(Self::InvalidChannel),
            704 => Some(Self::InvalidStateVariableList),
            705 => Some(Self::IllFormedCsvList),
            706 => Some(Self::InvalidStateVariableValue),
            707 => Some(Self::InvalidMediaRendererUdn),
            708 => Some(Self::InvalidServiceType),
            709 => Some(Self::InvalidServiceId),
            710 => Some(Self::StateVariablesSpecifiedImproperly),
            711 => Some(Self::TransformsNotAllowed),
            712 => Some(Self::UnsupportedValuesForTransforms),
            713 => Some(Self::InternalError),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RenderingControl Error {}: {}",
            self.code(),
            self.description()
        )
    }
}

impl std::error::Error for Error {}
