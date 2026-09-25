pub mod actions;
pub mod r#static;
pub mod traits;

pub use traits::*;

use crate::gena::EventPublisher;
use crate::state::StateVariableName;
use crate::types::upnp::{DataType as StateValue, Services, StateSchema, StateStore};
use std::sync::{Arc, Mutex};

/// AVTransport service — owns action registry, state variables, and eventing.
///
/// The application registers trait implementations (PlayAction, StopAction, etc.)
/// via `register_action()`. The SOAP handler dispatches incoming requests
/// to the correct trait implementation.
///
/// State variables are initialized in `init_state_vars()` and owned by this struct.
/// When state changes via `set_state_var()`, the service automatically:
/// 1. Validates the new value against allowed_values/range
/// 2. Detects if the value actually changed (same value = no event)
/// 3. Collects all evented state variables with current values
/// 4. Builds LastChange XML propertyset via `build_last_change()`
/// 5. Updates the LastChange state variable
/// 6. Notifies all GENA subscribers via `EventPublisher`
#[derive(Clone)]
pub struct AvTransportService {
    actions: Arc<Mutex<crate::types::upnp::ActionMap>>,
    state_store: Arc<Mutex<StateStore<r#static::StateVariableName>>>,
    event_publisher: Arc<std::sync::Mutex<EventPublisher>>,
}

/// Shared event publisher type alias for convenience.
pub type AvTransportEventPublisher = Arc<std::sync::Mutex<EventPublisher>>;

impl AvTransportService {
    /// Create a new empty AVTransport service with all state variables initialized.
    pub fn new() -> Self {
        Self::with_event_publisher(EventPublisher::new(
            Services::AVTransport(crate::types::upnp::ServiceVersion::V3).event_url(),
            false,
        ))
    }

    /// Create a new AVTransport service with a custom event publisher.
    pub fn with_event_publisher(publisher: EventPublisher) -> Self {
        let mut state_store = StateStore::new();
        Self::init_state_vars(&mut state_store);
        Self {
            actions: Arc::new(Mutex::new(crate::types::upnp::ActionMap::new(
                Services::AVTransport(crate::types::upnp::ServiceVersion::V3),
            ))),
            state_store: Arc::new(Mutex::new(state_store)),
            event_publisher: Arc::new(std::sync::Mutex::new(publisher)),
        }
    }

    /// Get a clone of the shared event publisher.
    pub fn event_publisher(&self) -> AvTransportEventPublisher {
        Arc::clone(&self.event_publisher)
    }

