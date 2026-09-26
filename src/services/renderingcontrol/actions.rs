/// RenderingControl service — bridge from SOAP `Action` trait to application traits.
///
/// Each struct wraps an Arc<T> where T implements the application trait.
/// The `execute` method extracts typed args, calls the trait, and wraps results.
use std::sync::Arc;

use super::r#static::Channel;
use super::{RenderingControlService, r#static::StateVariableName as RCStateVarName};
use crate::services::renderingcontrol::traits::*;
use crate::types::upnp::{
    Action, ActionArgs, Argument, ArgumentDirection, Error, StateValue,
};

// ===========================================================================
// Required Actions (R)
// ===========================================================================

pub struct ActionListPresets<T: ListPresets> {
    trait_impl: Arc<T>,
}

impl<T: ListPresets> ActionListPresets<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: ListPresets> Action for ActionListPresets<T> {
    fn name(&self) -> &'static str {
        "ListPresets"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "CurrentPresetNameList",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("PresetNameList"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let output = self
            .trait_impl
            .list_presets(instance_id)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set(
            "CurrentPresetNameList".to_string(),
            output.current_preset_name_list,
        );
        Ok(out)
    }
}

pub struct ActionSelectPreset<T: SelectPreset> {
    trait_impl: Arc<T>,
}

impl<T: SelectPreset> ActionSelectPreset<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: SelectPreset> Action for ActionSelectPreset<T> {
    fn name(&self) -> &'static str {
        "SelectPreset"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "PresetName",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PresetName"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let preset_name = args
            .get("PresetName")
            .ok_or(Error::ArgumentValueInvalid)?
            .to_string();
        let input = SelectPresetInput {
            instance_id,
            preset_name,
        };
        self.trait_impl
            .select_preset(input)
            .map_err(|_| Error::ActionFailed)?;
        Ok(ActionArgs::new())
    }
}

// ===========================================================================
// Optional Actions — Brightness / Contrast / Sharpness
// ===========================================================================

pub struct ActionGetBrightness<T: GetBrightness> {
    trait_impl: Arc<T>,
}

impl<T: GetBrightness> ActionGetBrightness<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetBrightness> Action for ActionGetBrightness<T> {
    fn name(&self) -> &'static str {
        "GetBrightness"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "CurrentBrightness",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("Brightness"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let input = GetBrightnessInput { instance_id };
        let output = self
            .trait_impl
            .get_brightness(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set(
            "CurrentBrightness".to_string(),
            output.current_brightness.to_string(),
        );
        Ok(out)
    }
}

pub struct ActionSetBrightness<T: SetBrightness> {
    trait_impl: Arc<T>,
    service: RenderingControlService,
}

impl<T: SetBrightness> ActionSetBrightness<T> {
    pub fn new(trait_impl: T, service: RenderingControlService) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            service,
        }
    }
}

impl<T: SetBrightness> Action for ActionSetBrightness<T> {
    fn name(&self) -> &'static str {
        "SetBrightness"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "DesiredBrightness",
                direction: ArgumentDirection::IN,
                related_state_var: Some("Brightness"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let desired_brightness = args
            .get("DesiredBrightness")
            .ok_or(Error::ArgumentValueInvalid)?
            .parse::<u16>()
            .map_err(|_| Error::ArgumentValueOutOfRange)?;
        let input = SetBrightnessInput {
            instance_id,
            desired_brightness,
        };
        self.trait_impl
            .set_brightness(input)
            .map_err(|_| Error::ActionFailed)?;
        // Update state variable and trigger LastChange event
        let desired = args
            .get("DesiredBrightness")
            .unwrap_or("0")
            .parse::<u16>()
            .unwrap_or(0);
        self.service.set_state_var(
            instance_id,
            RCStateVarName::Brightness,
            StateValue::Ui2(desired),
        );
        Ok(ActionArgs::new())
    }
}

pub struct ActionGetContrast<T: GetContrast> {
    trait_impl: Arc<T>,
}

impl<T: GetContrast> ActionGetContrast<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetContrast> Action for ActionGetContrast<T> {
    fn name(&self) -> &'static str {
        "GetContrast"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "CurrentContrast",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("Contrast"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let input = GetContrastInput { instance_id };
        let output = self
            .trait_impl
            .get_contrast(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set(
            "CurrentContrast".to_string(),
            output.current_contrast.to_string(),
        );
        Ok(out)
    }
}

pub struct ActionSetContrast<T: SetContrast> {
    trait_impl: Arc<T>,
    service: RenderingControlService,
}

impl<T: SetContrast> ActionSetContrast<T> {
    pub fn new(trait_impl: T, service: RenderingControlService) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            service,
        }
    }
}

