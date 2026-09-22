/// ConnectionManager service — bridge from SOAP `Action` trait to application traits.
///
/// Each struct wraps an Arc<T> where T implements the application trait.
/// The `execute` method extracts typed args, calls the trait, and wraps results.
///
/// Bridge structs are prefixed with `Action` to avoid name collision with traits.
use std::sync::Arc;

use super::r#static::Direction;
use crate::services::connectionmanager::traits::*;
use crate::types::upnp::{Action, ActionArgs, ArgumentDefinition, ArgumentDirection, Error};

// ===========================================================================
// Required Actions (R)
// ===========================================================================

pub struct ActionGetProtocolInfo<T: GetProtocolInfo> {
    trait_impl: Arc<T>,
}

impl<T: GetProtocolInfo> ActionGetProtocolInfo<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetProtocolInfo> Action for ActionGetProtocolInfo<T> {
    fn name(&self) -> &'static str {
        "GetProtocolInfo"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "Source",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("SourceProtocolInfo"),
            },
            ArgumentDefinition {
                name: "Sink",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("SinkProtocolInfo"),
            },
        ];
        &ARGS
    }

    fn execute(&self, _args: &ActionArgs) -> Result<ActionArgs, Error> {
        let output = self
            .trait_impl
            .get_protocol_info()
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set("Source".to_string(), output.source);
        out.set("Sink".to_string(), output.sink);
        Ok(out)
    }
}

pub struct ActionGetCurrentConnectionIDs<T: GetCurrentConnectionIDs> {
    trait_impl: Arc<T>,
}

impl<T: GetCurrentConnectionIDs> ActionGetCurrentConnectionIDs<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetCurrentConnectionIDs> Action for ActionGetCurrentConnectionIDs<T> {
    fn name(&self) -> &'static str {
        "GetCurrentConnectionIDs"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "ConnectionIDs",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("CurrentConnectionIDs"),
        }];
        &ARGS
    }

    fn execute(&self, _args: &ActionArgs) -> Result<ActionArgs, Error> {
        let output = self
            .trait_impl
            .get_current_connection_ids()
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set("ConnectionIDs".to_string(), output.connection_ids);
        Ok(out)
    }
}

pub struct ActionGetCurrentConnectionInfo<T: GetCurrentConnectionInfo> {
    trait_impl: Arc<T>,
}

impl<T: GetCurrentConnectionInfo> ActionGetCurrentConnectionInfo<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetCurrentConnectionInfo> Action for ActionGetCurrentConnectionInfo<T> {
    fn name(&self) -> &'static str {
        "GetCurrentConnectionInfo"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "ConnectionID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_ConnectionID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "RcsID",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_RcsID"),
            },
            ArgumentDefinition {
                name: "AVTransportID",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_AVTransportID"),
            },
            ArgumentDefinition {
                name: "ProtocolInfo",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_ProtocolInfo"),
            },
            ArgumentDefinition {
                name: "PeerConnectionManager",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_ConnectionManager"),
            },
            ArgumentDefinition {
                name: "PeerConnectionID",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_ConnectionID"),
            },
            ArgumentDefinition {
                name: "Direction",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_Direction"),
            },
            ArgumentDefinition {
                name: "Status",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_ConnectionStatus"),
            },
        ];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let connection_id = args
            .get("ConnectionID")
            .ok_or(Error::ArgumentValueInvalid)?
            .parse::<i32>()
            .map_err(|_| Error::ArgumentValueInvalid)?;
        let input = GetCurrentConnectionInfoInput { connection_id };
        let output = self
            .trait_impl
            .get_current_connection_info(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set("RcsID".to_string(), output.rcs_id.to_string());
        out.set(
            "AVTransportID".to_string(),
            output.av_transport_id.to_string(),
        );
        out.set("ProtocolInfo".to_string(), output.protocol_info);
        out.set(
            "PeerConnectionManager".to_string(),
            output.peer_connection_manager,
        );
        out.set(
            "PeerConnectionID".to_string(),
            output.peer_connection_id.to_string(),
        );
        out.set(
            "Direction".to_string(),
            output.direction.as_str().to_string(),
        );
        out.set("Status".to_string(), output.status.as_str().to_string());
        Ok(out)
    }
}

pub struct ActionGetFeatureList<T: GetFeatureList> {
    trait_impl: Arc<T>,
}

impl<T: GetFeatureList> ActionGetFeatureList<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetFeatureList> Action for ActionGetFeatureList<T> {
    fn name(&self) -> &'static str {
        "GetFeatureList"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "FeatureList",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("FeatureList"),
        }];
        &ARGS
    }

    fn execute(&self, _args: &ActionArgs) -> Result<ActionArgs, Error> {
        let output = self
            .trait_impl
            .get_feature_list()
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set("FeatureList".to_string(), output.feature_list);
        Ok(out)
    }
}

// ===========================================================================
// Allowed Actions (A)
// ===========================================================================

pub struct ActionPrepareForConnection<T: PrepareForConnection> {
    trait_impl: Arc<T>,
    /// Reference to connection table for registering new connections.
    connection_table: Arc<
        std::sync::Mutex<
            std::collections::HashMap<i32, crate::services::connectionmanager::ConnectionInfo>,
        >,
    >,
}

impl<T: PrepareForConnection> ActionPrepareForConnection<T> {
    pub fn new(
        trait_impl: T,
        connection_table: Arc<
            std::sync::Mutex<
                std::collections::HashMap<i32, crate::services::connectionmanager::ConnectionInfo>,
            >,
        >,
    ) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            connection_table,
        }
    }
}