    /// Get an Arc-wrapped reference to the state store for action construction.
    ///
    /// Actions need `Arc<Mutex<StateStore<StateVariableName>>>` to read/write state
    /// and trigger LastChange events after successful trait execution.
    pub fn state_store(&self) -> Arc<Mutex<StateStore<r#static::StateVariableName>>> {
        Arc::clone(&self.state_store)
    }

    /// Initialize all AVTransport state variables per UPnP-av-AVTransport-v3 spec §4.1.
    ///
    /// 30 state variables total:
    /// - 3 core transport state (TransportState, TransportStatus, CurrentMediaCategory)
    /// - 6 media identity (AVTransportURI, AVTransportURIMetaData, NextAVTransportURI,
    ///   NextAVTransportURIMetaData, CurrentTrackURI, CurrentTrackMetaData)
    /// - 8 playback position (CurrentTrack, NumberOfTracks, CurrentTrackDuration,
    ///   CurrentMediaDuration, RelativeTimePosition, AbsoluteTimePosition,
    ///   RelativeCounterPosition, AbsoluteCounterPosition)
    /// - 4 storage & capabilities (PlaybackStorageMedium, RecordStorageMedium,
    ///   PossiblePlaybackStorageMedia, PossibleRecordStorageMedia)
    /// - 2 play control (CurrentPlayMode, TransportPlaySpeed)
    /// - 4 recording (RecordMediumWriteStatus, CurrentRecordQualityMode,
    ///   PossibleRecordQualityModes, DRMState)
    /// - 2 eventing (LastChange, CurrentTransportActions)
    fn init_state_vars(state_store: &mut StateStore<r#static::StateVariableName>) {
        use r#static::StateVariableName;

        // Core transport state
        state_store.register(StateSchema {
            name: StateVariableName::TransportState,
            default: StateValue::String("STOPPED".to_string()),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::TransportStatus,
            default: StateValue::String("OK".to_string()),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::CurrentMediaCategory,
            default: StateValue::String("NO_MEDIA".to_string()),
            ..Default::default()
        });

        // Media identity
        state_store.register(StateSchema {
            name: StateVariableName::AVTransportURI,
            default: StateValue::String(String::new()),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::AVTransportURIMetaData,
            default: StateValue::String("NOT_IMPLEMENTED".to_string()),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::NextAVTransportURI,
            default: StateValue::String("NOT_IMPLEMENTED".to_string()),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::NextAVTransportURIMetaData,
            default: StateValue::String("NOT_IMPLEMENTED".to_string()),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::CurrentTrackURI,
            default: StateValue::String(String::new()),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::CurrentTrackMetaData,
            default: StateValue::String("NOT_IMPLEMENTED".to_string()),
            ..Default::default()
        });

        // Playback position
        state_store.register(StateSchema {
            name: StateVariableName::CurrentTrack,
            default: StateValue::Ui4(0),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::NumberOfTracks,
            default: StateValue::Ui4(0),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::CurrentTrackDuration,
            default: StateValue::String("00:00:00".to_string()),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::CurrentMediaDuration,
            default: StateValue::String("00:00:00".to_string()),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::RelativeTimePosition,
            default: StateValue::String("NOT_IMPLEMENTED".to_string()),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::AbsoluteTimePosition,
            default: StateValue::String("NOT_IMPLEMENTED".to_string()),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::RelativeCounterPosition,
            default: StateValue::I4(i32::MAX),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::AbsoluteCounterPosition,
            default: StateValue::Ui4(0),
            ..Default::default()
        });

        // Storage & capabilities
        state_store.register(StateSchema {
            name: StateVariableName::PlaybackStorageMedium,
            default: StateValue::String("NONE".to_string()),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::RecordStorageMedium,
            default: StateValue::String("NOT_IMPLEMENTED".to_string()),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::PossiblePlaybackStorageMedia,
            default: StateValue::String("NONE".to_string()),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::PossibleRecordStorageMedia,
            default: StateValue::String("NOT_IMPLEMENTED".to_string()),
            ..Default::default()
        });

        // Play control
        state_store.register(StateSchema {
            name: StateVariableName::CurrentPlayMode,
            default: StateValue::String("NORMAL".to_string()),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::TransportPlaySpeed,
            default: StateValue::String("1".to_string()),
            ..Default::default()
        });

        // Recording
        state_store.register(StateSchema {
            name: StateVariableName::RecordMediumWriteStatus,
            default: StateValue::String("NOT_IMPLEMENTED".to_string()),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::CurrentRecordQualityMode,
            default: StateValue::String("NOT_IMPLEMENTED".to_string()),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::PossibleRecordQualityModes,
            default: StateValue::String("NOT_IMPLEMENTED".to_string()),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::DRMState,
            default: StateValue::String("UNKNOWN".to_string()),
            ..Default::default()
        });

        // Eventing
        state_store.register(StateSchema {
            name: StateVariableName::LastChange,
            default: StateValue::String(String::new()),
            ..Default::default()
        });

        state_store.register(StateSchema {
            name: StateVariableName::CurrentTransportActions,
            default: StateValue::String("PLAY,STOP,PAUSE,SEEK,NEXT,PREVIOUS".to_string()),
            ..Default::default()
        });

        // =========================================================================
        // A_ARG_TYPE variables — type definitions for action arguments (not real state)
        // =========================================================================

        // A_ARG_TYPE_InstanceID — ui4, identifies virtual transport instance
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_InstanceID,
            default: StateValue::Ui4(0),
            argument_type: true, // Type definition, no defaultValue in SCPD
            ..Default::default()
        });

        // A_ARG_TYPE_SeekMode — string, seek mode identifier
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_SeekMode,
            default: StateValue::String(String::new()),
            allowed_values: Some(vec![
                "REL_TIME".to_string(),
                "ABS_TIME".to_string(),
                "TRACK_NR".to_string(),
                "TIME_OFFSET".to_string(),
                "TRACK_OFFSET".to_string(),
                "ABSOLUTE_TIME".to_string(),
                "ABSOLUTE_TIME_PRES".to_string(),
                "ABSOLUTE_TIME_COUNTER".to_string(),
                "ABSOLUTE_TIME_COUNTER_PRES".to_string(),
            ]),
            argument_type: true,
            ..Default::default()
        });

        // A_ARG_TYPE_SeekTarget — string, seek target value
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_SeekTarget,
            default: StateValue::String(String::new()),
            argument_type: true,
            ..Default::default()
        });

        // A_ARG_TYPE_RecordMedium — string, record medium type
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_RecordMedium,
            default: StateValue::String(String::new()),
            argument_type: true,
            ..Default::default()
        });

        // A_ARG_TYPE_StreamFormat — string, stream format
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_StreamFormat,
            default: StateValue::String(String::new()),
            argument_type: true,
            ..Default::default()
        });
    }

    /// Register a trait implementation for the given action name.
    pub fn register_action(&mut self, action: Box<dyn crate::types::upnp::Action>) {
        self.actions.lock().unwrap().register(action);
    }

    /// Get the action map for SOAP dispatch.
    pub fn actions(&self) -> Arc<Mutex<crate::types::upnp::ActionMap>> {
        Arc::clone(&self.actions)
    }

    /// Get a state variable by name.
    pub fn get_state_var(
        &self,
        name: r#static::StateVariableName,
    ) -> Result<crate::types::upnp::StateValue, crate::types::upnp::Error> {
        let store = self.state_store.lock().unwrap();
        store.get(name).map(|v| v.clone())
    }

    /// Set any state variable at a specific InstanceID.
    ///
    /// This is the single gatekeeper method for all state mutations. It:
    /// 1. Sets the value in StateStore (uses set_instance if var is instance-scoped)
    /// 2. Triggers GENA events via LastChange NOTIFY
    ///
    /// Per UPnP-av-2.0 spec, InstanceID=0 is global/post-mix, InstanceID>0 is per-stream.
    /// AVTransport state variables are all per-InstanceID (is_instance_scoped=true).
    ///
    /// Writing the same value does NOT set has_changes (per UPnP spec).
    pub fn set_state_var(
        &self,
        instance_id: u32,
        name: r#static::StateVariableName,
        value: StateValue,
    ) {
        let mut store = self.state_store.lock().unwrap();

        if name.is_instance_scoped() {
            store.set_instance(name, instance_id, value);
        } else {
            store.set(name, value);
        }
        drop(store);

        self.trigger_events_for_instance(instance_id);
    }

    /// Build the LastChange XML propertyset from all evented state variables for an instance.
    ///
    /// Format per UPnP-av-2.0 spec:
    /// ```xml
    /// <e:propertyset xmlns:e="urn:schemas-upnp-org:event-1-0">
    ///   <e:property>
    ///     <LastChange>&lt;Event xmlns="urn:schemas-upnp-org:metadata-1-0/AVT/"&gt;
    ///       &lt;InstanceID val="N"&gt;
    ///         &lt;TransportState&gt;PLAYING&lt;/TransportState&gt;
    ///       &lt;/InstanceID&gt;
    ///     &lt;/Event&gt;</LastChange>
    ///   </e:property>
    /// </e:propertyset>
    /// ```
    fn build_last_change(&self, instance_id: u32) -> String {
        let store = self.state_store.lock().unwrap();
        let evented = store.collect_evented_for_instance(instance_id);
        crate::services::lastchange::build_last_change(
            "urn:schemas-upnp-org:metadata-1-0/AVT/",
            &evented,
        )
    }

    /// Trigger GENA events for all evented state variables at a specific InstanceID.
    ///
    /// 1. Builds LastChange XML from all evented vars for the instance
    /// 2. Updates the LastChange state variable with the escaped XML
    /// 3. Notifies all subscribers via EventPublisher
    fn trigger_events_for_instance(&self, instance_id: u32) {
        let last_change_xml = self.build_last_change(instance_id);

        // Update LastChange state variable (global, not per-instance)
        {
            let mut store = self.state_store.lock().unwrap();
            if let Some(last_change_var) = store.get_mut(r#static::StateVariableName::LastChange) {
                last_change_var.current_value = StateValue::String(last_change_xml.clone());
            }
        }

        // Notify all subscribers with LastChange property (name="LastChange", value=XML string)
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