impl<T: SetContrast> Action for ActionSetContrast<T> {
    fn name(&self) -> &'static str {
        "SetContrast"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "DesiredContrast",
                direction: ArgumentDirection::IN,
                related_state_var: Some("Contrast"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let desired_contrast = args
            .get("DesiredContrast")
            .ok_or(Error::ArgumentValueInvalid)?
            .parse::<u16>()
            .map_err(|_| Error::ArgumentValueOutOfRange)?;
        let input = SetContrastInput {
            instance_id,
            desired_contrast,
        };
        self.trait_impl
            .set_contrast(input)
            .map_err(|_| Error::ActionFailed)?;
        let desired = args
            .get("DesiredContrast")
            .unwrap_or("0")
            .parse::<u16>()
            .unwrap_or(0);
        self.service.set_state_var(
            instance_id,
            RCStateVarName::Contrast,
            StateValue::Ui2(desired),
        );
        Ok(ActionArgs::new())
    }
}

pub struct ActionGetSharpness<T: GetSharpness> {
    trait_impl: Arc<T>,
}

impl<T: GetSharpness> ActionGetSharpness<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetSharpness> Action for ActionGetSharpness<T> {
    fn name(&self) -> &'static str {
        "GetSharpness"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "CurrentSharpness",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("Sharpness"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let input = GetSharpnessInput { instance_id };
        let output = self
            .trait_impl
            .get_sharpness(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set(
            "CurrentSharpness".to_string(),
            output.current_sharpness.to_string(),
        );
        Ok(out)
    }
}

pub struct ActionSetSharpness<T: SetSharpness> {
    trait_impl: Arc<T>,
    service: RenderingControlService,
}

impl<T: SetSharpness> ActionSetSharpness<T> {
    pub fn new(trait_impl: T, service: RenderingControlService) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            service,
        }
    }
}

impl<T: SetSharpness> Action for ActionSetSharpness<T> {
    fn name(&self) -> &'static str {
        "SetSharpness"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "DesiredSharpness",
                direction: ArgumentDirection::IN,
                related_state_var: Some("Sharpness"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let desired_sharpness = args
            .get("DesiredSharpness")
            .ok_or(Error::ArgumentValueInvalid)?
            .parse::<u16>()
            .map_err(|_| Error::ArgumentValueOutOfRange)?;
        let input = SetSharpnessInput {
            instance_id,
            desired_sharpness,
        };
        self.trait_impl
            .set_sharpness(input)
            .map_err(|_| Error::ActionFailed)?;
        let desired = args
            .get("DesiredSharpness")
            .unwrap_or("0")
            .parse::<u16>()
            .unwrap_or(0);
        self.service.set_state_var(
            instance_id,
            RCStateVarName::Sharpness,
            StateValue::Ui2(desired),
        );
        Ok(ActionArgs::new())
    }
}

// ===========================================================================
// Optional Actions — Video Color Controls
// ===========================================================================

macro_rules! impl_video_color {
    ($prefixed_get:ident, $get_trait:ident, $get_input:ident, $output_name:ident, $method_get:ident, $prefixed_set:ident, $set_trait:ident, $set_input:ident, $method_set:ident, $arg_name:expr, $soap_name:expr, $field_name:ident, $desired_field:ident, $state_var:expr, $state_var_enum:ident) => {
        pub struct $prefixed_get<T: $get_trait> {
            trait_impl: Arc<T>,
        }

        impl<T: $get_trait> $prefixed_get<T> {
            pub fn new(trait_impl: T) -> Self {
                Self {
                    trait_impl: Arc::new(trait_impl),
                }
            }
        }

        impl<T: $get_trait> Action for $prefixed_get<T> {
            fn name(&self) -> &'static str {
                concat!("Get", $arg_name)
            }

            fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
                static ARGS: &[Argument<&'static str, &'static str>] =
                    &[Argument {
                        name: "InstanceID",
                        direction: ArgumentDirection::IN,
                        related_state_var: Some("A_ARG_TYPE_InstanceID"),
                    }];
                &ARGS
            }

            fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
                static ARGS: &[Argument<&'static str, &'static str>] =
                    &[Argument {
                        name: $soap_name,
                        direction: ArgumentDirection::OUT,
                        related_state_var: Some($state_var),
                    }];
                &ARGS
            }

            fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
                let instance_id = args
                    .get("InstanceID")
                    .unwrap_or("0")
                    .parse::<u32>()
                    .unwrap_or(0);
                let input = $get_input { instance_id };
                let output = self
                    .trait_impl
                    .$method_get(input)
                    .map_err(|_| Error::ActionFailed)?;
                let mut out = ActionArgs::new();
                out.set($soap_name.to_string(), output.$field_name.to_string());
                Ok(out)
            }
        }

        pub struct $prefixed_set<T: $set_trait> {
            trait_impl: Arc<T>,
            service: RenderingControlService,
        }

        impl<T: $set_trait> $prefixed_set<T> {
            pub fn new(trait_impl: T, service: RenderingControlService) -> Self {
                Self {
                    trait_impl: Arc::new(trait_impl),
                    service,
                }
            }
        }

        impl<T: $set_trait> Action for $prefixed_set<T> {
            fn name(&self) -> &'static str {
                concat!("Set", $arg_name)
            }

            fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
                static ARGS: &[Argument<&'static str, &'static str>] = &[
                    Argument {
                        name: "InstanceID",
                        direction: ArgumentDirection::IN,
                        related_state_var: Some("A_ARG_TYPE_InstanceID"),
                    },
                    Argument {
                        name: concat!("Desired", $arg_name),
                        direction: ArgumentDirection::IN,
                        related_state_var: Some($state_var),
                    },
                ];
                &ARGS
            }

            fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
                &[]
            }

            fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
                let instance_id = args
                    .get("InstanceID")
                    .unwrap_or("0")
                    .parse::<u32>()
                    .unwrap_or(0);
                let desired: u16 = args
                    .get(concat!("Desired", $arg_name))
                    .ok_or(Error::ArgumentValueInvalid)?
                    .parse::<u16>()
                    .map_err(|_| Error::ArgumentValueOutOfRange)?;
                let input = $set_input {
                    instance_id,
                    $desired_field: desired,
                };
                self.trait_impl
                    .$method_set(input)
                    .map_err(|_| Error::ActionFailed)?;
                // Update state variable and trigger LastChange event
                self.service.set_state_var(
                    instance_id,
                    RCStateVarName::$state_var_enum,
                    StateValue::Ui2(desired),
                );
                Ok(ActionArgs::new())
            }
        }
    };
}

