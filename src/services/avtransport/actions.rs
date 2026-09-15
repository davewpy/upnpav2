/// AVTransport service — bridge from SOAP `Action` trait to application traits.
///
/// Each struct wraps an Arc<T> where T implements the application trait.
/// The `execute` method extracts typed args, calls the trait, and wraps results.
///
/// Bridge structs are prefixed with `Action` to avoid name collision with traits.
use std::sync::{Arc, Mutex};

use super::AvTransportEventPublisher;
use super::r#static::{PlayMode, StateVariableName};
use crate::services::avtransport::traits::*;
use crate::types::upnp::{
    Action, ActionArgs, ArgumentDefinition, ArgumentDirection, Error, StateStore, StateValue,
};

// ===========================================================================
// Required Actions (R) — MVP Core
// ===========================================================================

pub struct ActionPlay<T: Play> {
    trait_impl: Arc<T>,
    state_store: Arc<Mutex<StateStore<StateVariableName>>>,
    event_publisher: AvTransportEventPublisher,
}

impl<T: Play> ActionPlay<T> {
    pub fn new(
        trait_impl: T,
        state_store: Arc<Mutex<StateStore<StateVariableName>>>,
        event_publisher: AvTransportEventPublisher,
    ) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            state_store,
            event_publisher,
        }
    }

    fn trigger_last_change(&self) {
        let last_change_xml = {
            let store = self.state_store.lock().unwrap();
            crate::services::lastchange::build_last_change(
                "urn:schemas-upnp-org:metadata-1-0/AVT/",
                &store.collect_evented(),
            )
        };
        let mut store = self.state_store.lock().unwrap();
        if let Some(last_change_var) = store.get_mut(StateVariableName::LastChange) {
            last_change_var.current_value = StateValue::String(last_change_xml.clone());
        }
        drop(store);
        let properties = vec![("LastChange".to_string(), last_change_xml)];
        let mut publisher = self.event_publisher.lock().unwrap();
        let _ = publisher.notify(&properties);
    }
}

impl<T: Play> Action for ActionPlay<T> {
    fn name(&self) -> &'static str {
        "Play"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            ArgumentDefinition {
                name: "Speed",
                direction: ArgumentDirection::IN,
                related_state_var: Some("TransportPlaySpeed"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let speed = args.get("Speed").unwrap_or("1").to_string();
        let input = PlayInput {
            instance_id,
            speed: speed.clone(),
        };
        self.trait_impl
            .play(input)
            .map_err(|_| Error::ActionFailed)?;

        // Update state variables per spec §Play state effects
        let mut store = self.state_store.lock().unwrap();
        store.set(
            StateVariableName::TransportState,
            StateValue::String("PLAYING".to_string()),
        )?;
        store.set(
            StateVariableName::TransportPlaySpeed,
            StateValue::String(speed),
        )?;
        drop(store);
        // Trigger LastChange event
        self.trigger_last_change();

        Ok(ActionArgs::new())
    }
}

pub struct ActionStop<T: Stop> {
    trait_impl: Arc<T>,
    state_store: Arc<Mutex<StateStore<StateVariableName>>>,
    event_publisher: AvTransportEventPublisher,
}

impl<T: Stop> ActionStop<T> {
    pub fn new(
        trait_impl: T,
        state_store: Arc<Mutex<StateStore<StateVariableName>>>,
        event_publisher: AvTransportEventPublisher,
    ) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            state_store,
            event_publisher,
        }
    }

    fn trigger_last_change(&self) {
        let last_change_xml = {
            let store = self.state_store.lock().unwrap();
            crate::services::lastchange::build_last_change(
                "urn:schemas-upnp-org:metadata-1-0/AVT/",
                &store.collect_evented(),
            )
        };
        let mut store = self.state_store.lock().unwrap();
        if let Some(last_change_var) = store.get_mut(StateVariableName::LastChange) {
            last_change_var.current_value = StateValue::String(last_change_xml.clone());
        }
        drop(store);
        let properties = vec![("LastChange".to_string(), last_change_xml)];
        let mut publisher = self.event_publisher.lock().unwrap();
        let _ = publisher.notify(&properties);
    }
}

impl<T: Stop> Action for ActionStop<T> {
    fn name(&self) -> &'static str {
        "Stop"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let input = StopInput { instance_id };
        self.trait_impl
            .stop(input)
            .map_err(|_| Error::ActionFailed)?;

        // Update state variables per spec §Stop state effects
        let mut store = self.state_store.lock().unwrap();
        store.set(
            StateVariableName::TransportState,
            StateValue::String("STOPPED".to_string()),
        )?;
        drop(store);
        // Trigger LastChange event
        self.trigger_last_change();

        Ok(ActionArgs::new())
    }
}

pub struct ActionSeek<T: Seek> {
    trait_impl: Arc<T>,
    state_store: Arc<Mutex<StateStore<StateVariableName>>>,
    event_publisher: AvTransportEventPublisher,
}

impl<T: Seek> ActionSeek<T> {
    pub fn new(
        trait_impl: T,
        state_store: Arc<Mutex<StateStore<StateVariableName>>>,
        event_publisher: AvTransportEventPublisher,
    ) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            state_store,
            event_publisher,
        }
    }

    fn trigger_last_change(&self) {
        let last_change_xml = {
            let store = self.state_store.lock().unwrap();
            crate::services::lastchange::build_last_change(
                "urn:schemas-upnp-org:metadata-1-0/AVT/",
                &store.collect_evented(),
            )
        };
        let mut store = self.state_store.lock().unwrap();
        if let Some(last_change_var) = store.get_mut(StateVariableName::LastChange) {
            last_change_var.current_value = StateValue::String(last_change_xml.clone());
        }
        drop(store);
        let properties = vec![("LastChange".to_string(), last_change_xml)];
        let mut publisher = self.event_publisher.lock().unwrap();
        let _ = publisher.notify(&properties);
    }
}

