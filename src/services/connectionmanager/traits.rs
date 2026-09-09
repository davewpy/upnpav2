/// ConnectionManager action traits — application-facing API.
///
/// Each trait represents one UPnP ConnectionManager action with typed parameters.
/// The application implements these traits to provide actual behavior.
/// No UPnP protocol knowledge required.
use std::fmt::Display;

use super::r#static::{ConnectionStatus, Direction};

// ===========================================================================
// Input Types
// ===========================================================================

#[derive(Debug, Clone)]
pub struct GetCurrentConnectionInfoInput {
    pub connection_id: i32,
}

#[derive(Debug, Clone)]
pub struct PrepareForConnectionInput {
    pub remote_protocol_info: String,
    pub peer_connection_manager: String,
    pub peer_connection_id: i32,
    pub direction: Direction,
}

#[derive(Debug, Clone)]
pub struct ConnectionCompleteInput {
    pub connection_id: i32,
}

#[derive(Debug, Clone)]
pub struct GetRendererItemInfoInput {
    pub item_info_filter: String,
    pub item_metadata_list: String,
}

// ===========================================================================
// Output Types
// ===========================================================================

#[derive(Debug, Clone)]
pub struct GetProtocolInfoOutput {
    pub source: String,
    pub sink: String,
}

#[derive(Debug, Clone)]
pub struct PrepareForConnectionOutput {
    pub connection_id: i32,
    pub av_transport_id: i32,
    pub rcs_id: i32,
}

#[derive(Debug, Clone, Default)]
pub struct ConnectionCompleteOutput {}

#[derive(Debug, Clone)]
pub struct GetCurrentConnectionIDsOutput {
    pub connection_ids: String,
}

#[derive(Debug, Clone)]
pub struct GetCurrentConnectionInfoOutput {
    pub rcs_id: i32,
    pub av_transport_id: i32,
    pub protocol_info: String,
    pub peer_connection_manager: String,
    pub peer_connection_id: i32,
    pub direction: Direction,
    pub status: ConnectionStatus,
}

#[derive(Debug, Clone)]
pub struct GetRendererItemInfoOutput {
    pub item_rendering_info_list: String,
}

#[derive(Debug, Clone)]
pub struct GetFeatureListOutput {
    pub feature_list: String,
}

// ===========================================================================
// Required Actions (R)
// ===========================================================================

/// GetProtocolInfo — Return Source/Sink protocol info CSV lists.
pub trait GetProtocolInfo: Send + Sync {
    type Error: Display;
    fn get_protocol_info(&self) -> Result<GetProtocolInfoOutput, Self::Error>;
}

/// GetCurrentConnectionIDs — Return CSV of active ConnectionIDs.
pub trait GetCurrentConnectionIDs: Send + Sync {
    type Error: Display;
    fn get_current_connection_ids(&self) -> Result<GetCurrentConnectionIDsOutput, Self::Error>;
}

/// GetCurrentConnectionInfo — Return full details for a single connection.
pub trait GetCurrentConnectionInfo: Send + Sync {
    type Error: Display;
    fn get_current_connection_info(
        &self,
        input: GetCurrentConnectionInfoInput,
    ) -> Result<GetCurrentConnectionInfoOutput, Self::Error>;
}

/// GetFeatureList — Return Features XML Document.
pub trait GetFeatureList: Send + Sync {
    type Error: Display;
    fn get_feature_list(&self) -> Result<GetFeatureListOutput, Self::Error>;
}

// ===========================================================================
// Allowed Actions (A)
// ===========================================================================

/// PrepareForConnection — Allocate connection, return ConnectionID.
///
/// Direction: "Input" or "Output"
pub trait PrepareForConnection: Send + Sync {
    type Error: Display;
    fn prepare_for_connection(
        &self,
        input: PrepareForConnectionInput,
    ) -> Result<PrepareForConnectionOutput, Self::Error>;
}

/// ConnectionComplete — Tear down connection, release resources.
pub trait ConnectionComplete: Send + Sync {
    type Error: Display;
    fn connection_complete(
        &self,
        input: ConnectionCompleteInput,
    ) -> Result<ConnectionCompleteOutput, Self::Error>;
}

/// GetRendererItemInfo — Inspect DIDL-Lite items, return playback capability info.
pub trait GetRendererItemInfo: Send + Sync {
    type Error: Display;
    fn get_renderer_item_info(
        &self,
        input: GetRendererItemInfoInput,
    ) -> Result<GetRendererItemInfoOutput, Self::Error>;
}
