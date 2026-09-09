/// RenderingControl action traits — application-facing API.
///
/// Each trait represents one UPnP RenderingControl action with typed parameters.
/// The application implements these traits to provide actual behavior.
/// No UPnP protocol knowledge required.
use std::fmt::Display;

use super::r#static::Channel;

// ===========================================================================
// Input Types
// ===========================================================================

#[derive(Debug, Clone)]
pub struct SelectPresetInput {
    pub instance_id: u32,
    pub preset_name: String,
}

#[derive(Debug, Clone)]
pub struct GetBrightnessInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct SetBrightnessInput {
    pub instance_id: u32,
    pub desired_brightness: u16,
}

#[derive(Debug, Clone)]
pub struct GetContrastInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct SetContrastInput {
    pub instance_id: u32,
    pub desired_contrast: u16,
}

#[derive(Debug, Clone)]
pub struct GetSharpnessInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct SetSharpnessInput {
    pub instance_id: u32,
    pub desired_sharpness: u16,
}

#[derive(Debug, Clone)]
pub struct GetRedVideoGainInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct SetRedVideoGainInput {
    pub instance_id: u32,
    pub desired_red_video_gain: u16,
}

#[derive(Debug, Clone)]
pub struct GetGreenVideoGainInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct SetGreenVideoGainInput {
    pub instance_id: u32,
    pub desired_green_video_gain: u16,
}

#[derive(Debug, Clone)]
pub struct GetBlueVideoGainInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct SetBlueVideoGainInput {
    pub instance_id: u32,
    pub desired_blue_video_gain: u16,
}

#[derive(Debug, Clone)]
pub struct GetRedVideoBlackLevelInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct SetRedVideoBlackLevelInput {
    pub instance_id: u32,
    pub desired_red_video_black_level: u16,
}

#[derive(Debug, Clone)]
pub struct GetGreenVideoBlackLevelInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct SetGreenVideoBlackLevelInput {
    pub instance_id: u32,
    pub desired_green_video_black_level: u16,
}

#[derive(Debug, Clone)]
pub struct GetBlueVideoBlackLevelInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct SetBlueVideoBlackLevelInput {
    pub instance_id: u32,
    pub desired_blue_video_black_level: u16,
}

#[derive(Debug, Clone)]
pub struct GetColorTemperatureInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct SetColorTemperatureInput {
    pub instance_id: u32,
    pub desired_color_temperature: u16,
}

#[derive(Debug, Clone)]
pub struct GetHorizontalKeystoneInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct SetHorizontalKeystoneInput {
    pub instance_id: u32,
    pub desired_horizontal_keystone: i16,
}

#[derive(Debug, Clone)]
pub struct GetVerticalKeystoneInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct SetVerticalKeystoneInput {
    pub instance_id: u32,
    pub desired_vertical_keystone: i16,
}

#[derive(Debug, Clone)]
pub struct GetMuteInput {
    pub instance_id: u32,
    pub channel: Channel,
}

#[derive(Debug, Clone)]
pub struct SetMuteInput {
    pub instance_id: u32,
    pub channel: Channel,
    pub desired_mute: bool,
}

#[derive(Debug, Clone)]
pub struct GetVolumeInput {
    pub instance_id: u32,
    pub channel: Channel,
}

#[derive(Debug, Clone)]
pub struct SetVolumeInput {
    pub instance_id: u32,
    pub channel: Channel,
    pub desired_volume: u16,
}

#[derive(Debug, Clone)]
pub struct GetVolumeDBInput {
    pub instance_id: u32,
    pub channel: Channel,
}

#[derive(Debug, Clone)]
pub struct SetVolumeDBInput {
    pub instance_id: u32,
    pub channel: Channel,
    pub desired_volume_db: i16,
}

#[derive(Debug, Clone)]
pub struct GetVolumeDBRangeInput {
    pub instance_id: u32,
    pub channel: Channel,
}

#[derive(Debug, Clone)]
pub struct GetLoudnessInput {
    pub instance_id: u32,
    pub channel: Channel,
}

#[derive(Debug, Clone)]
pub struct SetLoudnessInput {
    pub instance_id: u32,
    pub channel: Channel,
    pub desired_loudness: bool,
}

