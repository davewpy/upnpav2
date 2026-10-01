pub mod actions;
pub mod r#static;
pub mod traits;

pub use traits::*;

use self::r#static::V3;
use crate::gena::EventPublisher;
use crate::state::StateStore;
use crate::types::upnp::{DataType, Services};
use crate::types::{Action, ActionMap};
use std::collections::HashMap;
use std::sync::Arc;

/// Connection table entry — tracks AVTransportID/RcsID bindings per spec §4.2.
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    /// Protocol info for the stream (from RemoteProtocolInfo input).
    pub protocol_info: String,
    /// Direction: Input or Output.
    pub direction: r#static::Direction,
    /// Peer ConnectionManager reference (UDN/serviceId).
    pub peer_connection_manager: String,
    /// Peer's ConnectionID (-1 if unknown).
    pub peer_connection_id: i32,
}

/// ConnectionManager service — owns action registry, state variables, and eventing.
///
/// The application registers trait implementations (GetProtocolInfoAction, PrepareForConnectionAction, etc.)
/// via `register_action()`. The SOAP handler dispatches incoming requests
/// to the correct trait implementation.
///
/// State variables are initialized in `init_state_vars()` and owned by this struct.
/// When state changes via `set_state_var()`, the service automatically triggers GENA events.
///
/// Connection tracking (per UPnP-av-ConnectionManager-v3 spec §4.2):
/// - PrepareForConnection() allocates ConnectionID and binds AVTransportID/RcsID
/// - CurrentConnectionIDs state variable tracks active connections
/// - ConnectionComplete() removes the connection from the table
#[derive(Clone)]
pub struct ConnectionManagerService {
    actions: Arc<std::sync::Mutex<ActionMap>>,
    state_store: Arc<std::sync::Mutex<StateStore<r#static::StateVariableName>>>,
    event_publisher: Arc<std::sync::Mutex<EventPublisher>>,
    /// Connection table: ConnectionID → (AVTransportID, RcsID, metadata).
    connection_table: Arc<std::sync::Mutex<HashMap<i32, ConnectionInfo>>>,
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

        // Per UPnP AV spec §5.4.3: ConnectionManager always has a default implicit connection
        // (ConnectionID=0, AVTransportID=0, RcsID=0) that exists without PrepareForConnection().
        let mut connection_table = HashMap::new();
        connection_table.insert(
            0,
            ConnectionInfo {
                protocol_info: String::new(),
                direction: r#static::Direction::Output,
                peer_connection_manager: String::new(),
                peer_connection_id: -1,
            },
        );