impl_video_color!(
    ActionGetRedVideoGain,
    GetRedVideoGain,
    GetRedVideoGainInput,
    GetRedVideoGainOutput,
    get_red_video_gain,
    ActionSetRedVideoGain,
    SetRedVideoGain,
    SetRedVideoGainInput,
    set_red_video_gain,
    "VideoGain",
    "CurrentRedVideoGain",
    current_red_video_gain,
    desired_red_video_gain,
    "RedVideoGain",
    RedVideoGain
);
impl_video_color!(
    ActionGetGreenVideoGain,
    GetGreenVideoGain,
    GetGreenVideoGainInput,
    GetGreenVideoGainOutput,
    get_green_video_gain,
    ActionSetGreenVideoGain,
    SetGreenVideoGain,
    SetGreenVideoGainInput,
    set_green_video_gain,
    "VideoGain",
    "CurrentGreenVideoGain",
    current_green_video_gain,
    desired_green_video_gain,
    "GreenVideoGain",
    GreenVideoGain
);
impl_video_color!(
    ActionGetBlueVideoGain,
    GetBlueVideoGain,
    GetBlueVideoGainInput,
    GetBlueVideoGainOutput,
    get_blue_video_gain,
    ActionSetBlueVideoGain,
    SetBlueVideoGain,
    SetBlueVideoGainInput,
    set_blue_video_gain,
    "VideoGain",
    "CurrentBlueVideoGain",
    current_blue_video_gain,
    desired_blue_video_gain,
    "BlueVideoGain",
    BlueVideoGain
);
impl_video_color!(
    ActionGetRedVideoBlackLevel,
    GetRedVideoBlackLevel,
    GetRedVideoBlackLevelInput,
    GetRedVideoBlackLevelOutput,
    get_red_video_black_level,
    ActionSetRedVideoBlackLevel,
    SetRedVideoBlackLevel,
    SetRedVideoBlackLevelInput,
    set_red_video_black_level,
    "VideoBlackLevel",
    "CurrentRedVideoBlackLevel",
    current_red_video_black_level,
    desired_red_video_black_level,
    "RedVideoBlackLevel",
    RedVideoBlackLevel
);
impl_video_color!(
    ActionGetGreenVideoBlackLevel,
    GetGreenVideoBlackLevel,
    GetGreenVideoBlackLevelInput,
    GetGreenVideoBlackLevelOutput,
    get_green_video_black_level,
    ActionSetGreenVideoBlackLevel,
    SetGreenVideoBlackLevel,
    SetGreenVideoBlackLevelInput,
    set_green_video_black_level,
    "VideoBlackLevel",
    "CurrentGreenVideoBlackLevel",
    current_green_video_black_level,
    desired_green_video_black_level,
    "GreenVideoBlackLevel",
    GreenVideoBlackLevel
);
impl_video_color!(
    ActionGetBlueVideoBlackLevel,
    GetBlueVideoBlackLevel,
    GetBlueVideoBlackLevelInput,
    GetBlueVideoBlackLevelOutput,
    get_blue_video_black_level,
    ActionSetBlueVideoBlackLevel,
    SetBlueVideoBlackLevel,
    SetBlueVideoBlackLevelInput,
    set_blue_video_black_level,
    "VideoBlackLevel",
    "CurrentBlueVideoBlackLevel",
    current_blue_video_black_level,
    desired_blue_video_black_level,
    "BlueVideoBlackLevel",
    BlueVideoBlackLevel
);