impl<T: PrepareForConnection> Action for ActionPrepareForConnection<T> {
    fn name(&self) -> &'static str {
        "PrepareForConnection"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "RemoteProtocolInfo",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_ProtocolInfo"),
            },
            ArgumentDefinition {
                name: "PeerConnectionManager",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_ConnectionManager"),
            },
            ArgumentDefinition {
                name: "PeerConnectionID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_ConnectionID"),
            },
            ArgumentDefinition {
                name: "Direction",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_Direction"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "ConnectionID",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_ConnectionID"),
            },
            ArgumentDefinition {
                name: "AVTransportID",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_AVTransportID"),
            },
            ArgumentDefinition {
                name: "RcsID",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_RcsID"),
            },
        ];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let remote_protocol_info = args
            .get("RemoteProtocolInfo")
            .ok_or(Error::ArgumentValueInvalid)?
            .to_string();
        let peer_connection_manager = args
            .get("PeerConnectionManager")
            .unwrap_or("-1")
            .to_string();
        let peer_connection_id = args
            .get("PeerConnectionID")
            .unwrap_or("-1")
            .parse::<i32>()
            .unwrap_or(-1);
        let direction_str = args.get("Direction").ok_or(Error::ArgumentValueInvalid)?;
        let direction = Direction::from_str(direction_str).ok_or(Error::ArgumentValueInvalid)?;
        let input = PrepareForConnectionInput {
            remote_protocol_info,
            peer_connection_manager,
            peer_connection_id,
            direction,
        };
        let output = self
            .trait_impl
            .prepare_for_connection(input.clone())
            .map_err(|e| {
                // Map connection manager errors to UPnP error codes
                match e.to_string().as_str() {
                    "702 Incompatible Directions" => {
                        Error::from_code(702).unwrap_or(Error::ActionFailed)
                    }
                    "708 Connection Table overflow" => {
                        Error::from_code(708).unwrap_or(Error::ActionFailed)
                    }
                    _ => Error::ActionFailed,
                }
            })?;

        // Register connection in library's connection table
        {
            let mut table = self.connection_table.lock().unwrap();
            table.insert(
                output.connection_id,
                crate::services::connectionmanager::ConnectionInfo {
                    protocol_info: output.av_transport_id.to_string(), // Placeholder — app bridge sets this
                    direction: input.direction,
                    peer_connection_manager: input.peer_connection_manager.clone(),
                    peer_connection_id: input.peer_connection_id,
                },
            );
        }

        let mut out = ActionArgs::new();
        out.set("ConnectionID".to_string(), output.connection_id.to_string());
        out.set(
            "AVTransportID".to_string(),
            output.av_transport_id.to_string(),
        );
        out.set("RcsID".to_string(), output.rcs_id.to_string());
        Ok(out)
    }
}

pub struct ActionConnectionComplete<T: ConnectionComplete> {
    trait_impl: Arc<T>,
    /// Reference to connection table for removing connections.
    connection_table: Arc<
        std::sync::Mutex<
            std::collections::HashMap<i32, crate::services::connectionmanager::ConnectionInfo>,
        >,
    >,
}

impl<T: ConnectionComplete> ActionConnectionComplete<T> {
    pub fn new(
        trait_impl: T,
        connection_table: Arc<
            std::sync::Mutex<
                std::collections::HashMap<i32, crate::services::connectionmanager::ConnectionInfo>,
            >,
        >,
    ) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            connection_table,
        }
    }
}

impl<T: ConnectionComplete> Action for ActionConnectionComplete<T> {
    fn name(&self) -> &'static str {
        "ConnectionComplete"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "ConnectionID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_ConnectionID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let connection_id = args
            .get("ConnectionID")
            .ok_or(Error::ArgumentValueInvalid)?
            .parse::<i32>()
            .map_err(|_| Error::ArgumentValueInvalid)?;

        // Validate and remove from library's connection table
        {
            let mut table = self.connection_table.lock().unwrap();
            if !table.contains_key(&connection_id) {
                return Err(Error::from_code(706).unwrap_or(Error::ActionFailed));
            }
            table.remove(&connection_id);
        }

        let input = ConnectionCompleteInput { connection_id };
        self.trait_impl
            .connection_complete(input)
            .map_err(|_| Error::ActionFailed)?;
        Ok(ActionArgs::new())
    }
}

pub struct ActionGetRendererItemInfo<T: GetRendererItemInfo> {
    trait_impl: Arc<T>,
}

impl<T: GetRendererItemInfo> ActionGetRendererItemInfo<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetRendererItemInfo> Action for ActionGetRendererItemInfo<T> {
    fn name(&self) -> &'static str {
        "GetRendererItemInfo"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "ItemInfoFilter",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_ItemInfoFilter"),
            },
            ArgumentDefinition {
                name: "ItemMetadataList",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_Result"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "ItemRenderingInfoList",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("A_ARG_TYPE_RenderingInfoList"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let item_info_filter = args.get("ItemInfoFilter").unwrap_or("*").to_string();
        let item_metadata_list = args
            .get("ItemMetadataList")
            .ok_or(Error::ArgumentValueInvalid)?
            .to_string();
        let input = GetRendererItemInfoInput {
            item_info_filter,
            item_metadata_list,
        };
        let output = self
            .trait_impl
            .get_renderer_item_info(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set(
            "ItemRenderingInfoList".to_string(),
            output.item_rendering_info_list,
        );
        Ok(out)
    }
}