impl<T: Seek> Action for ActionSeek<T> {
    fn name(&self) -> &'static str {
        "Seek"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            ArgumentDefinition {
                name: "Unit",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_SeekMode"),
            },
            ArgumentDefinition {
                name: "Target",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_SeekTarget"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let unit = args
            .get("Unit")
            .ok_or(Error::ArgumentValueInvalid)?
            .to_string();
        let target = args
            .get("Target")
            .ok_or(Error::ArgumentValueInvalid)?
            .to_string();
        let input = SeekInput {
            instance_id,
            unit,
            target,
        };
        self.trait_impl
            .seek(input)
            .map_err(|_| Error::ActionFailed)?;

        // Update state variables per spec §Seek state effects
        // Seek: TransportState → TRANSITIONING → previous state
        // For now, set to TRANSITIONING; app trait should restore final state
        let mut store = self.state_store.lock().unwrap();
        store.set(
            StateVariableName::TransportState,
            StateValue::String("TRANSITIONING".to_string()),
        )?;
        drop(store);
        // Trigger LastChange event
        self.trigger_last_change();

        Ok(ActionArgs::new())
    }
}

pub struct ActionNext<T: Next> {
    trait_impl: Arc<T>,
    state_store: Arc<Mutex<StateStore<StateVariableName>>>,
    event_publisher: AvTransportEventPublisher,
}

impl<T: Next> ActionNext<T> {
    pub fn new(
        trait_impl: T,
        state_store: Arc<Mutex<StateStore<StateVariableName>>>,
        event_publisher: AvTransportEventPublisher,
    ) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            state_store,
            event_publisher,
        }
    }

    fn trigger_last_change(&self) {
        let last_change_xml = {
            let store = self.state_store.lock().unwrap();
            crate::services::lastchange::build_last_change(
                "urn:schemas-upnp-org:metadata-1-0/AVT/",
                &store.collect_evented(),
            )
        };
        let mut store = self.state_store.lock().unwrap();
        if let Some(last_change_var) = store.get_mut(StateVariableName::LastChange) {
            last_change_var.current_value = StateValue::String(last_change_xml.clone());
        }
        drop(store);
        let properties = vec![("LastChange".to_string(), last_change_xml)];
        let mut publisher = self.event_publisher.lock().unwrap();
        let _ = publisher.notify(&properties);
    }
}

impl<T: Next> Action for ActionNext<T> {
    fn name(&self) -> &'static str {
        "Next"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let input = NextInput { instance_id };
        self.trait_impl
            .next(input)
            .map_err(|_| Error::ActionFailed)?;

        // Update state variables per spec §Next state effects
        let mut store = self.state_store.lock().unwrap();
        store.set(
            StateVariableName::TransportState,
            StateValue::String("TRANSITIONING".to_string()),
        )?;
        drop(store);
        // Trigger LastChange event
        self.trigger_last_change();

        Ok(ActionArgs::new())
    }
}

pub struct ActionPrevious<T: Previous> {
    trait_impl: Arc<T>,
    state_store: Arc<Mutex<StateStore<StateVariableName>>>,
    event_publisher: AvTransportEventPublisher,
}

impl<T: Previous> ActionPrevious<T> {
    pub fn new(
        trait_impl: T,
        state_store: Arc<Mutex<StateStore<StateVariableName>>>,
        event_publisher: AvTransportEventPublisher,
    ) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            state_store,
            event_publisher,
        }
    }

    fn trigger_last_change(&self) {
        let last_change_xml = {
            let store = self.state_store.lock().unwrap();
            crate::services::lastchange::build_last_change(
                "urn:schemas-upnp-org:metadata-1-0/AVT/",
                &store.collect_evented(),
            )
        };
        let mut store = self.state_store.lock().unwrap();
        if let Some(last_change_var) = store.get_mut(StateVariableName::LastChange) {
            last_change_var.current_value = StateValue::String(last_change_xml.clone());
        }
        drop(store);
        let properties = vec![("LastChange".to_string(), last_change_xml)];
        let mut publisher = self.event_publisher.lock().unwrap();
        let _ = publisher.notify(&properties);
    }
}

impl<T: Previous> Action for ActionPrevious<T> {
    fn name(&self) -> &'static str {
        "Previous"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let input = PreviousInput { instance_id };
        self.trait_impl
            .previous(input)
            .map_err(|_| Error::ActionFailed)?;

        // Update state variables per spec §Previous state effects
        let mut store = self.state_store.lock().unwrap();
        store.set(
            StateVariableName::TransportState,
            StateValue::String("TRANSITIONING".to_string()),
        )?;
        drop(store);
        // Trigger LastChange event
        self.trigger_last_change();

        Ok(ActionArgs::new())
    }
}

pub struct ActionSetAVTransportURI<T: SetAVTransportURI> {
    trait_impl: Arc<T>,
    state_store: Arc<Mutex<StateStore<StateVariableName>>>,
    event_publisher: AvTransportEventPublisher,
}

impl<T: SetAVTransportURI> ActionSetAVTransportURI<T> {
    pub fn new(
        trait_impl: T,
        state_store: Arc<Mutex<StateStore<StateVariableName>>>,
        event_publisher: AvTransportEventPublisher,
    ) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            state_store,
            event_publisher,
        }
    }
}

