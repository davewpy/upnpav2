pub mod actions;
pub mod r#static;
pub mod traits;

pub use r#static::{ActionName, ArgumentName, Error};
pub use traits::*;

use self::r#static::V3;
use crate::gena::EventPublisher;

use crate::state::StateStore;
use crate::types::upnp::{DataType, Services};
use crate::types::{Action, ActionMap};
use std::sync::Arc;

/// RenderingControl service — owns action registry, state variables, and eventing.
///
/// The application registers trait implementations (GetVolumeAction, SetMuteAction, etc.)
/// via `register_action()`. The SOAP handler dispatches incoming requests
/// to the correct trait implementation.
///
/// State variables are initialized in `init_state_vars()` and owned by this struct.
/// When state changes via `set_state_var()`, the service automatically triggers GENA events.
#[derive(Clone)]
pub struct RenderingControlService {
    actions: Arc<std::sync::Mutex<ActionMap>>,
    state_store: Arc<std::sync::Mutex<StateStore<r#static::StateVariableName>>>,
    event_publisher: Arc<std::sync::Mutex<EventPublisher>>,
}

/// Shared event publisher type alias for convenience.
pub type RenderingControlEventPublisher = Arc<std::sync::Mutex<EventPublisher>>;

impl RenderingControlService {
    /// Create a new empty RenderingControl service with all state variables initialized.
    pub fn new() -> Self {
        Self::with_event_publisher(EventPublisher::new(
            Services::RenderingControl(V3).event_url(),
            false,
        ))
    }

    /// Create a new RenderingControl service with a custom event publisher.
    pub fn with_event_publisher(publisher: EventPublisher) -> Self {
        let mut state_store = StateStore::new();
        Self::init_state_vars(&mut state_store);
        Self {
            actions: Arc::new(std::sync::Mutex::new(ActionMap::new(
                Services::RenderingControl(V3),
            ))),
            state_store: Arc::new(std::sync::Mutex::new(state_store)),
            event_publisher: Arc::new(std::sync::Mutex::new(publisher)),
        }
    }

    /// Get a clone of the shared event publisher.
    pub fn event_publisher(&self) -> RenderingControlEventPublisher {
        Arc::clone(&self.event_publisher)
    }

    /// Get an Arc-wrapped reference to the state store for action construction.
    pub fn state_store(&self) -> Arc<std::sync::Mutex<StateStore<r#static::StateVariableName>>> {
        Arc::clone(&self.state_store)
    }

    /// Initialize all RenderingControl state variables per UPnP-av-RenderingControl-v3 spec §2.1.
    fn init_state_vars(state_store: &mut StateStore<r#static::StateVariableName>) {
        state_store.register_batch(r#static::STATE_VARIABLE_SCHEMAS.to_vec());
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
    ) -> Result<crate::types::upnp::DataType, crate::types::upnp::Error> {
        let store = self.state_store.lock().unwrap();
        store.get_owned(name)
    }

    /// Get a clone of a state variable by name.
    pub fn get_state_var_mut(
        &mut self,
        name: r#static::StateVariableName,
    ) -> Option<crate::types::statevariable::StateVariableType> {
        let store = self.state_store.lock().unwrap();
        store.get_mut_owned(name)
    }

    /// Set any state variable at a specific InstanceID.
    ///
    /// This is the single gatekeeper method for all state mutations. It:
    /// 1. Sets the value in StateStore (uses set_instance if var is instance-scoped)
    /// 2. Triggers GENA events via LastChange NOTIFY
    ///
    /// Per UPnP-av-2.0 spec, InstanceID=0 is global/post-mix, InstanceID>0 is per-stream.
    /// RenderingControl state variables are all per-InstanceID (is_instance_scoped=true).
    ///
    /// Writing the same value does NOT set has_changes (per UPnP spec).
    pub fn set_state_var(
        &self,
        instance_id: u32,
        name: r#static::StateVariableName,
        value: DataType,
    ) {
        let mut store = self.state_store.lock().unwrap();

        // Look up schema to determine if this var is instance-scoped
        let key = format!("{}", name);
        let is_instance_scoped = store
            .schema(&key)
            .map(|s| s.is_instance_scoped)
            .unwrap_or(true);

        if is_instance_scoped {
            store.set_instance(name, instance_id, value);
        } else {
            store.set(name, value);
        }
        drop(store);

        self.trigger_events_for_instance(instance_id);
    }

    /// Build the LastChange XML propertyset from all evented state variables for an instance.
    fn build_last_change(&self, instance_id: u32) -> String {
        let store = self.state_store.lock().unwrap();
        let evented = store.collect_evented_for_instance(instance_id);
        crate::services::lastchange::build_last_change(
            "urn:schemas-upnp-org:metadata-1-0/RC/",
            &evented,
        )
    }

    /// Trigger GENA events for all evented state variables at a specific InstanceID.
    fn trigger_events_for_instance(&self, instance_id: u32) {
        let last_change_xml = self.build_last_change(instance_id);

        // Update LastChange state variable (global, not per-instance)
        {
            let mut store = self.state_store.lock().unwrap();
            if let Some(last_change_var) = store.get_mut(r#static::StateVariableName::LastChange) {
                last_change_var.current_value = DataType::String(last_change_xml.clone());
            }
        }

        // Notify all subscribers with LastChange property
        let properties = vec![("LastChange".to_string(), last_change_xml)];
        let mut publisher = self.event_publisher.lock().unwrap();
        let _results = publisher.notify(&properties);
    }

    /// Get all state variable names (for SCPD generation).
    pub fn state_var_names(&self) -> Vec<String> {
        let store = self.state_store.lock().unwrap();
        store.names()
    }
}
