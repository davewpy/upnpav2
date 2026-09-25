/// AVTransport action traits - application-facing API.
///
/// Each trait represents one UPnP AVTransport action with typed parameters.
/// The application implements these traits to provide actual behavior.
/// No UPnP protocol knowledge required.
use std::fmt::Display;

use super::r#static::{PlayMode, StorageMedium, TransportState, TransportStatus};

// ===========================================================================
// Input Types - library-owned, application receives these in trait methods
// ===========================================================================

#[derive(Debug, Clone)]
pub struct PlayInput {
    pub instance_id: u32,
    pub speed: String,
}

#[derive(Debug, Clone)]
pub struct StopInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct PauseInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct SeekInput {
    pub instance_id: u32,
    pub unit: String,
    pub target: String,
}

#[derive(Debug, Clone)]
pub struct NextInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct PreviousInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct SetAVTransportURIInput {
    pub instance_id: u32,
    pub current_uri: String,
    pub current_uri_metadata: String,
}

#[derive(Debug, Clone)]
pub struct SetNextAVTransportURIInput {
    pub instance_id: u32,
    pub next_uri: String,
    pub next_uri_metadata: String,
}

#[derive(Debug, Clone)]
pub struct GetMediaInfoInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct GetTransportInfoInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct GetPositionInfoInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct GetDeviceCapabilitiesInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct GetTransportSettingsInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct SetPlayModeInput {
    pub instance_id: u32,
    pub play_mode: PlayMode,
}

#[derive(Debug, Clone)]
pub struct GetCurrentTransportActionsInput {
    pub instance_id: u32,
}

