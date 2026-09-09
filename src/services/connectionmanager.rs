pub mod actions;
pub mod r#static;
pub mod traits;

pub use traits::*;

use crate::gena::EventPublisher;
use crate::{
    connectionmanager::r#static::V3,
    types::upnp::{Action, ActionMap, Services, StateSchema, StateStore, StateValue},
};
use std::sync::Arc;

/// ConnectionManager service — owns action registry, state variables, and eventing.
///
/// The application registers trait implementations (GetProtocolInfoAction, PrepareForConnectionAction, etc.)
/// via `register_action()`. The SOAP handler dispatches incoming requests
/// to the correct trait implementation.
///
/// State variables are initialized in `init_state_vars()` and owned by this struct.
/// When state changes via `set_state_var()`, the service automatically triggers GENA events.
pub struct ConnectionManagerService {
    actions: Arc<std::sync::Mutex<ActionMap>>,
    state_store: Arc<std::sync::Mutex<StateStore<r#static::StateVariableName>>>,
    event_publisher: Arc<std::sync::Mutex<EventPublisher>>,
}

/// Shared event publisher type alias for convenience.
pub type ConnectionManagerEventPublisher = Arc<std::sync::Mutex<EventPublisher>>;

impl ConnectionManagerService {
    /// Create a new empty ConnectionManager service with all state variables initialized.
    pub fn new() -> Self {
        Self::with_event_publisher(EventPublisher::new(
            Services::ConnectionManager(V3).event_url(),
            false,
        ))
    }

    /// Create a new ConnectionManager service with a custom event publisher.
    pub fn with_event_publisher(publisher: EventPublisher) -> Self {
        let mut state_store = StateStore::new();
        Self::init_state_vars(&mut state_store);
        Self {
            actions: Arc::new(std::sync::Mutex::new(ActionMap::new(Services::ConnectionManager(V3)))),
            state_store: Arc::new(std::sync::Mutex::new(state_store)),
            event_publisher: Arc::new(std::sync::Mutex::new(publisher)),
        }
    }

    /// Get a clone of the shared event publisher.
    pub fn event_publisher(&self) -> ConnectionManagerEventPublisher {
        Arc::clone(&self.event_publisher)
    }