// ===========================================================================
// Optional Actions — ColorTemperature / Keystone
// ===========================================================================

pub struct ActionGetColorTemperature<T: GetColorTemperature> {
    trait_impl: Arc<T>,
}

impl<T: GetColorTemperature> ActionGetColorTemperature<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetColorTemperature> Action for ActionGetColorTemperature<T> {
    fn name(&self) -> &'static str {
        "GetColorTemperature"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "CurrentColorTemperature",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("ColorTemperature"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let input = GetColorTemperatureInput { instance_id };
        let output = self
            .trait_impl
            .get_color_temperature(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set(
            "CurrentColorTemperature".to_string(),
            output.current_color_temperature.to_string(),
        );
        Ok(out)
    }
}

pub struct ActionSetColorTemperature<T: SetColorTemperature> {
    trait_impl: Arc<T>,
    service: RenderingControlService,
}

impl<T: SetColorTemperature> ActionSetColorTemperature<T> {
    pub fn new(trait_impl: T, service: RenderingControlService) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            service,
        }
    }
}

impl<T: SetColorTemperature> Action for ActionSetColorTemperature<T> {
    fn name(&self) -> &'static str {
        "SetColorTemperature"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "DesiredColorTemperature",
                direction: ArgumentDirection::IN,
                related_state_var: Some("ColorTemperature"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let desired = args
            .get("DesiredColorTemperature")
            .ok_or(Error::ArgumentValueInvalid)?
            .parse::<u16>()
            .map_err(|_| Error::ArgumentValueOutOfRange)?;
        let input = SetColorTemperatureInput {
            instance_id,
            desired_color_temperature: desired,
        };
        self.trait_impl
            .set_color_temperature(input)
            .map_err(|_| Error::ActionFailed)?;
        self.service.set_state_var(
            instance_id,
            RCStateVarName::ColorTemperature,
            StateValue::Ui2(desired),
        );
        Ok(ActionArgs::new())
    }
}

pub struct ActionGetHorizontalKeystone<T: GetHorizontalKeystone> {
    trait_impl: Arc<T>,
}

impl<T: GetHorizontalKeystone> ActionGetHorizontalKeystone<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetHorizontalKeystone> Action for ActionGetHorizontalKeystone<T> {
    fn name(&self) -> &'static str {
        "GetHorizontalKeystone"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "CurrentHorizontalKeystone",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("HorizontalKeystone"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let input = GetHorizontalKeystoneInput { instance_id };
        let output = self
            .trait_impl
            .get_horizontal_keystone(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set(
            "CurrentHorizontalKeystone".to_string(),
            output.current_horizontal_keystone.to_string(),
        );
        Ok(out)
    }
}

pub struct ActionSetHorizontalKeystone<T: SetHorizontalKeystone> {
    trait_impl: Arc<T>,
    service: RenderingControlService,
}

impl<T: SetHorizontalKeystone> ActionSetHorizontalKeystone<T> {
    pub fn new(trait_impl: T, service: RenderingControlService) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            service,
        }
    }
}

impl<T: SetHorizontalKeystone> Action for ActionSetHorizontalKeystone<T> {
    fn name(&self) -> &'static str {
        "SetHorizontalKeystone"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "DesiredHorizontalKeystone",
                direction: ArgumentDirection::IN,
                related_state_var: Some("HorizontalKeystone"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let desired = args
            .get("DesiredHorizontalKeystone")
            .ok_or(Error::ArgumentValueInvalid)?
            .parse::<i16>()
            .map_err(|_| Error::ArgumentValueOutOfRange)?;
        let input = SetHorizontalKeystoneInput {
            instance_id,
            desired_horizontal_keystone: desired,
        };
        self.trait_impl
            .set_horizontal_keystone(input)
            .map_err(|_| Error::ActionFailed)?;
        self.service.set_state_var(
            instance_id,
            RCStateVarName::HorizontalKeystone,
            StateValue::I2(desired),
        );
        Ok(ActionArgs::new())
    }
}

pub struct ActionGetVerticalKeystone<T: GetVerticalKeystone> {
    trait_impl: Arc<T>,
}

impl<T: GetVerticalKeystone> ActionGetVerticalKeystone<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetVerticalKeystone> Action for ActionGetVerticalKeystone<T> {
    fn name(&self) -> &'static str {
        "GetVerticalKeystone"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "CurrentVerticalKeystone",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("VerticalKeystone"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let input = GetVerticalKeystoneInput { instance_id };
        let output = self
            .trait_impl
            .get_vertical_keystone(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set(
            "CurrentVerticalKeystone".to_string(),
            output.current_vertical_keystone.to_string(),
        );
        Ok(out)
    }
}

pub struct ActionSetVerticalKeystone<T: SetVerticalKeystone> {
    trait_impl: Arc<T>,
    service: RenderingControlService,
}

impl<T: SetVerticalKeystone> ActionSetVerticalKeystone<T> {
    pub fn new(trait_impl: T, service: RenderingControlService) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            service,
        }
    }
}