#[derive(Debug, Clone)]
pub struct GetStateVariablesInput {
    pub instance_id: u32,
    pub var_list: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SetStateVariablesInput {
    pub instance_id: u32,
    pub pairs: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct GetAllowedTransformsInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct GetTransformsInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct SetTransformsInput {
    pub instance_id: u32,
    pub desired_transforms: String,
}

#[derive(Debug, Clone)]
pub struct GetAllowedDefaultTransformsInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct GetDefaultTransformsInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct SetDefaultTransformsInput {
    pub instance_id: u32,
    pub desired_transforms: String,
}

#[derive(Debug, Clone)]
pub struct GetAllAvailableTransformsInput {
    pub instance_id: u32,
}

// ===========================================================================
// Output Types
// ===========================================================================

#[derive(Debug, Clone, Default)]
pub struct ListPresetsOutput {
    pub current_preset_name_list: String,
}

#[derive(Debug, Clone, Default)]
pub struct SelectPresetOutput {}

#[derive(Debug, Clone)]
pub struct GetBrightnessOutput {
    pub current_brightness: u16,
}

#[derive(Debug, Clone, Default)]
pub struct SetBrightnessOutput {}

#[derive(Debug, Clone)]
pub struct GetContrastOutput {
    pub current_contrast: u16,
}

#[derive(Debug, Clone, Default)]
pub struct SetContrastOutput {}

#[derive(Debug, Clone)]
pub struct GetSharpnessOutput {
    pub current_sharpness: u16,
}

#[derive(Debug, Clone, Default)]
pub struct SetSharpnessOutput {}

#[derive(Debug, Clone)]
pub struct GetRedVideoGainOutput {
    pub current_red_video_gain: u16,
}

#[derive(Debug, Clone, Default)]
pub struct SetRedVideoGainOutput {}

#[derive(Debug, Clone)]
pub struct GetGreenVideoGainOutput {
    pub current_green_video_gain: u16,
}

#[derive(Debug, Clone, Default)]
pub struct SetGreenVideoGainOutput {}

#[derive(Debug, Clone)]
pub struct GetBlueVideoGainOutput {
    pub current_blue_video_gain: u16,
}

#[derive(Debug, Clone, Default)]
pub struct SetBlueVideoGainOutput {}

#[derive(Debug, Clone)]
pub struct GetRedVideoBlackLevelOutput {
    pub current_red_video_black_level: u16,
}

#[derive(Debug, Clone, Default)]
pub struct SetRedVideoBlackLevelOutput {}

#[derive(Debug, Clone)]
pub struct GetGreenVideoBlackLevelOutput {
    pub current_green_video_black_level: u16,
}

#[derive(Debug, Clone, Default)]
pub struct SetGreenVideoBlackLevelOutput {}

#[derive(Debug, Clone)]
pub struct GetBlueVideoBlackLevelOutput {
    pub current_blue_video_black_level: u16,
}

#[derive(Debug, Clone, Default)]
pub struct SetBlueVideoBlackLevelOutput {}

#[derive(Debug, Clone)]
pub struct GetColorTemperatureOutput {
    pub current_color_temperature: u16,
}

#[derive(Debug, Clone, Default)]
pub struct SetColorTemperatureOutput {}

#[derive(Debug, Clone)]
pub struct GetHorizontalKeystoneOutput {
    pub current_horizontal_keystone: i16,
}

#[derive(Debug, Clone, Default)]
pub struct SetHorizontalKeystoneOutput {}

#[derive(Debug, Clone)]
pub struct GetVerticalKeystoneOutput {
    pub current_vertical_keystone: i16,
}

#[derive(Debug, Clone, Default)]
pub struct SetVerticalKeystoneOutput {}

#[derive(Debug, Clone)]
pub struct GetMuteOutput {
    pub current_mute: bool,
}

#[derive(Debug, Clone, Default)]
pub struct SetMuteOutput {}

#[derive(Debug, Clone)]
pub struct GetVolumeOutput {
    pub current_volume: u16,
}

#[derive(Debug, Clone, Default)]
pub struct SetVolumeOutput {}

#[derive(Debug, Clone)]
pub struct GetVolumeDBOutput {
    pub current_volume_db: i16,
}

#[derive(Debug, Clone, Default)]
pub struct SetVolumeDBOutput {}

#[derive(Debug, Clone)]
pub struct GetVolumeDBRangeOutput {
    pub minimum_value: i16,
    pub maximum_value: i16,
    #[allow(dead_code)]
    pub qualified_min_value: i16,
    #[allow(dead_code)]
    pub qualified_max_value: i16,
}

#[derive(Debug, Clone)]
pub struct GetLoudnessOutput {
    pub current_loudness: bool,
}

#[derive(Debug, Clone, Default)]
pub struct SetLoudnessOutput {}

#[derive(Debug, Clone)]
pub struct GetStateVariablesOutput {
    pub values: Vec<(String, String)>,
}

#[derive(Debug, Clone, Default)]
pub struct SetStateVariablesOutput {}

#[derive(Debug, Clone)]
pub struct GetAllowedTransformsOutput {
    pub current_allowed_transforms: String,
}

#[derive(Debug, Clone)]
pub struct GetTransformsOutput {
    pub current_transforms: String,
}

#[derive(Debug, Clone, Default)]
pub struct SetTransformsOutput {}

#[derive(Debug, Clone)]
pub struct GetAllowedDefaultTransformsOutput {
    pub current_allowed_default_transforms: String,
}

#[derive(Debug, Clone)]
pub struct GetDefaultTransformsOutput {
    pub current_default_transforms: String,
}

#[derive(Debug, Clone, Default)]
pub struct SetDefaultTransformsOutput {}

#[derive(Debug, Clone)]
pub struct GetAllAvailableTransformsOutput {
    pub all_available_transforms: String,
}

// ===========================================================================
// Required Actions (R)
// ===========================================================================

/// ListPresets — Return the current list of available presets.
pub trait ListPresets: Send + Sync {
    type Error: Display;
    fn list_presets(&self, instance_id: u32) -> Result<ListPresetsOutput, Self::Error>;
}

/// SelectPreset — Restore state variables to values from the specified preset.
pub trait SelectPreset: Send + Sync {
    type Error: Display;
    fn select_preset(&self, input: SelectPresetInput) -> Result<SelectPresetOutput, Self::Error>;
}

// ===========================================================================
// Optional Actions (O) — Brightness / Contrast / Sharpness
// ===========================================================================

/// GetBrightness — Return brightness level.
pub trait GetBrightness: Send + Sync {
    type Error: Display;
    fn get_brightness(&self, input: GetBrightnessInput)
    -> Result<GetBrightnessOutput, Self::Error>;
}

/// SetBrightness — Set brightness level.
pub trait SetBrightness: Send + Sync {
    type Error: Display;
    fn set_brightness(&self, input: SetBrightnessInput)
    -> Result<SetBrightnessOutput, Self::Error>;
}

/// GetContrast — Return contrast level.
pub trait GetContrast: Send + Sync {
    type Error: Display;
    fn get_contrast(&self, input: GetContrastInput) -> Result<GetContrastOutput, Self::Error>;
}

/// SetContrast — Set contrast level.
pub trait SetContrast: Send + Sync {
    type Error: Display;
    fn set_contrast(&self, input: SetContrastInput) -> Result<SetContrastOutput, Self::Error>;
}

/// GetSharpness — Return sharpness level.
pub trait GetSharpness: Send + Sync {
    type Error: Display;
    fn get_sharpness(&self, input: GetSharpnessInput) -> Result<GetSharpnessOutput, Self::Error>;
}

/// SetSharpness — Set sharpness level.
pub trait SetSharpness: Send + Sync {
    type Error: Display;
    fn set_sharpness(&self, input: SetSharpnessInput) -> Result<SetSharpnessOutput, Self::Error>;
}

// ===========================================================================
// Optional Actions (O) — Video Color Controls
// ===========================================================================

/// GetRedVideoGain — Return red video gain level.
pub trait GetRedVideoGain: Send + Sync {
    type Error: Display;
    fn get_red_video_gain(
        &self,
        input: GetRedVideoGainInput,
    ) -> Result<GetRedVideoGainOutput, Self::Error>;
}

/// SetRedVideoGain — Set red video gain level.
pub trait SetRedVideoGain: Send + Sync {
    type Error: Display;
    fn set_red_video_gain(
        &self,
        input: SetRedVideoGainInput,
    ) -> Result<SetRedVideoGainOutput, Self::Error>;
}

/// GetGreenVideoGain — Return green video gain level.
pub trait GetGreenVideoGain: Send + Sync {
    type Error: Display;
    fn get_green_video_gain(
        &self,
        input: GetGreenVideoGainInput,
    ) -> Result<GetGreenVideoGainOutput, Self::Error>;
}

/// SetGreenVideoGain — Set green video gain level.
pub trait SetGreenVideoGain: Send + Sync {
    type Error: Display;
    fn set_green_video_gain(
        &self,
        input: SetGreenVideoGainInput,
    ) -> Result<SetGreenVideoGainOutput, Self::Error>;
}

/// GetBlueVideoGain — Return blue video gain level.
pub trait GetBlueVideoGain: Send + Sync {
    type Error: Display;
    fn get_blue_video_gain(
        &self,
        input: GetBlueVideoGainInput,
    ) -> Result<GetBlueVideoGainOutput, Self::Error>;
}

/// SetBlueVideoGain — Set blue video gain level.
pub trait SetBlueVideoGain: Send + Sync {
    type Error: Display;
    fn set_blue_video_gain(
        &self,
        input: SetBlueVideoGainInput,
    ) -> Result<SetBlueVideoGainOutput, Self::Error>;
}

/// GetRedVideoBlackLevel — Return red video black level.
pub trait GetRedVideoBlackLevel: Send + Sync {
    type Error: Display;
    fn get_red_video_black_level(
        &self,
        input: GetRedVideoBlackLevelInput,
    ) -> Result<GetRedVideoBlackLevelOutput, Self::Error>;
}

/// SetRedVideoBlackLevel — Set red video black level.
pub trait SetRedVideoBlackLevel: Send + Sync {
    type Error: Display;
    fn set_red_video_black_level(
        &self,
        input: SetRedVideoBlackLevelInput,
    ) -> Result<SetRedVideoBlackLevelOutput, Self::Error>;
}

/// GetGreenVideoBlackLevel — Return green video black level.
pub trait GetGreenVideoBlackLevel: Send + Sync {
    type Error: Display;
    fn get_green_video_black_level(
        &self,
        input: GetGreenVideoBlackLevelInput,
    ) -> Result<GetGreenVideoBlackLevelOutput, Self::Error>;
}

/// SetGreenVideoBlackLevel — Set green video black level.
pub trait SetGreenVideoBlackLevel: Send + Sync {
    type Error: Display;
    fn set_green_video_black_level(
        &self,
        input: SetGreenVideoBlackLevelInput,
    ) -> Result<SetGreenVideoBlackLevelOutput, Self::Error>;
}

/// GetBlueVideoBlackLevel — Return blue video black level.
pub trait GetBlueVideoBlackLevel: Send + Sync {
    type Error: Display;
    fn get_blue_video_black_level(
        &self,
        input: GetBlueVideoBlackLevelInput,
    ) -> Result<GetBlueVideoBlackLevelOutput, Self::Error>;
}

/// SetBlueVideoBlackLevel — Set blue video black level.
pub trait SetBlueVideoBlackLevel: Send + Sync {
    type Error: Display;
    fn set_blue_video_black_level(
        &self,
        input: SetBlueVideoBlackLevelInput,
    ) -> Result<SetBlueVideoBlackLevelOutput, Self::Error>;
}

/// GetColorTemperature — Return color temperature.
pub trait GetColorTemperature: Send + Sync {
    type Error: Display;
    fn get_color_temperature(
        &self,
        input: GetColorTemperatureInput,
    ) -> Result<GetColorTemperatureOutput, Self::Error>;
}

/// SetColorTemperature — Set color temperature.
pub trait SetColorTemperature: Send + Sync {
    type Error: Display;
    fn set_color_temperature(
        &self,
        input: SetColorTemperatureInput,
    ) -> Result<SetColorTemperatureOutput, Self::Error>;
}

// ===========================================================================
// Optional Actions (O) — Keystone Controls
// ===========================================================================

/// GetHorizontalKeystone — Return horizontal keystone value.
pub trait GetHorizontalKeystone: Send + Sync {
    type Error: Display;
    fn get_horizontal_keystone(
        &self,
        input: GetHorizontalKeystoneInput,
    ) -> Result<GetHorizontalKeystoneOutput, Self::Error>;
}

/// SetHorizontalKeystone — Set horizontal keystone value.
pub trait SetHorizontalKeystone: Send + Sync {
    type Error: Display;
    fn set_horizontal_keystone(
        &self,
        input: SetHorizontalKeystoneInput,
    ) -> Result<SetHorizontalKeystoneOutput, Self::Error>;
}

/// GetVerticalKeystone — Return vertical keystone value.
pub trait GetVerticalKeystone: Send + Sync {
    type Error: Display;
    fn get_vertical_keystone(
        &self,
        input: GetVerticalKeystoneInput,
    ) -> Result<GetVerticalKeystoneOutput, Self::Error>;
}

/// SetVerticalKeystone — Set vertical keystone value.
pub trait SetVerticalKeystone: Send + Sync {
    type Error: Display;
    fn set_vertical_keystone(
        &self,
        input: SetVerticalKeystoneInput,
    ) -> Result<SetVerticalKeystoneOutput, Self::Error>;
}

// ===========================================================================
// Optional Actions (O) — Audio Controls
// ===========================================================================

/// GetMute — Return mute state for a channel.
pub trait GetMute: Send + Sync {
    type Error: Display;
    fn get_mute(&self, input: GetMuteInput) -> Result<GetMuteOutput, Self::Error>;
}

/// SetMute — Set mute on/off for a channel.
pub trait SetMute: Send + Sync {
    type Error: Display;
    fn set_mute(&self, input: SetMuteInput) -> Result<SetMuteOutput, Self::Error>;
}

/// GetVolume — Return current volume level for a channel.
pub trait GetVolume: Send + Sync {
    type Error: Display;
    fn get_volume(&self, input: GetVolumeInput) -> Result<GetVolumeOutput, Self::Error>;
}

/// SetVolume — Set volume level for a channel.
pub trait SetVolume: Send + Sync {
    type Error: Display;
    fn set_volume(&self, input: SetVolumeInput) -> Result<SetVolumeOutput, Self::Error>;
}

/// GetVolumeDB — Return current volume in dB for a channel.
pub trait GetVolumeDB: Send + Sync {
    type Error: Display;
    fn get_volume_db(&self, input: GetVolumeDBInput) -> Result<GetVolumeDBOutput, Self::Error>;
}

/// SetVolumeDB — Set volume in dB for a channel.
pub trait SetVolumeDB: Send + Sync {
    type Error: Display;
    fn set_volume_db(&self, input: SetVolumeDBInput) -> Result<SetVolumeDBOutput, Self::Error>;
}

/// GetVolumeDBRange — Return valid dB range for a channel.
pub trait GetVolumeDBRange: Send + Sync {
    type Error: Display;
    fn get_volume_db_range(
        &self,
        input: GetVolumeDBRangeInput,
    ) -> Result<GetVolumeDBRangeOutput, Self::Error>;
}

/// GetLoudness — Return loudness state for a channel.
pub trait GetLoudness: Send + Sync {
    type Error: Display;
    fn get_loudness(&self, input: GetLoudnessInput) -> Result<GetLoudnessOutput, Self::Error>;
}

/// SetLoudness — Set loudness on/off for a channel.
pub trait SetLoudness: Send + Sync {
    type Error: Display;
    fn set_loudness(&self, input: SetLoudnessInput) -> Result<SetLoudnessOutput, Self::Error>;
}

// ===========================================================================
// Conditionally Required Actions (CR)
// ===========================================================================

/// GetStateVariables — Get state variable values.
pub trait GetStateVariables: Send + Sync {
    type Error: Display;
    fn get_state_variables(
        &self,
        input: GetStateVariablesInput,
    ) -> Result<GetStateVariablesOutput, Self::Error>;
}

/// SetStateVariables — Set state variable values.
pub trait SetStateVariables: Send + Sync {
    type Error: Display;
    fn set_state_variables(
        &self,
        input: SetStateVariablesInput,
    ) -> Result<SetStateVariablesOutput, Self::Error>;
}

/// GetAllowedTransforms — Return allowed transform settings.
pub trait GetAllowedTransforms: Send + Sync {
    type Error: Display;
    fn get_allowed_transforms(
        &self,
        input: GetAllowedTransformsInput,
    ) -> Result<GetAllowedTransformsOutput, Self::Error>;
}

/// GetTransforms — Return current transform settings.
pub trait GetTransforms: Send + Sync {
    type Error: Display;
    fn get_transforms(&self, input: GetTransformsInput)
    -> Result<GetTransformsOutput, Self::Error>;
}

/// SetTransforms — Apply transform settings.
pub trait SetTransforms: Send + Sync {
    type Error: Display;
    fn set_transforms(&self, input: SetTransformsInput)
    -> Result<SetTransformsOutput, Self::Error>;
}

/// GetAllowedDefaultTransforms — Return allowed default transform settings.
pub trait GetAllowedDefaultTransforms: Send + Sync {
    type Error: Display;
    fn get_allowed_default_transforms(
        &self,
        input: GetAllowedDefaultTransformsInput,
    ) -> Result<GetAllowedDefaultTransformsOutput, Self::Error>;
}

/// GetDefaultTransforms — Return default transform settings.
pub trait GetDefaultTransforms: Send + Sync {
    type Error: Display;
    fn get_default_transforms(
        &self,
        input: GetDefaultTransformsInput,
    ) -> Result<GetDefaultTransformsOutput, Self::Error>;
}

/// SetDefaultTransforms — Set default transform settings.
pub trait SetDefaultTransforms: Send + Sync {
    type Error: Display;
    fn set_default_transforms(
        &self,
        input: SetDefaultTransformsInput,
    ) -> Result<SetDefaultTransformsOutput, Self::Error>;
}

/// GetAllAvailableTransforms — Return all available transforms.
pub trait GetAllAvailableTransforms: Send + Sync {
    type Error: Display;
    fn get_all_available_transforms(
        &self,
        input: GetAllAvailableTransformsInput,
    ) -> Result<GetAllAvailableTransformsOutput, Self::Error>;
}