impl<T: SetAVTransportURI> Action for ActionSetAVTransportURI<T> {
    fn name(&self) -> &'static str {
        "SetAVTransportURI"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            ArgumentDefinition {
                name: "CurrentURI",
                direction: ArgumentDirection::IN,
                related_state_var: Some("AVTransportURI"),
            },
            ArgumentDefinition {
                name: "CurrentURIMetaData",
                direction: ArgumentDirection::IN,
                related_state_var: Some("AVTransportURIMetaData"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "NrTracks",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("NumberOfTracks"),
            },
            ArgumentDefinition {
                name: "MediaDuration",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentMediaDuration"),
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
        let current_uri = args
            .get("CurrentURI")
            .ok_or(Error::ArgumentValueInvalid)?
            .to_string();
        let current_uri_metadata = args.get("CurrentURIMetaData").unwrap_or("").to_string();
        let input = SetAVTransportURIInput {
            instance_id,
            current_uri: current_uri.clone(),
            current_uri_metadata: current_uri_metadata.clone(),
        };
        let output = self
            .trait_impl
            .set_av_transport_uri(input)
            .map_err(|_| Error::ActionFailed)?;

        // Update state variables per spec §SetAVTransportURI state effects
        {
            // Derive PlaybackStorageMedium from URI scheme first (before moving current_uri)
            let medium = if current_uri.starts_with("http") {
                "NETWORK"
            } else if current_uri.starts_with("file://") {
                "HDD"
            } else {
                "UNKNOWN"
            };
            let nr_tracks = output.nr_tracks;
            let media_duration = output.media_duration.clone();
            let current_uri_metadata = current_uri_metadata.clone();

            let mut store = self.state_store.lock().unwrap();
            store.set(
                StateVariableName::AVTransportURI,
                StateValue::Uri(current_uri),
            )?;
            store.set(
                StateVariableName::AVTransportURIMetaData,
                StateValue::String(current_uri_metadata),
            )?;
            store.set(
                StateVariableName::NumberOfTracks,
                StateValue::Ui4(nr_tracks),
            )?;
            store.set(
                StateVariableName::CurrentMediaDuration,
                StateValue::String(media_duration),
            )?;
            store.set(
                StateVariableName::PlaybackStorageMedium,
                StateValue::String(medium.to_string()),
            )?;
            store.set(
                StateVariableName::CurrentMediaCategory,
                StateValue::String(
                    if nr_tracks > 0 {
                        "TRACK_AWARE"
                    } else {
                        "NO_MEDIA"
                    }
                    .to_string(),
                ),
            )?;
        }
        // Trigger LastChange event
        let last_change_xml = {
            let store = self.state_store.lock().unwrap();
            crate::services::lastchange::build_last_change(
                "urn:schemas-upnp-org:metadata-1-0/AVT/",
                &store.collect_evented(),
            )
        };
        let mut store = self.state_store.lock().unwrap();
        if let Some(last_change_var) = store.get_mut(StateVariableName::LastChange) {
            last_change_var.current_value = StateValue::String(last_change_xml.clone());
        }
        drop(store);
        let properties = vec![("LastChange".to_string(), last_change_xml)];
        let mut publisher = self.event_publisher.lock().unwrap();
        let _ = publisher.notify(&properties);

        let mut out = ActionArgs::new();
        out.set("NrTracks".to_string(), output.nr_tracks.to_string());
        out.set("MediaDuration".to_string(), output.media_duration);
        Ok(out)
    }
}

pub struct ActionGetMediaInfo<T: GetMediaInfo> {
    trait_impl: Arc<T>,
    state_store: Arc<Mutex<StateStore<StateVariableName>>>,
    event_publisher: AvTransportEventPublisher,
}

impl<T: GetMediaInfo> ActionGetMediaInfo<T> {
    pub fn new(
        trait_impl: T,
        state_store: Arc<Mutex<StateStore<StateVariableName>>>,
        event_publisher: AvTransportEventPublisher,
    ) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            state_store,
            event_publisher,
        }
    }
}

impl<T: GetMediaInfo> Action for ActionGetMediaInfo<T> {
    fn name(&self) -> &'static str {
        "GetMediaInfo"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "NrTracks",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("NumberOfTracks"),
            },
            ArgumentDefinition {
                name: "MediaDuration",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentMediaDuration"),
            },
            ArgumentDefinition {
                name: "CurrentURI",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("AVTransportURI"),
            },
            ArgumentDefinition {
                name: "CurrentURIMetaData",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("AVTransportURIMetaData"),
            },
            ArgumentDefinition {
                name: "NextURI",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("NextAVTransportURI"),
            },
            ArgumentDefinition {
                name: "NextURIMetaData",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("NextAVTransportURIMetaData"),
            },
            ArgumentDefinition {
                name: "PlayMedium",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaybackStorageMedium"),
            },
            ArgumentDefinition {
                name: "RecordMedium",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("RecordStorageMedium"),
            },
            ArgumentDefinition {
                name: "WriteStatus",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("RecordMediumWriteStatus"),
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
        let input = GetMediaInfoInput { instance_id };
        let output = self
            .trait_impl
            .get_media_info(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set("NrTracks".to_string(), output.nr_tracks.to_string());
        out.set("MediaDuration".to_string(), output.media_duration);
        out.set("CurrentURI".to_string(), output.current_uri);
        out.set(
            "CurrentURIMetaData".to_string(),
            output.current_uri_metadata,
        );
        out.set("NextURI".to_string(), output.next_uri);
        out.set("NextURIMetaData".to_string(), output.next_uri_metadata);
        out.set(
            "PlayMedium".to_string(),
            output.play_medium.as_str().to_string(),
        );
        out.set(
            "RecordMedium".to_string(),
            output.record_medium.as_str().to_string(),
        );
        out.set(
            "WriteStatus".to_string(),
            output.write_status.as_str().to_string(),
        );
        Ok(out)
    }
}

pub struct ActionGetTransportInfo<T: GetTransportInfo> {
    trait_impl: Arc<T>,
    state_store: Arc<Mutex<StateStore<StateVariableName>>>,
    event_publisher: AvTransportEventPublisher,
}

impl<T: GetTransportInfo> ActionGetTransportInfo<T> {
    pub fn new(
        trait_impl: T,
        state_store: Arc<Mutex<StateStore<StateVariableName>>>,
        event_publisher: AvTransportEventPublisher,
    ) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            state_store,
            event_publisher,
        }
    }
}

impl<T: GetTransportInfo> Action for ActionGetTransportInfo<T> {
    fn name(&self) -> &'static str {
        "GetTransportInfo"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "CurrentTransportState",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("TransportState"),
            },
            ArgumentDefinition {
                name: "CurrentTransportStatus",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("TransportStatus"),
            },
            ArgumentDefinition {
                name: "CurrentSpeed",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("TransportPlaySpeed"),
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
        let input = GetTransportInfoInput { instance_id };
        let output = self
            .trait_impl
            .get_transport_info(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set(
            "CurrentTransportState".to_string(),
            output.transport_state.as_str().to_string(),
        );
        out.set(
            "CurrentTransportStatus".to_string(),
            output.transport_status.as_str().to_string(),
        );
        out.set("CurrentSpeed".to_string(), output.play_speed);
        Ok(out)
    }
}

pub struct ActionGetPositionInfo<T: GetPositionInfo> {
    trait_impl: Arc<T>,
    state_store: Arc<Mutex<StateStore<StateVariableName>>>,
    event_publisher: AvTransportEventPublisher,
}