impl<T: SetVerticalKeystone> Action for ActionSetVerticalKeystone<T> {
    fn name(&self) -> &'static str {
        "SetVerticalKeystone"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "DesiredVerticalKeystone",
                direction: ArgumentDirection::IN,
                related_state_var: Some("VerticalKeystone"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let desired = args
            .get("DesiredVerticalKeystone")
            .ok_or(Error::ArgumentValueInvalid)?
            .parse::<i16>()
            .map_err(|_| Error::ArgumentValueOutOfRange)?;
        let input = SetVerticalKeystoneInput {
            instance_id,
            desired_vertical_keystone: desired,
        };
        self.trait_impl
            .set_vertical_keystone(input)
            .map_err(|_| Error::ActionFailed)?;
        self.service.set_state_var(
            instance_id,
            RCStateVarName::VerticalKeystone,
            StateValue::I2(desired),
        );
        Ok(ActionArgs::new())
    }
}

// ===========================================================================
// Optional Actions — Mute / Volume / VolumeDB / Loudness
// ===========================================================================

pub struct ActionGetMute<T: GetMute> {
    trait_impl: Arc<T>,
}

impl<T: GetMute> ActionGetMute<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetMute> Action for ActionGetMute<T> {
    fn name(&self) -> &'static str {
        "GetMute"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "Channel",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_Channel"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "CurrentMute",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("Mute"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let channel_str = args.get("Channel").unwrap_or("Master");
        let channel = Channel::from_str(channel_str).unwrap_or(Channel::Master);
        let input = GetMuteInput {
            instance_id,
            channel,
        };
        let output = self
            .trait_impl
            .get_mute(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set("CurrentMute".to_string(), output.current_mute.to_string());
        Ok(out)
    }
}

pub struct ActionSetMute<T: SetMute> {
    trait_impl: Arc<T>,
    service: RenderingControlService,
}

impl<T: SetMute> ActionSetMute<T> {
    pub fn new(trait_impl: T, service: RenderingControlService) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            service,
        }
    }
}

impl<T: SetMute> Action for ActionSetMute<T> {
    fn name(&self) -> &'static str {
        "SetMute"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "Channel",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_Channel"),
            },
            Argument {
                name: "DesiredMute",
                direction: ArgumentDirection::IN,
                related_state_var: Some("Mute"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let channel_str = args.get("Channel").unwrap_or("Master");
        let channel = Channel::from_str(channel_str).unwrap_or(Channel::Master);
        // UPnP SOAP sends Boolean as "0"/"1", not "true"/"false"
        let desired = match args.get("DesiredMute") {
            Some("0") => false,
            Some("1") => true,
            Some(_) | None => return Err(Error::ArgumentValueInvalid),
        };
        let input = SetMuteInput {
            instance_id,
            channel,
            desired_mute: desired,
        };
        self.trait_impl
            .set_mute(input)
            .map_err(|_| Error::ActionFailed)?;
        self.service.set_state_var(
            instance_id,
            RCStateVarName::Mute,
            StateValue::Boolean(desired),
        );
        Ok(ActionArgs::new())
    }
}

pub struct ActionGetVolume<T: GetVolume> {
    trait_impl: Arc<T>,
}

impl<T: GetVolume> ActionGetVolume<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetVolume> Action for ActionGetVolume<T> {
    fn name(&self) -> &'static str {
        "GetVolume"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "Channel",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_Channel"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "CurrentVolume",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("Volume"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let channel_str = args.get("Channel").unwrap_or("Master");
        let channel = Channel::from_str(channel_str).unwrap_or(Channel::Master);
        let input = GetVolumeInput {
            instance_id,
            channel,
        };
        let output = self
            .trait_impl
            .get_volume(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set(
            "CurrentVolume".to_string(),
            output.current_volume.to_string(),
        );
        Ok(out)
    }
}

pub struct ActionSetVolume<T: SetVolume> {
    trait_impl: Arc<T>,
    service: RenderingControlService,
}

impl<T: SetVolume> ActionSetVolume<T> {
    pub fn new(trait_impl: T, service: RenderingControlService) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            service,
        }
    }
}

impl<T: SetVolume> Action for ActionSetVolume<T> {
    fn name(&self) -> &'static str {
        "SetVolume"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "Channel",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_Channel"),
            },
            Argument {
                name: "DesiredVolume",
                direction: ArgumentDirection::IN,
                related_state_var: Some("Volume"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let channel_str = args.get("Channel").unwrap_or("Master");
        let channel = Channel::from_str(channel_str).unwrap_or(Channel::Master);
        let desired = args
            .get("DesiredVolume")
            .ok_or(Error::ArgumentValueInvalid)?
            .parse::<u16>()
            .map_err(|_| Error::ArgumentValueOutOfRange)?;
        let input = SetVolumeInput {
            instance_id,
            channel,
            desired_volume: desired,
        };
        self.trait_impl
            .set_volume(input)
            .map_err(|_| Error::ActionFailed)?;
        self.service.set_state_var(
            instance_id,
            RCStateVarName::Volume,
            StateValue::Ui2(desired),
        );
        Ok(ActionArgs::new())
    }
}