        Self {
            actions: Arc::new(std::sync::Mutex::new(ActionMap::new(
                Services::ConnectionManager(V3),
            ))),
            state_store: Arc::new(std::sync::Mutex::new(state_store)),
            event_publisher: Arc::new(std::sync::Mutex::new(publisher)),
            connection_table: Arc::new(std::sync::Mutex::new(connection_table)),
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

    // ===========================================================================
    // Connection Table Management (per UPnP-av-ConnectionManager-v3 spec §4.2)
    // ===========================================================================

    /// Register a new connection in the table and update CurrentConnectionIDs state variable.
    ///
    /// Returns error 708 if the connection table is full (single-connection device).
    pub fn register_connection(
        &self,
        connection_id: i32,
        info: ConnectionInfo,
    ) -> Result<(), r#static::Error> {
        let mut table = self.connection_table.lock().unwrap();

        // Check for existing connection — single connection device per spec §4.2
        if !table.is_empty() {
            return Err(r#static::Error::ConnectionTableOverflow);
        }

        table.insert(connection_id, info);

        // Update CurrentConnectionIDs state variable (CSV of active IDs)
        let mut store = self.state_store.lock().unwrap();
        let _ = store.set(
            r#static::StateVariableName::CurrentConnectionIDs,
            DataType::String(connection_id.to_string()),
        );

        Ok(())
    }

    /// Remove a connection from the table and update CurrentConnectionIDs state variable.
    ///
    /// Returns error 706 if the connection ID is not found.
    pub fn remove_connection(&self, connection_id: i32) -> Result<(), r#static::Error> {
        let mut table = self.connection_table.lock().unwrap();

        // Validate connection exists before removing
        if !table.contains_key(&connection_id) {
            return Err(r#static::Error::InvalidConnectionReference);
        }

        table.remove(&connection_id);

        // Update CurrentConnectionIDs state variable (CSV of remaining IDs)
        let mut store = self.state_store.lock().unwrap();
        let remaining_ids: Vec<String> = table.keys().map(|id| id.to_string()).collect();
        let _ = store.set(
            r#static::StateVariableName::CurrentConnectionIDs,
            DataType::String(remaining_ids.join(",")),
        );

        Ok(())
    }

    /// Get the connection info for a given connection ID.
    ///
    /// Returns error 706 if the connection ID is not found.
    pub fn get_connection_info(
        &self,
        connection_id: i32,
    ) -> Result<ConnectionInfo, r#static::Error> {
        self.connection_table
            .lock()
            .unwrap()
            .get(&connection_id)
            .cloned()
            .ok_or(r#static::Error::InvalidConnectionReference)
    }

    /// Get the current connection IDs as a CSV string.
    pub fn get_current_connection_ids(&self) -> String {
        let table = self.connection_table.lock().unwrap();
        let ids: Vec<String> = table.keys().map(|id| id.to_string()).collect();
        ids.join(",")
    }

    /// Validate that a connection ID exists in the table.
    ///
    /// Returns error 706 if not found.
    pub fn validate_connection(&self, connection_id: i32) -> Result<(), r#static::Error> {
        self.connection_table
            .lock()
            .unwrap()
            .contains_key(&connection_id)
            .then_some(())
            .ok_or(r#static::Error::InvalidConnectionReference)
    }

    /// Get the number of active connections.
    pub fn connection_count(&self) -> usize {
        self.connection_table.lock().unwrap().len()
    }

    /// Get a reference to the connection table for validation by other bridges.
    pub fn connection_table(&self) -> &Arc<std::sync::Mutex<HashMap<i32, ConnectionInfo>>> {
        &self.connection_table
    }

    /// Initialize all ConnectionManager state variables per UPnP-av-ConnectionManager-v3 spec §4.2.
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
    /// 1. Sets the value in StateStore
    /// 2. Triggers GENA events via direct NOTIFY (CM has no LastChange var)
    ///
    /// ConnectionManager has NO instance-scoped variables per UPnP spec.
    /// The only evented variable is DeviceClockInfoUpdates (is_evented=YES, via_lastchange=—),
    /// which sends a bare propertyset NOTIFY — no LastChange wrapping.
    ///
    /// Writing the same value does NOT set has_changes (per UPnP spec).
    pub fn set_state_var(
        &self,
        _instance_id: u32,
        name: r#static::StateVariableName,
        value: DataType,
    ) {
        let mut store = self.state_store.lock().unwrap();
        store.set(name, value);
        drop(store);

        self.trigger_events();
    }

    /// Trigger GENA events for all evented state variables.
    ///
    /// ConnectionManager uses direct NOTIFY (bare propertyset), not LastChange.
    /// Only DeviceClockInfoUpdates is evented per spec §4.2.
    fn trigger_events(&self) {
        let store = self.state_store.lock().unwrap();
        let evented = store.collect_evented();
        drop(store);

        // Direct NOTIFY: bare propertyset, no LastChange wrapping
        let mut publisher = self.event_publisher.lock().unwrap();
        let _results = publisher.notify(&evented);
    }

    /// Get all state variable names (for SCPD generation).
    pub fn state_var_names(&self) -> Vec<String> {
        let store = self.state_store.lock().unwrap();
        store.names()
    }
}