impl<T: GetPositionInfo> ActionGetPositionInfo<T> {
    pub fn new(
        trait_impl: T,
        state_store: Arc<Mutex<StateStore<StateVariableName>>>,
        event_publisher: AvTransportEventPublisher,
    ) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            state_store,
            event_publisher,
        }
    }
}

impl<T: GetPositionInfo> Action for ActionGetPositionInfo<T> {
    fn name(&self) -> &'static str {
        "GetPositionInfo"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "Track",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentTrack"),
            },
            ArgumentDefinition {
                name: "TrackDuration",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentTrackDuration"),
            },
            ArgumentDefinition {
                name: "TrackMetaData",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentTrackMetaData"),
            },
            ArgumentDefinition {
                name: "TrackURI",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentTrackURI"),
            },
            ArgumentDefinition {
                name: "RelTime",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("RelativeTimePosition"),
            },
            ArgumentDefinition {
                name: "AbsTime",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("AbsoluteTimePosition"),
            },
            ArgumentDefinition {
                name: "RelCount",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("RelativeCounterPosition"),
            },
            ArgumentDefinition {
                name: "AbsCount",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("AbsoluteCounterPosition"),
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
        let input = GetPositionInfoInput { instance_id };
        let output = self
            .trait_impl
            .get_position_info(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set("Track".to_string(), output.track.to_string());
        out.set("TrackDuration".to_string(), output.track_duration);
        out.set("TrackMetaData".to_string(), output.track_metadata);
        out.set("TrackURI".to_string(), output.track_uri);
        out.set("RelTime".to_string(), output.rel_time);
        out.set("AbsTime".to_string(), output.abs_time);
        out.set("RelCount".to_string(), output.rel_count.to_string());
        out.set("AbsCount".to_string(), output.abs_count.to_string());
        Ok(out)
    }
}

pub struct ActionGetDeviceCapabilities<T: GetDeviceCapabilities> {
    trait_impl: Arc<T>,
    state_store: Arc<Mutex<StateStore<StateVariableName>>>,
    event_publisher: AvTransportEventPublisher,
}

impl<T: GetDeviceCapabilities> ActionGetDeviceCapabilities<T> {
    pub fn new(
        trait_impl: T,
        state_store: Arc<Mutex<StateStore<StateVariableName>>>,
        event_publisher: AvTransportEventPublisher,
    ) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            state_store,
            event_publisher,
        }
    }
}

impl<T: GetDeviceCapabilities> Action for ActionGetDeviceCapabilities<T> {
    fn name(&self) -> &'static str {
        "GetDeviceCapabilities"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "PlayMedia",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PossiblePlaybackStorageMedia"),
            },
            ArgumentDefinition {
                name: "RecMedia",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PossibleRecordStorageMedia"),
            },
            ArgumentDefinition {
                name: "RecQualityModes",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PossibleRecordQualityModes"),
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
        let input = GetDeviceCapabilitiesInput { instance_id };
        let output = self
            .trait_impl
            .get_device_capabilities(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set("PlayMedia".to_string(), output.play_media);
        out.set("RecMedia".to_string(), output.rec_media);
        out.set("RecQualityModes".to_string(), output.rec_quality_modes);
        Ok(out)
    }
}

pub struct ActionGetTransportSettings<T: GetTransportSettings> {
    trait_impl: Arc<T>,
    state_store: Arc<Mutex<StateStore<StateVariableName>>>,
    event_publisher: AvTransportEventPublisher,
}

impl<T: GetTransportSettings> ActionGetTransportSettings<T> {
    pub fn new(
        trait_impl: T,
        state_store: Arc<Mutex<StateStore<StateVariableName>>>,
        event_publisher: AvTransportEventPublisher,
    ) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            state_store,
            event_publisher,
        }
    }
}

impl<T: GetTransportSettings> Action for ActionGetTransportSettings<T> {
    fn name(&self) -> &'static str {
        "GetTransportSettings"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "PlayMode",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentPlayMode"),
            },
            ArgumentDefinition {
                name: "RecQualityMode",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentRecordQualityMode"),
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
        let input = GetTransportSettingsInput { instance_id };
        let output = self
            .trait_impl
            .get_transport_settings(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set(
            "PlayMode".to_string(),
            output.play_mode.as_str().to_string(),
        );
        out.set("RecQualityMode".to_string(), output.rec_quality_mode);
        Ok(out)
    }
}

// ===========================================================================
// Allowed Actions (A)
// ===========================================================================

pub struct ActionSetNextAVTransportURI<T: SetNextAVTransportURI> {
    trait_impl: Arc<T>,
    state_store: Arc<Mutex<StateStore<StateVariableName>>>,
    event_publisher: AvTransportEventPublisher,
}

impl<T: SetNextAVTransportURI> ActionSetNextAVTransportURI<T> {
    pub fn new(
        trait_impl: T,
        state_store: Arc<Mutex<StateStore<StateVariableName>>>,
        event_publisher: AvTransportEventPublisher,
    ) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            state_store,
            event_publisher,
        }
    }
}

impl<T: SetNextAVTransportURI> Action for ActionSetNextAVTransportURI<T> {
    fn name(&self) -> &'static str {
        "SetNextAVTransportURI"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            ArgumentDefinition {
                name: "NextURI",
                direction: ArgumentDirection::IN,
                related_state_var: Some("NextAVTransportURI"),
            },
            ArgumentDefinition {
                name: "NextURIMetaData",
                direction: ArgumentDirection::IN,
                related_state_var: Some("NextAVTransportURIMetaData"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "NrTracks",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("NumberOfTracks"),
            },
            ArgumentDefinition {
                name: "MediaDuration",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentMediaDuration"),
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
        let next_uri = args
            .get("NextURI")
            .ok_or(Error::ArgumentValueInvalid)?
            .to_string();
        let next_uri_metadata = args.get("NextURIMetaData").unwrap_or("").to_string();
        let input = SetNextAVTransportURIInput {
            instance_id,
            next_uri,
            next_uri_metadata,
        };
        let output = self
            .trait_impl
            .set_next_av_transport_uri(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set("NrTracks".to_string(), output.nr_tracks.to_string());
        out.set("MediaDuration".to_string(), output.media_duration);
        Ok(out)
    }
}

pub struct ActionPause<T: Pause> {
    trait_impl: Arc<T>,
    state_store: Arc<Mutex<StateStore<StateVariableName>>>,
    event_publisher: AvTransportEventPublisher,
}

impl<T: Pause> ActionPause<T> {
    pub fn new(
        trait_impl: T,
        state_store: Arc<Mutex<StateStore<StateVariableName>>>,
        event_publisher: AvTransportEventPublisher,
    ) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            state_store,
            event_publisher,
        }
    }