pub struct ActionGetVolumeDB<T: GetVolumeDB> {
    trait_impl: Arc<T>,
}

impl<T: GetVolumeDB> ActionGetVolumeDB<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetVolumeDB> Action for ActionGetVolumeDB<T> {
    fn name(&self) -> &'static str {
        "GetVolumeDB"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "Channel",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_Channel"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "CurrentVolume",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("VolumeDB"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let channel_str = args.get("Channel").unwrap_or("Master");
        let channel = Channel::from_str(channel_str).unwrap_or(Channel::Master);
        let input = GetVolumeDBInput {
            instance_id,
            channel,
        };
        let output = self
            .trait_impl
            .get_volume_db(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set(
            "CurrentVolume".to_string(),
            output.current_volume_db.to_string(),
        );
        Ok(out)
    }
}

pub struct ActionSetVolumeDB<T: SetVolumeDB> {
    trait_impl: Arc<T>,
}

impl<T: SetVolumeDB> ActionSetVolumeDB<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: SetVolumeDB> Action for ActionSetVolumeDB<T> {
    fn name(&self) -> &'static str {
        "SetVolumeDB"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "Channel",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_Channel"),
            },
            Argument {
                name: "DesiredVolumeDB",
                direction: ArgumentDirection::IN,
                related_state_var: Some("VolumeDB"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let channel_str = args.get("Channel").unwrap_or("Master");
        let channel = Channel::from_str(channel_str).unwrap_or(Channel::Master);
        let desired = args
            .get("DesiredVolumeDB")
            .ok_or(Error::ArgumentValueInvalid)?
            .parse::<i16>()
            .map_err(|_| Error::ArgumentValueOutOfRange)?;
        let input = SetVolumeDBInput {
            instance_id,
            channel,
            desired_volume_db: desired,
        };
        self.trait_impl
            .set_volume_db(input)
            .map_err(|_| Error::ActionFailed)?;
        Ok(ActionArgs::new())
    }
}

pub struct ActionGetVolumeDBRange<T: GetVolumeDBRange> {
    trait_impl: Arc<T>,
}

impl<T: GetVolumeDBRange> ActionGetVolumeDBRange<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetVolumeDBRange> Action for ActionGetVolumeDBRange<T> {
    fn name(&self) -> &'static str {
        "GetVolumeDBRange"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "Channel",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_Channel"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "MinValue",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("VolumeDB"),
            },
            Argument {
                name: "MaxValue",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("VolumeDB"),
            },
        ];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let channel_str = args.get("Channel").unwrap_or("Master");
        let channel = Channel::from_str(channel_str).unwrap_or(Channel::Master);
        let input = GetVolumeDBRangeInput {
            instance_id,
            channel,
        };
        let output = self
            .trait_impl
            .get_volume_db_range(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set("MinValue".to_string(), output.minimum_value.to_string());
        out.set("MaxValue".to_string(), output.maximum_value.to_string());
        Ok(out)
    }
}

pub struct ActionGetLoudness<T: GetLoudness> {
    trait_impl: Arc<T>,
}

impl<T: GetLoudness> ActionGetLoudness<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetLoudness> Action for ActionGetLoudness<T> {
    fn name(&self) -> &'static str {
        "GetLoudness"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "Channel",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_Channel"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "CurrentLoudness",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("Loudness"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let channel_str = args.get("Channel").unwrap_or("Master");
        let channel = Channel::from_str(channel_str).unwrap_or(Channel::Master);
        let input = GetLoudnessInput {
            instance_id,
            channel,
        };
        let output = self
            .trait_impl
            .get_loudness(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set(
            "CurrentLoudness".to_string(),
            output.current_loudness.to_string(),
        );
        Ok(out)
    }
}

pub struct ActionSetLoudness<T: SetLoudness> {
    trait_impl: Arc<T>,
}

impl<T: SetLoudness> ActionSetLoudness<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: SetLoudness> Action for ActionSetLoudness<T> {
    fn name(&self) -> &'static str {
        "SetLoudness"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "Channel",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_Channel"),
            },
            Argument {
                name: "DesiredLoudness",
                direction: ArgumentDirection::IN,
                related_state_var: Some("Loudness"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let channel_str = args.get("Channel").unwrap_or("Master");
        let channel = Channel::from_str(channel_str).unwrap_or(Channel::Master);
        // UPnP SOAP sends Boolean as "0"/"1", not "true"/"false"
        let desired = match args.get("DesiredLoudness") {
            Some("0") => false,
            Some("1") => true,
            Some(_) | None => return Err(Error::ArgumentValueInvalid),
        };
        let input = SetLoudnessInput {
            instance_id,
            channel,
            desired_loudness: desired,
        };
        self.trait_impl
            .set_loudness(input)
            .map_err(|_| Error::ActionFailed)?;
        Ok(ActionArgs::new())
    }
}