#[derive(Debug, Clone)]
pub struct SetRecordQualityModeInput {
    pub instance_id: u32,
    pub rec_quality_mode: String,
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
pub struct SetSyncOffsetInput {
    pub instance_id: u32,
    pub new_sync_offset: String,
}

#[derive(Debug, Clone)]
pub struct AdjustSyncOffsetInput {
    pub instance_id: u32,
    pub adjustment: i32,
    pub sync_point: String,
}

#[derive(Debug, Clone)]
pub struct SetStaticPlaylistInput {
    pub instance_id: u32,
    pub playlist_data: String,
    pub playlist_data_length: u32,
    pub playlist_offset: u32,
    pub playlist_total_length: u32,
}

#[derive(Debug, Clone)]
pub struct SetStreamingPlaylistInput {
    pub instance_id: u32,
    pub playlist_data: String,
    pub playlist_data_length: u32,
    pub playlist_offset: u32,
    pub playlist_total_length: u32,
    pub mime_type: String,
    pub extended_type: String,
    pub step: String,
    pub playlist_type: String,
}

// ===========================================================================
// Output Types - library-owned, application uses these directly
// ===========================================================================

#[derive(Debug, Clone, Default)]
pub struct PlayOutput {}

#[derive(Debug, Clone, Default)]
pub struct StopOutput {}

#[derive(Debug, Clone, Default)]
pub struct PauseOutput {}

#[derive(Debug, Clone, Default)]
pub struct SeekOutput {}

#[derive(Debug, Clone, Default)]
pub struct NextOutput {}

#[derive(Debug, Clone, Default)]
pub struct PreviousOutput {}

#[derive(Debug, Clone, Default)]
pub struct SetAVTransportURIOutput {}

#[derive(Debug, Clone, Default)]
pub struct SetNextAVTransportURIOutput {}

#[derive(Debug, Clone)]
pub struct GetMediaInfoOutput {
    pub nr_tracks: u32,
    pub media_duration: String,
    pub current_uri: String,
    pub current_uri_metadata: String,
    pub next_uri: String,
    pub next_uri_metadata: String,
    pub play_medium: StorageMedium,
    pub record_medium: StorageMedium,
    pub write_status: super::r#static::RecordMediumWriteStatus,
}

#[derive(Debug, Clone)]
pub struct GetTransportInfoOutput {
    pub transport_state: TransportState,
    pub transport_status: TransportStatus,
    pub play_speed: String,
}

#[derive(Debug, Clone)]
pub struct GetPositionInfoOutput {
    pub track: u32,
    pub track_duration: String,
    pub track_metadata: String,
    pub track_uri: String,
    pub rel_time: String,
    pub abs_time: String,
    pub rel_count: i32,
    pub abs_count: u32,
}

#[derive(Debug, Clone)]
pub struct GetDeviceCapabilitiesOutput {
    pub play_media: String,
    pub rec_media: String,
    pub rec_quality_modes: String,
}

#[derive(Debug, Clone)]
pub struct GetTransportSettingsOutput {
    pub play_mode: PlayMode,
    pub rec_quality_mode: String,
}

#[derive(Debug, Clone, Default)]
pub struct SetPlayModeOutput {}

#[derive(Debug, Clone)]
pub struct GetCurrentTransportActionsOutput {
    pub actions: String,
}

#[derive(Debug, Clone, Default)]
pub struct SetRecordQualityModeOutput {}

#[derive(Debug, Clone)]
pub struct GetDRMStateOutput {
    pub drm_state: String,
}

#[derive(Debug, Clone)]
pub struct GetStateVariablesOutput {
    pub values: Vec<(String, String)>,
}

#[derive(Debug, Clone, Default)]
pub struct SetStateVariablesOutput {}

#[derive(Debug, Clone)]
pub struct GetSyncOffsetOutput {
    pub sync_offset: String,
}

#[derive(Debug, Clone, Default)]
pub struct SetSyncOffsetOutput {}

#[derive(Debug, Clone, Default)]
pub struct AdjustSyncOffsetOutput {}

#[derive(Debug, Clone, Default)]
pub struct SyncPlayOutput {}

#[derive(Debug, Clone, Default)]
pub struct SyncStopOutput {}

#[derive(Debug, Clone, Default)]
pub struct SyncPauseOutput {}

#[derive(Debug, Clone)]
pub struct SetStaticPlaylistOutput {
    pub playlist_data_length: u32,
    pub playlist_offset: u32,
    pub playlist_total_length: u32,
}

#[derive(Debug, Clone)]
pub struct SetStreamingPlaylistOutput {
    pub playlist_data_length: u32,
    pub playlist_offset: u32,
    pub playlist_total_length: u32,
}

#[derive(Debug, Clone)]
pub struct GetPlaylistInfoOutput {
    pub playlist_data: String,
    pub playlist_data_length: u32,
    pub playlist_offset: u32,
    pub playlist_total_length: u32,
    pub playlist_mime_type: String,
    pub playlist_extended_type: String,
    pub playlist_step: String,
    pub playlist_type: String,
    pub playlist_info: String,
}

#[derive(Debug, Clone)]
pub struct GetMediaInfoExtOutput {
    pub nr_tracks: u32,
    pub media_duration: String,
    pub current_uri: String,
    pub current_uri_metadata: String,
    pub next_uri: String,
    pub next_uri_metadata: String,
    pub play_medium: StorageMedium,
    pub record_medium: StorageMedium,
    pub write_status: super::r#static::RecordMediumWriteStatus,
    pub current_track_metadata: String,
    pub track_duration: String,
    pub track_uri: String,
    pub rel_time: String,
    pub abs_time: String,
    pub rel_count: i32,
    pub abs_count: u32,
    pub transport_state: TransportState,
    pub transport_status: TransportStatus,
    pub transport_speed: String,
    pub track_nav_enabled: bool,
}

// ===========================================================================
// Required Actions (R) - MVP Core
// ===========================================================================

/// Play — Start playback at current position, specified speed.
pub trait Play: Send + Sync {
    type Error: Display;
    fn play(&self, input: PlayInput) -> Result<PlayOutput, Self::Error>;
}

/// Stop — Stop media progression.
pub trait Stop: Send + Sync {
    type Error: Display;
    fn stop(&self, input: StopInput) -> Result<StopOutput, Self::Error>;
}

/// Pause — Halt progression, hold position.
pub trait Pause: Send + Sync {
    type Error: Display;
    fn pause(&self, input: PauseInput) -> Result<PauseOutput, Self::Error>;
}

/// Seek — Seek to position within media.
pub trait Seek: Send + Sync {
    type Error: Display;
    fn seek(&self, input: SeekInput) -> Result<SeekOutput, Self::Error>;
}

/// Next — Advance to next track.
pub trait Next: Send + Sync {
    type Error: Display;
    fn next(&self, input: NextInput) -> Result<NextOutput, Self::Error>;
}

/// Previous — Advance to previous track.
pub trait Previous: Send + Sync {
    type Error: Display;
    fn previous(&self, input: PreviousInput) -> Result<PreviousOutput, Self::Error>;
}

/// SetAVTransportURI — Set the resource URI for playback.
pub trait SetAVTransportURI: Send + Sync {
    type Error: Display;
    fn set_av_transport_uri(
        &self,
        input: SetAVTransportURIInput,
    ) -> Result<SetAVTransportURIOutput, Self::Error>;
}

/// GetMediaInfo — Return media information for the current instance.
pub trait GetMediaInfo: Send + Sync {
    type Error: Display;
    fn get_media_info(&self, input: GetMediaInfoInput) -> Result<GetMediaInfoOutput, Self::Error>;
}

/// GetTransportInfo — Return current transport state.
pub trait GetTransportInfo: Send + Sync {
    type Error: Display;
    fn get_transport_info(
        &self,
        input: GetTransportInfoInput,
    ) -> Result<GetTransportInfoOutput, Self::Error>;
}

/// GetPositionInfo — Return current playback position.
pub trait GetPositionInfo: Send + Sync {
    type Error: Display;
    fn get_position_info(
        &self,
        input: GetPositionInfoInput,
    ) -> Result<GetPositionInfoOutput, Self::Error>;
}

/// GetDeviceCapabilities — Return supported playback/recording formats.
pub trait GetDeviceCapabilities: Send + Sync {
    type Error: Display;
    fn get_device_capabilities(
        &self,
        input: GetDeviceCapabilitiesInput,
    ) -> Result<GetDeviceCapabilitiesOutput, Self::Error>;
}

/// GetTransportSettings — Return current play/recording settings.
pub trait GetTransportSettings: Send + Sync {
    type Error: Display;
    fn get_transport_settings(
        &self,
        input: GetTransportSettingsInput,
    ) -> Result<GetTransportSettingsOutput, Self::Error>;
}

// ===========================================================================
// Allowed Actions (A)
// ===========================================================================

/// SetNextAVTransportURI — Set the URI to play after current resource finishes.
pub trait SetNextAVTransportURI: Send + Sync {
    type Error: Display;
    fn set_next_av_transport_uri(
        &self,
        input: SetNextAVTransportURIInput,
    ) -> Result<SetNextAVTransportURIOutput, Self::Error>;
}

/// Record — Begin recording media.
pub trait Record: Send + Sync {
    type Error: Display;
    fn record(&self, input: StopInput) -> Result<(), Self::Error>;
}

/// SetPlayMode — Set shuffle/repeat behavior.
pub trait SetPlayMode: Send + Sync {
    type Error: Display;
    fn set_play_mode(&self, input: SetPlayModeInput) -> Result<SetPlayModeOutput, Self::Error>;
}

/// GetCurrentTransportActions — Return actions currently valid for this instance.
pub trait GetCurrentTransportActions: Send + Sync {
    type Error: Display;
    fn get_current_transport_actions(
        &self,
        input: GetCurrentTransportActionsInput,
    ) -> Result<GetCurrentTransportActionsOutput, Self::Error>;
}

// ===========================================================================
// Conditionally Required / Allowed - Deferred
// ===========================================================================

/// SetRecordQualityMode — Set recording quality mode.
pub trait SetRecordQualityMode: Send + Sync {
    type Error: Display;
    fn set_record_quality_mode(
        &self,
        input: SetRecordQualityModeInput,
    ) -> Result<SetRecordQualityModeOutput, Self::Error>;
}

/// GetDRMState — Get current DRM state.
pub trait GetDRMState: Send + Sync {
    type Error: Display;
    fn get_drm_state(&self) -> Result<GetDRMStateOutput, Self::Error>;
}

/// GetStateVariables — Get values for specified state variables.
pub trait GetStateVariables: Send + Sync {
    type Error: Display;
    fn get_state_variables(
        &self,
        input: GetStateVariablesInput,
    ) -> Result<GetStateVariablesOutput, Self::Error>;
}

/// SetStateVariables — Set values for specified state variables.
pub trait SetStateVariables: Send + Sync {
    type Error: Display;
    fn set_state_variables(
        &self,
        input: SetStateVariablesInput,
    ) -> Result<SetStateVariablesOutput, Self::Error>;
}

/// GetSyncOffset — Get current sync offset.
pub trait GetSyncOffset: Send + Sync {
    type Error: Display;
    fn get_sync_offset(&self) -> Result<GetSyncOffsetOutput, Self::Error>;
}

/// SetSyncOffset — Set sync offset.
pub trait SetSyncOffset: Send + Sync {
    type Error: Display;
    fn set_sync_offset(
        &self,
        input: SetSyncOffsetInput,
    ) -> Result<SetSyncOffsetOutput, Self::Error>;
}

/// AdjustSyncOffset — Adjust sync offset by delta.
pub trait AdjustSyncOffset: Send + Sync {
    type Error: Display;
    fn adjust_sync_offset(
        &self,
        input: AdjustSyncOffsetInput,
    ) -> Result<AdjustSyncOffsetOutput, Self::Error>;
}

/// SyncPlay — Synchronize playback across multiple renderers.
pub trait SyncPlay: Send + Sync {
    type Error: Display;
    fn sync_play(&self) -> Result<SyncPlayOutput, Self::Error>;
}

/// SyncStop — Stop synchronized playback.
pub trait SyncStop: Send + Sync {
    type Error: Display;
    fn sync_stop(&self) -> Result<SyncStopOutput, Self::Error>;
}

/// SyncPause — Pause synchronized playback.
pub trait SyncPause: Send + Sync {
    type Error: Display;
    fn sync_pause(&self) -> Result<SyncPauseOutput, Self::Error>;
}

/// SetStaticPlaylist — Set a static playlist.
pub trait SetStaticPlaylist: Send + Sync {
    type Error: Display;
    fn set_static_playlist(
        &self,
        input: SetStaticPlaylistInput,
    ) -> Result<SetStaticPlaylistOutput, Self::Error>;
}

/// SetStreamingPlaylist — Set a streaming playlist.
pub trait SetStreamingPlaylist: Send + Sync {
    type Error: Display;
    fn set_streaming_playlist(
        &self,
        input: SetStreamingPlaylistInput,
    ) -> Result<SetStreamingPlaylistOutput, Self::Error>;
}

/// GetPlaylistInfo — Get information about the current playlist.
pub trait GetPlaylistInfo: Send + Sync {
    type Error: Display;
    fn get_playlist_info(
        &self,
        input: GetMediaInfoInput,
    ) -> Result<GetPlaylistInfoOutput, Self::Error>;
}

/// GetMediaInfoExt — Extended media information.
pub trait GetMediaInfoExt: Send + Sync {
    type Error: Display;
    fn get_media_info_ext(
        &self,
        input: GetMediaInfoInput,
    ) -> Result<GetMediaInfoExtOutput, Self::Error>;
}