    fn trigger_last_change(&self) {
        let last_change_xml = {
            let store = self.state_store.lock().unwrap();
            crate::services::lastchange::build_last_change(
                "urn:schemas-upnp-org:metadata-1-0/AVT/",
                &store.collect_evented(),
            )
        };
        let mut store = self.state_store.lock().unwrap();
        if let Some(last_change_var) = store.get_mut(StateVariableName::LastChange) {
            last_change_var.current_value = StateValue::String(last_change_xml.clone());
        }
        drop(store);
        let properties = vec![("LastChange".to_string(), last_change_xml)];
        let mut publisher = self.event_publisher.lock().unwrap();
        let _ = publisher.notify(&properties);
    }
}

impl<T: Pause> Action for ActionPause<T> {
    fn name(&self) -> &'static str {
        "Pause"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let input = PauseInput { instance_id };
        self.trait_impl
            .pause(input)
            .map_err(|_| Error::ActionFailed)?;

        // Update state variables per spec §Pause state effects
        let mut store = self.state_store.lock().unwrap();
        store.set(
            StateVariableName::TransportState,
            StateValue::String("PAUSED_PLAYBACK".to_string()),
        )?;
        drop(store);
        // Trigger LastChange event
        self.trigger_last_change();

        Ok(ActionArgs::new())
    }
}

pub struct ActionSetPlayMode<T: SetPlayMode> {
    trait_impl: Arc<T>,
    state_store: Arc<Mutex<StateStore<StateVariableName>>>,
    event_publisher: AvTransportEventPublisher,
}

impl<T: SetPlayMode> ActionSetPlayMode<T> {
    pub fn new(
        trait_impl: T,
        state_store: Arc<Mutex<StateStore<StateVariableName>>>,
        event_publisher: AvTransportEventPublisher,
    ) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            state_store,
            event_publisher,
        }
    }

    fn trigger_last_change(&self) {
        let last_change_xml = {
            let store = self.state_store.lock().unwrap();
            crate::services::lastchange::build_last_change(
                "urn:schemas-upnp-org:metadata-1-0/AVT/",
                &store.collect_evented(),
            )
        };
        let mut store = self.state_store.lock().unwrap();
        if let Some(last_change_var) = store.get_mut(StateVariableName::LastChange) {
            last_change_var.current_value = StateValue::String(last_change_xml.clone());
        }
        drop(store);
        let properties = vec![("LastChange".to_string(), last_change_xml)];
        let mut publisher = self.event_publisher.lock().unwrap();
        let _ = publisher.notify(&properties);
    }
}

impl<T: SetPlayMode> Action for ActionSetPlayMode<T> {
    fn name(&self) -> &'static str {
        "SetPlayMode"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            ArgumentDefinition {
                name: "NewPlayMode",
                direction: ArgumentDirection::IN,
                related_state_var: Some("CurrentPlayMode"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let play_mode_str = args.get("NewPlayMode").ok_or(Error::ArgumentValueInvalid)?;
        let play_mode = PlayMode::from_str(play_mode_str).ok_or(Error::ArgumentValueInvalid)?;
        let input = SetPlayModeInput {
            instance_id,
            play_mode,
        };
        self.trait_impl
            .set_play_mode(input)
            .map_err(|_| Error::ActionFailed)?;

        // Update state variables per spec §SetPlayMode state effects
        let mut store = self.state_store.lock().unwrap();
        store.set(
            StateVariableName::CurrentPlayMode,
            StateValue::String(play_mode.as_str().to_string()),
        )?;
        drop(store);
        // Trigger LastChange event
        self.trigger_last_change();

        Ok(ActionArgs::new())
    }
}

pub struct ActionGetCurrentTransportActions<T: GetCurrentTransportActions> {
    trait_impl: Arc<T>,
    state_store: Arc<Mutex<StateStore<StateVariableName>>>,
    event_publisher: AvTransportEventPublisher,
}

impl<T: GetCurrentTransportActions> ActionGetCurrentTransportActions<T> {
    pub fn new(
        trait_impl: T,
        state_store: Arc<Mutex<StateStore<StateVariableName>>>,
        event_publisher: AvTransportEventPublisher,
    ) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            state_store,
            event_publisher,
        }
    }
}

impl<T: GetCurrentTransportActions> Action for ActionGetCurrentTransportActions<T> {
    fn name(&self) -> &'static str {
        "GetCurrentTransportActions"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "Actions",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("CurrentTransportActions"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let input = GetCurrentTransportActionsInput { instance_id };
        let output = self
            .trait_impl
            .get_current_transport_actions(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set("Actions".to_string(), output.actions);
        Ok(out)
    }
}

// ===========================================================================
// Conditionally Required / Allowed (CR / CA) — Deferred
// ===========================================================================

pub struct ActionRecord<T: Record> {
    trait_impl: Arc<T>,
}

impl<T: Record> ActionRecord<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: Record> Action for ActionRecord<T> {
    fn name(&self) -> &'static str {
        "Record"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let input = StopInput { instance_id };
        self.trait_impl
            .record(input)
            .map_err(|_| Error::ActionFailed)?;
        Ok(ActionArgs::new())
    }
}

pub struct ActionSetRecordQualityMode<T: SetRecordQualityMode> {
    trait_impl: Arc<T>,
}