// ===========================================================================
// Optional Actions — Transforms
// ===========================================================================

pub struct ActionGetAllowedTransforms<T: GetAllowedTransforms> {
    trait_impl: Arc<T>,
}

impl<T: GetAllowedTransforms> ActionGetAllowedTransforms<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetAllowedTransforms> Action for ActionGetAllowedTransforms<T> {
    fn name(&self) -> &'static str {
        "GetAllowedTransforms"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "CurrentAllowedTransformSettings",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("AllowedTransformSettings"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let input = GetAllowedTransformsInput { instance_id };
        let output = self
            .trait_impl
            .get_allowed_transforms(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set(
            "CurrentAllowedTransforms".to_string(),
            output.current_allowed_transforms,
        );
        Ok(out)
    }
}

pub struct ActionGetTransforms<T: GetTransforms> {
    trait_impl: Arc<T>,
}

impl<T: GetTransforms> ActionGetTransforms<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetTransforms> Action for ActionGetTransforms<T> {
    fn name(&self) -> &'static str {
        "GetTransforms"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "CurrentTransformValues",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("TransformSettings"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let input = GetTransformsInput { instance_id };
        let output = self
            .trait_impl
            .get_transforms(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set("CurrentTransforms".to_string(), output.current_transforms);
        Ok(out)
    }
}

pub struct ActionSetTransforms<T: SetTransforms> {
    trait_impl: Arc<T>,
}

impl<T: SetTransforms> ActionSetTransforms<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: SetTransforms> Action for ActionSetTransforms<T> {
    fn name(&self) -> &'static str {
        "SetTransforms"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "DesiredTransformValues",
                direction: ArgumentDirection::IN,
                related_state_var: Some("TransformSettings"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let desired_transforms = args
            .get("DesiredTransforms")
            .ok_or(Error::ArgumentValueInvalid)?
            .to_string();
        let input = SetTransformsInput {
            instance_id,
            desired_transforms,
        };
        self.trait_impl
            .set_transforms(input)
            .map_err(|_| Error::ActionFailed)?;
        Ok(ActionArgs::new())
    }
}

pub struct ActionGetAllAvailableTransforms<T: GetAllAvailableTransforms> {
    trait_impl: Arc<T>,
}

impl<T: GetAllAvailableTransforms> ActionGetAllAvailableTransforms<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetAllAvailableTransforms> Action for ActionGetAllAvailableTransforms<T> {
    fn name(&self) -> &'static str {
        "GetAllAvailableTransforms"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "AllAllowedTransformSettings",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("AllowedTransformSettings"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let input = GetAllAvailableTransformsInput { instance_id };
        let output = self
            .trait_impl
            .get_all_available_transforms(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set(
            "AllAvailableTransforms".to_string(),
            output.all_available_transforms,
        );
        Ok(out)
    }
}

// ===========================================================================
// Optional Actions — Default Transforms
// ===========================================================================

pub struct ActionGetAllowedDefaultTransforms<T: GetAllowedDefaultTransforms> {
    trait_impl: Arc<T>,
}

impl<T: GetAllowedDefaultTransforms> ActionGetAllowedDefaultTransforms<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetAllowedDefaultTransforms> Action for ActionGetAllowedDefaultTransforms<T> {
    fn name(&self) -> &'static str {
        "GetAllowedDefaultTransforms"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "AllowedDefaultTransformSettings",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("AllowedDefaultTransformSettings"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let input = GetAllowedDefaultTransformsInput { instance_id };
        let output = self
            .trait_impl
            .get_allowed_default_transforms(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set(
            "CurrentAllowedDefaultTransforms".to_string(),
            output.current_allowed_default_transforms,
        );
        Ok(out)
    }
}

pub struct ActionGetDefaultTransforms<T: GetDefaultTransforms> {
    trait_impl: Arc<T>,
}

impl<T: GetDefaultTransforms> ActionGetDefaultTransforms<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetDefaultTransforms> Action for ActionGetDefaultTransforms<T> {
    fn name(&self) -> &'static str {
        "GetDefaultTransforms"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "CurrentDefaultTransforms",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("DefaultTransformSettings"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let input = GetDefaultTransformsInput { instance_id };
        let output = self
            .trait_impl
            .get_default_transforms(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set(
            "CurrentDefaultTransforms".to_string(),
            output.current_default_transforms,
        );
        Ok(out)
    }
}

pub struct ActionSetDefaultTransforms<T: SetDefaultTransforms> {
    trait_impl: Arc<T>,
}

impl<T: SetDefaultTransforms> ActionSetDefaultTransforms<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: SetDefaultTransforms> Action for ActionSetDefaultTransforms<T> {
    fn name(&self) -> &'static str {
        "SetDefaultTransforms"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "DesiredDefaultTransforms",
                direction: ArgumentDirection::IN,
                related_state_var: Some("DefaultTransformSettings"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let desired_transforms = args
            .get("DesiredDefaultTransforms")
            .ok_or(Error::ArgumentValueInvalid)?
            .to_string();
        let input = SetDefaultTransformsInput {
            instance_id,
            desired_transforms,
        };
        self.trait_impl
            .set_default_transforms(input)
            .map_err(|_| Error::ActionFailed)?;
        Ok(ActionArgs::new())
    }
}

// ===========================================================================
// Optional Actions — StateVariables
// ===========================================================================

pub struct ActionGetStateVariables<T: GetStateVariables> {
    trait_impl: Arc<T>,
}

impl<T: GetStateVariables> ActionGetStateVariables<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetStateVariables> Action for ActionGetStateVariables<T> {
    fn name(&self) -> &'static str {
        "GetStateVariables"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "StateVariableList",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_StateVariableList"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "StateVariableValuePairs",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("A_ARG_TYPE_StateVariableValuePairs"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        // Parse CSV StateVariableList into Vec<String>
        let state_var_list_csv = args.get("StateVariableList").unwrap_or("*");
        let var_list: Vec<String> = if state_var_list_csv == "*" {
            vec!["*".to_string()]
        } else {
            state_var_list_csv
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        };
        let input = GetStateVariablesInput {
            instance_id,
            var_list,
        };
        let output = self
            .trait_impl
            .get_state_variables(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        // Build XML string of name/value pairs
        let pairs: String = output
            .values
            .iter()
            .map(|(name, value)| {
                format!(
                    "<stateVariable variableName=\"{}\">{}</stateVariable>",
                    name, value
                )
            })
            .collect::<Vec<_>>()
            .join("");
        out.set(
            "StateVariableValuePairs".to_string(),
            format!("<stateVariableValuePairs xmlns=\"urn:schemas-upnp-org:av:avs\">{}</stateVariableValuePairs>", pairs),
        );
        Ok(out)
    }
}

pub struct ActionSetStateVariables<T: SetStateVariables> {
    trait_impl: Arc<T>,
}

impl<T: SetStateVariables> ActionSetStateVariables<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: SetStateVariables> Action for ActionSetStateVariables<T> {
    fn name(&self) -> &'static str {
        "SetStateVariables"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "RenderingControlUDN",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_DeviceUDN"),
            },
            Argument {
                name: "ServiceType",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_ServiceType"),
            },
            Argument {
                name: "ServiceId",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_ServiceID"),
            },
            Argument {
                name: "StateVariableValuePairs",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_StateVariableValuePairs"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "StateVariableList",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("A_ARG_TYPE_StateVariableList"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let state_var_xml = args
            .get("StateVariableValuePairs")
            .ok_or(Error::ArgumentValueInvalid)?;
        // Parse XML to extract Vec<(variable_name, value)> pairs
        let mut pairs: Vec<(String, String)> = Vec::new();
        let mut reader = quick_xml::Reader::from_str(state_var_xml);
        reader.config_mut().trim_text(true);
        let mut buf = Vec::new();
        let mut current_name: Option<String> = None;
        let mut current_value: Option<String> = None;
        let mut in_var = false;
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(quick_xml::events::Event::Start(e)) => {
                    if e.name() == quick_xml::name::QName("stateVariable") {
                        in_var = true;
                        // Extract variableName attribute
                        for attr_result in e.attributes() {
                            if let Ok(attr) = attr_result {
                                if attr.key.as_ref() == "variableName" {
                                    current_name = Some(attr.value.to_string());
                                }
                            }
                        }
                        current_value = Some(String::new());
                    }
                }
                Ok(quick_xml::events::Event::End(e)) => {
                    if e.name() == quick_xml::name::QName("stateVariable") {
                        if let (Some(name), Some(value)) =
                            (current_name.take(), current_value.take())
                        {
                            pairs.push((name, value.trim().to_string()));
                        }
                        in_var = false;
                    }
                }
                Ok(quick_xml::events::Event::Text(t)) => {
                    if in_var {
                        if let Some(ref mut val) = current_value {
                            val.push_str(&t);
                        }
                    }
                }
                Ok(quick_xml::events::Event::Eof) => break,
                _ => {}
            }
            buf.clear();
        }
        let input = SetStateVariablesInput { instance_id, pairs };
        self.trait_impl
            .set_state_variables(input)
            .map_err(|_| Error::ActionFailed)?;
        Ok(ActionArgs::new())
    }
}