    /// Get an Arc-wrapped reference to the state store for action construction.
    pub fn state_store(&self) -> Arc<std::sync::Mutex<StateStore<r#static::StateVariableName>>> {
        Arc::clone(&self.state_store)
    }

    /// Initialize all ConnectionManager state variables per UPnP-av-ConnectionManager-v3 spec §4.2.
    fn init_state_vars(state_store: &mut StateStore<r#static::StateVariableName>) {
        use r#static::StateVariableName;

        // SourceProtocolInfo — CSV of protocol info entries
        state_store.register(StateSchema {
            name: StateVariableName::SourceProtocolInfo,
            data_type: crate::types::upnp::DataType::String,
            send_events: true,
            default: StateValue::String(String::new()),
            ..Default::default()
        });

        // SinkProtocolInfo — CSV of protocol info entries
        state_store.register(StateSchema {
            name: StateVariableName::SinkProtocolInfo,
            data_type: crate::types::upnp::DataType::String,
            send_events: true,
            default: StateValue::String(String::new()),
            ..Default::default()
        });

        // CurrentConnectionIDs — CSV of active ConnectionID values
        state_store.register(StateSchema {
            name: StateVariableName::CurrentConnectionIDs,
            data_type: crate::types::upnp::DataType::String,
            send_events: true,
            default: StateValue::String("0".to_string()),
            ..Default::default()
        });

        // FeatureList — Features XML Document
        state_store.register(StateSchema {
            name: StateVariableName::FeatureList,
            data_type: crate::types::upnp::DataType::String,
            send_events: false,
            default: StateValue::String(String::new()),
            ..Default::default()
        });

        // ClockUpdateID — ui4, monotonic counter
        state_store.register(StateSchema {
            name: StateVariableName::ClockUpdateID,
            data_type: crate::types::upnp::DataType::UnsignedInt,
            send_events: false,
            default: StateValue::Ui4(0),
            ..Default::default()
        });

        // DeviceClockInfoUpdates — XML document
        state_store.register(StateSchema {
            name: StateVariableName::DeviceClockInfoUpdates,
            data_type: crate::types::upnp::DataType::String,
            send_events: true,
            default: StateValue::String(String::new()),
            ..Default::default()
        });

        // =========================================================================
        // A_ARG_TYPE variables — type definitions for action arguments
        // =========================================================================

        // A_ARG_TYPE_ConnectionStatus — allowed values per spec §4.2
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_ConnectionStatus,
            data_type: crate::types::upnp::DataType::String,
            send_events: false,
            default: StateValue::String(String::new()),
            allowed_values: Some(vec![
                "OK".to_string(),
                "ContentFormatMismatch".to_string(),
                "InsufficientBandwidth".to_string(),
                "UnreliableChannel".to_string(),
                "Unknown".to_string(),
            ]),
            argument_type: true,
            ..Default::default()
        });

        // A_ARG_TYPE_ConnectionManager — UDN/serviceId reference
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_ConnectionManager,
            data_type: crate::types::upnp::DataType::String,
            send_events: false,
            default: StateValue::String(String::new()),
            argument_type: true,
            ..Default::default()
        });

        // A_ARG_TYPE_Direction — Input or Output
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_Direction,
            data_type: crate::types::upnp::DataType::String,
            send_events: false,
            default: StateValue::String(String::new()),
            allowed_values: Some(vec!["Input".to_string(), "Output".to_string()]),
            argument_type: true,
            ..Default::default()
        });

        // A_ARG_TYPE_ProtocolInfo — protocol info string format
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_ProtocolInfo,
            data_type: crate::types::upnp::DataType::String,
            send_events: false,
            default: StateValue::String(String::new()),
            argument_type: true,
            ..Default::default()
        });

        // A_ARG_TYPE_ConnectionID — i4, connection identifier
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_ConnectionID,
            data_type: crate::types::upnp::DataType::Int,
            send_events: false,
            default: StateValue::I4(-1),
            argument_type: true,
            ..Default::default()
        });

        // A_ARG_TYPE_AVTransportID — i4, AVTransport instance ID
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_AVTransportID,
            data_type: crate::types::upnp::DataType::Int,
            send_events: false,
            default: StateValue::I4(-1),
            argument_type: true,
            ..Default::default()
        });

        // A_ARG_TYPE_RcsID — i4, RenderingControl instance ID
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_RcsID,
            data_type: crate::types::upnp::DataType::Int,
            send_events: false,
            default: StateValue::I4(-1),
            argument_type: true,
            ..Default::default()
        });

        // A_ARG_TYPE_ItemInfoFilter — CSV of property specifiers
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_ItemInfoFilter,
            data_type: crate::types::upnp::DataType::String,
            send_events: false,
            default: StateValue::String(String::new()),
            argument_type: true,
            ..Default::default()
        });

        // A_ARG_TYPE_Result — DIDL-Lite XML document
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_Result,
            data_type: crate::types::upnp::DataType::String,
            send_events: false,
            default: StateValue::String(String::new()),
            argument_type: true,
            ..Default::default()
        });

        // A_ARG_TYPE_RenderingInfoList — XML rendering info
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_RenderingInfoList,
            data_type: crate::types::upnp::DataType::String,
            send_events: false,
            default: StateValue::String(String::new()),
            argument_type: true,
            ..Default::default()
        });
    }

    /// Register a trait implementation for the given action name.
    pub fn register_action(&mut self, action: Box<dyn Action>) {
        self.actions.lock().unwrap().register(action);
    }

    /// Get the action map for SOAP dispatch.
    pub fn actions(&self) -> Arc<std::sync::Mutex<ActionMap>> {
        Arc::clone(&self.actions)
    }

    /// Get a state variable by name.
    pub fn get_state_var(
        &self,
        name: r#static::StateVariableName,
    ) -> Result<crate::types::upnp::StateValue, crate::types::upnp::Error> {
        let store = self.state_store.lock().unwrap();
        store.get_owned(name)
    }

    /// Get a clone of a state variable by name.
    pub fn get_state_var_mut(
        &mut self,
        name: r#static::StateVariableName,
    ) -> Option<crate::types::upnp::StateVariable> {
        let store = self.state_store.lock().unwrap();
        store.get_mut_owned(name)
    }

    /// Set a state variable value. Returns error if validation fails.
    ///
    /// If the value actually changed, automatically triggers GENA events for all evented state variables.
    pub fn set_state_var(
        &mut self,
        name: r#static::StateVariableName,
        value: String,
    ) -> Result<(), crate::types::upnp::Error> {
        {
            let mut store = self.state_store.lock().unwrap();
            store.set(name, StateValue::String(value))?;
        }
        self.trigger_events();
        Ok(())
    }

    /// Build the LastChange XML propertyset from all evented state variables.
    fn build_last_change(&self) -> String {
        let store = self.state_store.lock().unwrap();
        let evented = store.collect_evented();
        crate::services::lastchange::build_last_change(
            "urn:schemas-upnp-org:metadata-1-0/CM/",
            &evented,
        )
    }

    /// Trigger GENA events for all evented state variables.
    fn trigger_events(&mut self) {
        let last_change_xml = self.build_last_change();

        // Update LastChange state variable (if it exists)
        // Note: ConnectionManager doesn't have a LastChange var per spec,
        // but we keep the pattern for consistency
        let _ = last_change_xml;

        let store = self.state_store.lock().unwrap();
        let evented = store.collect_evented();
        let mut publisher = self.event_publisher.lock().unwrap();
        let _results = publisher.notify(&evented);
    }

    /// Get all state variable names (for SCPD generation).
    pub fn state_var_names(&self) -> Vec<String> {
        let store = self.state_store.lock().unwrap();
        store.names()
    }
}