impl<T: SetRecordQualityMode> ActionSetRecordQualityMode<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: SetRecordQualityMode> Action for ActionSetRecordQualityMode<T> {
    fn name(&self) -> &'static str {
        "SetRecordQualityMode"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            ArgumentDefinition {
                name: "NewRecordQualityMode",
                direction: ArgumentDirection::IN,
                related_state_var: Some("CurrentRecordQualityMode"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let rec_quality_mode = args
            .get("NewRecordQualityMode")
            .ok_or(Error::ArgumentValueInvalid)?
            .to_string();
        let input = SetRecordQualityModeInput {
            instance_id,
            rec_quality_mode,
        };
        self.trait_impl
            .set_record_quality_mode(input)
            .map_err(|_| Error::ActionFailed)?;
        Ok(ActionArgs::new())
    }
}

pub struct ActionGetDRMState<T: GetDRMState> {
    trait_impl: Arc<T>,
}

impl<T: GetDRMState> ActionGetDRMState<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetDRMState> Action for ActionGetDRMState<T> {
    fn name(&self) -> &'static str {
        "GetDRMState"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "DRMState",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("DRMState"),
        }];
        &ARGS
    }

    fn execute(&self, _args: &ActionArgs) -> Result<ActionArgs, Error> {
        let output = self
            .trait_impl
            .get_drm_state()
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set("DRMState".to_string(), output.drm_state);
        Ok(out)
    }
}

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

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            ArgumentDefinition {
                name: "StateVariableList",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_StateVariableList"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
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
        let state_var_list_raw = args.get("StateVariableList").unwrap_or("*");
        let state_var_list: Vec<String> = if state_var_list_raw == "*" {
            vec![]
        } else {
            state_var_list_raw
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
        };
        let input = GetStateVariablesInput {
            instance_id,
            var_list: state_var_list,
        };
        let output = self
            .trait_impl
            .get_state_variables(input)
            .map_err(|_| Error::ActionFailed)?;
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
        let mut out = ActionArgs::new();
        out.set("StateVariableValuePairs".to_string(),
            format!("<stateVariableValuePairs xmlns=\"urn:schemas-upnp-org:av:avs\">{}</stateVariableValuePairs>", pairs));
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

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            ArgumentDefinition {
                name: "RenderingControlUDN",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_DeviceUDN"),
            },
            ArgumentDefinition {
                name: "ServiceType",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_ServiceType"),
            },
            ArgumentDefinition {
                name: "ServiceId",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_ServiceID"),
            },
            ArgumentDefinition {
                name: "StateVariableValuePairs",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_StateVariableValuePairs"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
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
        let state_var_pairs = args
            .get("StateVariableValuePairs")
            .ok_or(Error::ArgumentValueInvalid)?;
        let _service_type = args.get("ServiceType").unwrap_or("AVTransport:3");
        let _service_id = args.get("ServiceID").unwrap_or("");
        let _ = (_service_type, _service_id);
        let input = SetStateVariablesInput {
            instance_id,
            pairs: vec![(state_var_pairs.to_string(), _service_type.to_string())],
        };
        self.trait_impl
            .set_state_variables(input)
            .map_err(|_| Error::ActionFailed)?;
        Ok(ActionArgs::new())
    }
}

pub struct ActionGetSyncOffset<T: GetSyncOffset> {
    trait_impl: Arc<T>,
}

impl<T: GetSyncOffset> ActionGetSyncOffset<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetSyncOffset> Action for ActionGetSyncOffset<T> {
    fn name(&self) -> &'static str {
        "GetSyncOffset"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "SyncOffset",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("SyncOffset"),
        }];
        &ARGS
    }

    fn execute(&self, _args: &ActionArgs) -> Result<ActionArgs, Error> {
        let output = self
            .trait_impl
            .get_sync_offset()
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set("SyncOffset".to_string(), output.sync_offset);
        Ok(out)
    }
}

pub struct ActionSetSyncOffset<T: SetSyncOffset> {
    trait_impl: Arc<T>,
}

impl<T: SetSyncOffset> ActionSetSyncOffset<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: SetSyncOffset> Action for ActionSetSyncOffset<T> {
    fn name(&self) -> &'static str {
        "SetSyncOffset"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "SyncOffset",
            direction: ArgumentDirection::IN,
            related_state_var: Some("SyncOffset"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let sync_offset = args
            .get("SyncOffset")
            .ok_or(Error::ArgumentValueInvalid)?
            .to_string();
        let input = SetSyncOffsetInput { sync_offset };
        self.trait_impl
            .set_sync_offset(input)
            .map_err(|_| Error::ActionFailed)?;
        Ok(ActionArgs::new())
    }
}

pub struct ActionAdjustSyncOffset<T: AdjustSyncOffset> {
    trait_impl: Arc<T>,
}

impl<T: AdjustSyncOffset> ActionAdjustSyncOffset<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: AdjustSyncOffset> Action for ActionAdjustSyncOffset<T> {
    fn name(&self) -> &'static str {
        "AdjustSyncOffset"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "SyncOffsetAdj",
                direction: ArgumentDirection::IN,
                related_state_var: Some("SyncOffsetAdj"),
            },
            ArgumentDefinition {
                name: "SyncPoint",
                direction: ArgumentDirection::IN,
                related_state_var: Some("SyncPoint"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let sync_offset_adj = args
            .get("SyncOffsetAdj")
            .ok_or(Error::ArgumentValueInvalid)?;
        let adj: i32 = sync_offset_adj.parse().unwrap_or(0);
        let sync_point = args.get("SyncPoint").unwrap_or("0").to_string();
        let input = AdjustSyncOffsetInput {
            sync_offset_adj: adj,
            sync_point,
        };
        self.trait_impl
            .adjust_sync_offset(input)
            .map_err(|_| Error::ActionFailed)?;
        Ok(ActionArgs::new())
    }
}

pub struct ActionSyncPlay<T: SyncPlay> {
    trait_impl: Arc<T>,
}

impl<T: SyncPlay> ActionSyncPlay<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: SyncPlay> Action for ActionSyncPlay<T> {
    fn name(&self) -> &'static str {
        "SyncPlay"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, _args: &ActionArgs) -> Result<ActionArgs, Error> {
        self.trait_impl
            .sync_play()
            .map_err(|_| Error::ActionFailed)?;
        Ok(ActionArgs::new())
    }
}

pub struct ActionSyncStop<T: SyncStop> {
    trait_impl: Arc<T>,
}

impl<T: SyncStop> ActionSyncStop<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: SyncStop> Action for ActionSyncStop<T> {
    fn name(&self) -> &'static str {
        "SyncStop"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, _args: &ActionArgs) -> Result<ActionArgs, Error> {
        self.trait_impl
            .sync_stop()
            .map_err(|_| Error::ActionFailed)?;
        Ok(ActionArgs::new())
    }
}

pub struct ActionSyncPause<T: SyncPause> {
    trait_impl: Arc<T>,
}

impl<T: SyncPause> ActionSyncPause<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: SyncPause> Action for ActionSyncPause<T> {
    fn name(&self) -> &'static str {
        "SyncPause"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        &[]
    }

    fn execute(&self, _args: &ActionArgs) -> Result<ActionArgs, Error> {
        self.trait_impl
            .sync_pause()
            .map_err(|_| Error::ActionFailed)?;
        Ok(ActionArgs::new())
    }
}

pub struct ActionSetStaticPlaylist<T: SetStaticPlaylist> {
    trait_impl: Arc<T>,
}

impl<T: SetStaticPlaylist> ActionSetStaticPlaylist<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: SetStaticPlaylist> Action for ActionSetStaticPlaylist<T> {
    fn name(&self) -> &'static str {
        "SetStaticPlaylist"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            ArgumentDefinition {
                name: "PlaylistData",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistData"),
            },
            ArgumentDefinition {
                name: "PlaylistDataLength",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistDataLength"),
            },
            ArgumentDefinition {
                name: "PlaylistOffset",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistOffset"),
            },
            ArgumentDefinition {
                name: "PlaylistTotalLength",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistTotalLength"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "PlaylistDataLength",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_PlaylistDataLength"),
            },
            ArgumentDefinition {
                name: "PlaylistOffset",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_PlaylistOffset"),
            },
            ArgumentDefinition {
                name: "PlaylistTotalLength",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_PlaylistTotalLength"),
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
        let playlist_data = args
            .get("PlaylistData")
            .ok_or(Error::ArgumentValueInvalid)?
            .to_string();
        let playlist_data_length = args
            .get("PlaylistDataLength")
            .ok_or(Error::ArgumentValueInvalid)?
            .parse::<u32>()
            .unwrap_or(0);
        let playlist_offset = args
            .get("PlaylistOffset")
            .ok_or(Error::ArgumentValueInvalid)?
            .parse::<u32>()
            .unwrap_or(0);
        let playlist_total_length = args
            .get("PlaylistTotalLength")
            .ok_or(Error::ArgumentValueInvalid)?
            .parse::<u32>()
            .unwrap_or(0);
        let input = SetStaticPlaylistInput {
            instance_id,
            playlist_data,
            playlist_data_length,
            playlist_offset,
            playlist_total_length,
        };
        let output = self
            .trait_impl
            .set_static_playlist(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set(
            "PlaylistDataLength".to_string(),
            output.playlist_data_length.to_string(),
        );
        out.set(
            "PlaylistOffset".to_string(),
            output.playlist_offset.to_string(),
        );
        out.set(
            "PlaylistTotalLength".to_string(),
            output.playlist_total_length.to_string(),
        );
        Ok(out)
    }
}

pub struct ActionSetStreamingPlaylist<T: SetStreamingPlaylist> {
    trait_impl: Arc<T>,
}

impl<T: SetStreamingPlaylist> ActionSetStreamingPlaylist<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: SetStreamingPlaylist> Action for ActionSetStreamingPlaylist<T> {
    fn name(&self) -> &'static str {
        "SetStreamingPlaylist"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            ArgumentDefinition {
                name: "PlaylistData",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistData"),
            },
            ArgumentDefinition {
                name: "PlaylistDataLength",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistDataLength"),
            },
            ArgumentDefinition {
                name: "PlaylistOffset",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistOffset"),
            },
            ArgumentDefinition {
                name: "PlaylistTotalLength",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistTotalLength"),
            },
            ArgumentDefinition {
                name: "PlaylistMIMEType",
                direction: ArgumentDirection::IN,
                related_state_var: Some("PlaylistMIMEType"),
            },
            ArgumentDefinition {
                name: "PlaylistExtendedType",
                direction: ArgumentDirection::IN,
                related_state_var: Some("PlaylistExtendedType"),
            },
            ArgumentDefinition {
                name: "PlaylistStep",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistStep"),
            },
            ArgumentDefinition {
                name: "PlaylistType",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistType"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "PlaylistDataLength",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_PlaylistDataLength"),
            },
            ArgumentDefinition {
                name: "PlaylistOffset",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_PlaylistOffset"),
            },
            ArgumentDefinition {
                name: "PlaylistTotalLength",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_PlaylistTotalLength"),
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
        let playlist_data = args
            .get("PlaylistData")
            .ok_or(Error::ArgumentValueInvalid)?
            .to_string();
        let playlist_data_length = args
            .get("PlaylistDataLength")
            .ok_or(Error::ArgumentValueInvalid)?
            .parse::<u32>()
            .unwrap_or(0);
        let playlist_offset = args
            .get("PlaylistOffset")
            .ok_or(Error::ArgumentValueInvalid)?
            .parse::<u32>()
            .unwrap_or(0);
        let playlist_total_length = args
            .get("PlaylistTotalLength")
            .ok_or(Error::ArgumentValueInvalid)?
            .parse::<u32>()
            .unwrap_or(0);
        let mime_type = args.get("PlaylistMIMEType").unwrap_or("").to_string();
        let extended_type = args.get("PlaylistExtendedType").unwrap_or("").to_string();
        let step = args.get("PlaylistStep").unwrap_or("").to_string();
        let playlist_type = args.get("PlaylistType").unwrap_or("").to_string();
        let input = SetStreamingPlaylistInput {
            instance_id,
            playlist_data,
            playlist_data_length,
            playlist_offset,
            playlist_total_length,
            mime_type,
            extended_type,
            step,
            playlist_type,
        };
        let output = self
            .trait_impl
            .set_streaming_playlist(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set(
            "PlaylistDataLength".to_string(),
            output.playlist_data_length.to_string(),
        );
        out.set(
            "PlaylistOffset".to_string(),
            output.playlist_offset.to_string(),
        );
        out.set(
            "PlaylistTotalLength".to_string(),
            output.playlist_total_length.to_string(),
        );
        Ok(out)
    }
}

pub struct ActionGetPlaylistInfo<T: GetPlaylistInfo> {
    trait_impl: Arc<T>,
}

impl<T: GetPlaylistInfo> ActionGetPlaylistInfo<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetPlaylistInfo> Action for ActionGetPlaylistInfo<T> {
    fn name(&self) -> &'static str {
        "GetPlaylistInfo"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "PlaylistData",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaylistData"),
            },
            ArgumentDefinition {
                name: "PlaylistDataLength",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaylistDataLength"),
            },
            ArgumentDefinition {
                name: "PlaylistOffset",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaylistOffset"),
            },
            ArgumentDefinition {
                name: "PlaylistTotalLength",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaylistTotalLength"),
            },
            ArgumentDefinition {
                name: "PlaylistMIMEType",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaylistMIMEType"),
            },
            ArgumentDefinition {
                name: "PlaylistExtendedType",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaylistExtendedType"),
            },
            ArgumentDefinition {
                name: "PlaylistStep",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaylistStep"),
            },
            ArgumentDefinition {
                name: "PlaylistType",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaylistType"),
            },
            ArgumentDefinition {
                name: "PlaylistInfo",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaylistInfo"),
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
        let input = GetMediaInfoInput { instance_id };
        let output = self
            .trait_impl
            .get_playlist_info(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set("PlaylistData".to_string(), output.playlist_data);
        out.set(
            "PlaylistDataLength".to_string(),
            output.playlist_data_length.to_string(),
        );
        out.set(
            "PlaylistOffset".to_string(),
            output.playlist_offset.to_string(),
        );
        out.set(
            "PlaylistTotalLength".to_string(),
            output.playlist_total_length.to_string(),
        );
        out.set("PlaylistMIMEType".to_string(), output.playlist_mime_type);
        out.set(
            "PlaylistExtendedType".to_string(),
            output.playlist_extended_type,
        );
        out.set("PlaylistStep".to_string(), output.playlist_step);
        out.set("PlaylistType".to_string(), output.playlist_type);
        out.set("PlaylistInfo".to_string(), output.playlist_info);
        Ok(out)
    }
}

pub struct ActionGetMediaInfoExt<T: GetMediaInfoExt> {
    trait_impl: Arc<T>,
}

impl<T: GetMediaInfoExt> ActionGetMediaInfoExt<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetMediaInfoExt> Action for ActionGetMediaInfoExt<T> {
    fn name(&self) -> &'static str {
        "GetMediaInfo_Ext"
    }

    fn in_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[ArgumentDefinition {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[ArgumentDefinition<&'static str, &'static str>] {
        static ARGS: &[ArgumentDefinition<&'static str, &'static str>] = &[
            ArgumentDefinition {
                name: "NrTracks",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("NumberOfTracks"),
            },
            ArgumentDefinition {
                name: "MediaDuration",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentMediaDuration"),
            },
            ArgumentDefinition {
                name: "CurrentURI",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("AVTransportURI"),
            },
            ArgumentDefinition {
                name: "CurrentURIMetaData",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("AVTransportURIMetaData"),
            },
            ArgumentDefinition {
                name: "NextURI",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("NextAVTransportURI"),
            },
            ArgumentDefinition {
                name: "NextURIMetaData",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("NextAVTransportURIMetaData"),
            },
            ArgumentDefinition {
                name: "PlayMedium",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaybackStorageMedium"),
            },
            ArgumentDefinition {
                name: "RecordMedium",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("RecordStorageMedium"),
            },
            ArgumentDefinition {
                name: "WriteStatus",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("RecordMediumWriteStatus"),
            },
            ArgumentDefinition {
                name: "CurrentTrackMetaData",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentTrackMetaData"),
            },
            ArgumentDefinition {
                name: "TrackDuration",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentTrackDuration"),
            },
            ArgumentDefinition {
                name: "TrackURI",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentTrackURI"),
            },
            ArgumentDefinition {
                name: "RelTime",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("RelativeTimePosition"),
            },
            ArgumentDefinition {
                name: "AbsTime",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("AbsoluteTimePosition"),
            },
            ArgumentDefinition {
                name: "RelCount",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("RelativeCounterPosition"),
            },
            ArgumentDefinition {
                name: "AbsCount",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("AbsoluteCounterPosition"),
            },
            ArgumentDefinition {
                name: "TransportState",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("TransportState"),
            },
            ArgumentDefinition {
                name: "TransportStatus",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("TransportStatus"),
            },
            ArgumentDefinition {
                name: "TransportSpeed",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("TransportPlaySpeed"),
            },
            ArgumentDefinition {
                name: "TrackNavEnabled",
                direction: ArgumentDirection::OUT,
                related_state_var: None,
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
        let input = GetMediaInfoInput { instance_id };
        let output = self
            .trait_impl
            .get_media_info_ext(input)
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set("NrTracks".to_string(), output.nr_tracks.to_string());
        out.set("MediaDuration".to_string(), output.media_duration);
        out.set("CurrentURI".to_string(), output.current_uri);
        out.set(
            "CurrentURIMetaData".to_string(),
            output.current_uri_metadata,
        );
        out.set("NextURI".to_string(), output.next_uri);
        out.set("NextURIMetaData".to_string(), output.next_uri_metadata);
        out.set(
            "PlayMedium".to_string(),
            output.play_medium.as_str().to_string(),
        );
        out.set(
            "RecordMedium".to_string(),
            output.record_medium.as_str().to_string(),
        );
        out.set(
            "WriteStatus".to_string(),
            output.write_status.as_str().to_string(),
        );
        out.set(
            "CurrentTrackMetaData".to_string(),
            output.current_track_metadata,
        );
        out.set("TrackDuration".to_string(), output.track_duration);
        out.set("TrackURI".to_string(), output.track_uri);
        out.set("RelTime".to_string(), output.rel_time);
        out.set("AbsTime".to_string(), output.abs_time);
        out.set("RelCount".to_string(), output.rel_count.to_string());
        out.set("AbsCount".to_string(), output.abs_count.to_string());
        out.set(
            "TransportState".to_string(),
            output.transport_state.as_str().to_string(),
        );
        out.set(
            "TransportStatus".to_string(),
            output.transport_status.as_str().to_string(),
        );
        out.set("TransportSpeed".to_string(), output.transport_speed);
        out.set(
            "TrackNavEnabled".to_string(),
            if output.track_nav_enabled {
                "1".to_string()
            } else {
                "0".to_string()
            },
        );
        Ok(out)
    }
}
