/// AVTransport service — bridge from SOAP `Action` trait to application traits.
///
/// Each struct wraps an Arc<T> where T implements the application trait.
/// The `execute` method extracts typed args, calls the trait, and wraps results.
///
/// Bridge structs are prefixed with `Action` to avoid name collision with traits.
use std::sync::Arc;

use super::AvTransportService;
use super::r#static::{PlayMode, StateVariableName};
use crate::services::avtransport::traits::*;
use crate::types::upnp::{
    Action, ActionArgs, Argument, ArgumentDirection, Error, StateValue,
};

// ===========================================================================
// Required Actions (R)
// ===========================================================================

pub struct ActionPlay<T: Play> {
    trait_impl: Arc<T>,
    service: AvTransportService,
}

impl<T: Play> ActionPlay<T> {
    pub fn new(trait_impl: T, service: AvTransportService) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            service,
        }
    }
}

impl<T: Play> Action for ActionPlay<T> {
    fn name(&self) -> &'static str {
        "Play"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "Speed",
                direction: ArgumentDirection::IN,
                related_state_var: Some("TransportPlaySpeed"),
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
        let speed = args.get("Speed").unwrap_or("1").to_string();
        let input = PlayInput {
            instance_id,
            speed: speed.clone(),
        };
        self.trait_impl
            .play(input)
            .map_err(|_| Error::ActionFailed)?;

        // Update state variables per spec §Play state effects
        self.service.set_state_var(
            instance_id,
            StateVariableName::TransportState,
            StateValue::String("PLAYING".to_string()),
        );
        self.service.set_state_var(
            instance_id,
            StateVariableName::TransportPlaySpeed,
            StateValue::String(speed),
        );

        Ok(ActionArgs::new())
    }
}

pub struct ActionStop<T: Stop> {
    trait_impl: Arc<T>,
    service: AvTransportService,
}

impl<T: Stop> ActionStop<T> {
    pub fn new(trait_impl: T, service: AvTransportService) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            service,
        }
    }
}

impl<T: Stop> Action for ActionStop<T> {
    fn name(&self) -> &'static str {
        "Stop"
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
        self.service.set_state_var(
            instance_id,
            StateVariableName::TransportState,
            StateValue::String("STOPPED".to_string()),
        );

        Ok(ActionArgs::new())
    }
}

pub struct ActionSeek<T: Seek> {
    trait_impl: Arc<T>,
    service: AvTransportService,
}

impl<T: Seek> ActionSeek<T> {
    pub fn new(trait_impl: T, service: AvTransportService) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            service,
        }
    }
}

impl<T: Seek> Action for ActionSeek<T> {
    fn name(&self) -> &'static str {
        "Seek"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "Unit",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_SeekMode"),
            },
            Argument {
                name: "Target",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_SeekTarget"),
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
        self.service.set_state_var(
            instance_id,
            StateVariableName::TransportState,
            StateValue::String("TRANSITIONING".to_string()),
        );

        Ok(ActionArgs::new())
    }
}

pub struct ActionNext<T: Next> {
    trait_impl: Arc<T>,
    service: AvTransportService,
}

impl<T: Next> ActionNext<T> {
    pub fn new(trait_impl: T, service: AvTransportService) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            service,
        }
    }
}

impl<T: Next> Action for ActionNext<T> {
    fn name(&self) -> &'static str {
        "Next"
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
        self.service.set_state_var(
            instance_id,
            StateVariableName::TransportState,
            StateValue::String("TRANSITIONING".to_string()),
        );

        Ok(ActionArgs::new())
    }
}

pub struct ActionPrevious<T: Previous> {
    trait_impl: Arc<T>,
    service: AvTransportService,
}

impl<T: Previous> ActionPrevious<T> {
    pub fn new(trait_impl: T, service: AvTransportService) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            service,
        }
    }
}

impl<T: Previous> Action for ActionPrevious<T> {
    fn name(&self) -> &'static str {
        "Previous"
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
        self.service.set_state_var(
            instance_id,
            StateVariableName::TransportState,
            StateValue::String("TRANSITIONING".to_string()),
        );

        Ok(ActionArgs::new())
    }
}

pub struct ActionSetAVTransportURI<T: SetAVTransportURI> {
    trait_impl: Arc<T>,
    service: AvTransportService,
}

impl<T: SetAVTransportURI> ActionSetAVTransportURI<T> {
    pub fn new(trait_impl: T, service: AvTransportService) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            service,
        }
    }
}

impl<T: SetAVTransportURI> Action for ActionSetAVTransportURI<T> {
    fn name(&self) -> &'static str {
        "SetAVTransportURI"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "CurrentURI",
                direction: ArgumentDirection::IN,
                related_state_var: Some("AVTransportURI"),
            },
            Argument {
                name: "CurrentURIMetaData",
                direction: ArgumentDirection::IN,
                related_state_var: Some("AVTransportURIMetaData"),
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
        self.trait_impl
            .set_av_transport_uri(input)
            .map_err(|_| Error::ActionFailed)?;

        // Update state variables per spec §SetAVTransportURI state effects
        {
            let medium = if current_uri.starts_with("http") {
                "NETWORK"
            } else if current_uri.starts_with("file://") {
                "HDD"
            } else {
                "UNKNOWN"
            };

            self.service.set_state_var(
                instance_id,
                StateVariableName::AVTransportURI,
                StateValue::Uri(current_uri),
            );
            self.service.set_state_var(
                instance_id,
                StateVariableName::AVTransportURIMetaData,
                StateValue::String(current_uri_metadata),
            );
            // PlaybackStorageMedium and CurrentMediaCategory derived from URI/number of tracks
            // are updated by the application bridge via set_state_var calls after Load completes.
        }

        Ok(ActionArgs::new())
    }
}

pub struct ActionGetMediaInfo<T: GetMediaInfo> {
    trait_impl: Arc<T>,
}

impl<T: GetMediaInfo> ActionGetMediaInfo<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetMediaInfo> Action for ActionGetMediaInfo<T> {
    fn name(&self) -> &'static str {
        "GetMediaInfo"
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
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "NrTracks",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("NumberOfTracks"),
            },
            Argument {
                name: "MediaDuration",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentMediaDuration"),
            },
            Argument {
                name: "CurrentURI",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("AVTransportURI"),
            },
            Argument {
                name: "CurrentURIMetaData",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("AVTransportURIMetaData"),
            },
            Argument {
                name: "NextURI",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("NextAVTransportURI"),
            },
            Argument {
                name: "NextURIMetaData",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("NextAVTransportURIMetaData"),
            },
            Argument {
                name: "PlayMedium",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaybackStorageMedium"),
            },
            Argument {
                name: "RecordMedium",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("RecordStorageMedium"),
            },
            Argument {
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
}

impl<T: GetTransportInfo> ActionGetTransportInfo<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetTransportInfo> Action for ActionGetTransportInfo<T> {
    fn name(&self) -> &'static str {
        "GetTransportInfo"
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
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "CurrentTransportState",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("TransportState"),
            },
            Argument {
                name: "CurrentTransportStatus",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("TransportStatus"),
            },
            Argument {
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
}

impl<T: GetPositionInfo> ActionGetPositionInfo<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetPositionInfo> Action for ActionGetPositionInfo<T> {
    fn name(&self) -> &'static str {
        "GetPositionInfo"
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
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "Track",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentTrack"),
            },
            Argument {
                name: "TrackDuration",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentTrackDuration"),
            },
            Argument {
                name: "TrackMetaData",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentTrackMetaData"),
            },
            Argument {
                name: "TrackURI",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentTrackURI"),
            },
            Argument {
                name: "RelTime",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("RelativeTimePosition"),
            },
            Argument {
                name: "AbsTime",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("AbsoluteTimePosition"),
            },
            Argument {
                name: "RelCount",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("RelativeCounterPosition"),
            },
            Argument {
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
}

impl<T: GetDeviceCapabilities> ActionGetDeviceCapabilities<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetDeviceCapabilities> Action for ActionGetDeviceCapabilities<T> {
    fn name(&self) -> &'static str {
        "GetDeviceCapabilities"
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
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "PlayMedia",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PossiblePlaybackStorageMedia"),
            },
            Argument {
                name: "RecMedia",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PossibleRecordStorageMedia"),
            },
            Argument {
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
}

impl<T: GetTransportSettings> ActionGetTransportSettings<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetTransportSettings> Action for ActionGetTransportSettings<T> {
    fn name(&self) -> &'static str {
        "GetTransportSettings"
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
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "PlayMode",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentPlayMode"),
            },
            Argument {
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
}

impl<T: SetNextAVTransportURI> ActionSetNextAVTransportURI<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: SetNextAVTransportURI> Action for ActionSetNextAVTransportURI<T> {
    fn name(&self) -> &'static str {
        "SetNextAVTransportURI"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "NextURI",
                direction: ArgumentDirection::IN,
                related_state_var: Some("NextAVTransportURI"),
            },
            Argument {
                name: "NextURIMetaData",
                direction: ArgumentDirection::IN,
                related_state_var: Some("NextAVTransportURIMetaData"),
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
        self.trait_impl
            .set_next_av_transport_uri(input)
            .map_err(|_| Error::ActionFailed)?;
        Ok(ActionArgs::new())
    }
}

pub struct ActionPause<T: Pause> {
    trait_impl: Arc<T>,
    service: AvTransportService,
}

impl<T: Pause> ActionPause<T> {
    pub fn new(trait_impl: T, service: AvTransportService) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            service,
        }
    }
}

impl<T: Pause> Action for ActionPause<T> {
    fn name(&self) -> &'static str {
        "Pause"
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
        self.service.set_state_var(
            instance_id,
            StateVariableName::TransportState,
            StateValue::String("PAUSED_PLAYBACK".to_string()),
        );

        Ok(ActionArgs::new())
    }
}

pub struct ActionSetPlayMode<T: SetPlayMode> {
    trait_impl: Arc<T>,
    service: AvTransportService,
}

impl<T: SetPlayMode> ActionSetPlayMode<T> {
    pub fn new(trait_impl: T, service: AvTransportService) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
            service,
        }
    }
}

impl<T: SetPlayMode> Action for ActionSetPlayMode<T> {
    fn name(&self) -> &'static str {
        "SetPlayMode"
    }

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "NewPlayMode",
                direction: ArgumentDirection::IN,
                related_state_var: Some("CurrentPlayMode"),
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
        self.service.set_state_var(
            instance_id,
            StateVariableName::CurrentPlayMode,
            StateValue::String(play_mode.as_str().to_string()),
        );

        Ok(ActionArgs::new())
    }
}

pub struct ActionGetCurrentTransportActions<T: GetCurrentTransportActions> {
    trait_impl: Arc<T>,
}

impl<T: GetCurrentTransportActions> ActionGetCurrentTransportActions<T> {
    pub fn new(trait_impl: T) -> Self {
        Self {
            trait_impl: Arc::new(trait_impl),
        }
    }
}

impl<T: GetCurrentTransportActions> Action for ActionGetCurrentTransportActions<T> {
    fn name(&self) -> &'static str {
        "GetCurrentTransportActions"
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

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
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

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "NewRecordQualityMode",
                direction: ArgumentDirection::IN,
                related_state_var: Some("CurrentRecordQualityMode"),
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

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
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

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "AVTransportUDN",
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
            name: "CurrentSyncOffset",
            direction: ArgumentDirection::OUT,
            related_state_var: Some("SyncOffset"),
        }];
        &ARGS
    }

    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error> {
        let _instance_id = args
            .get("InstanceID")
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        let output = self
            .trait_impl
            .get_sync_offset()
            .map_err(|_| Error::ActionFailed)?;
        let mut out = ActionArgs::new();
        out.set("CurrentSyncOffset".to_string(), output.sync_offset);
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

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "NewSyncOffset",
                direction: ArgumentDirection::IN,
                related_state_var: Some("SyncOffset"),
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
        let new_sync_offset = args
            .get("NewSyncOffset")
            .ok_or(Error::ArgumentValueInvalid)?
            .to_string();
        let input = SetSyncOffsetInput {
            instance_id,
            new_sync_offset: new_sync_offset,
        };
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

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "Adjustment",
                direction: ArgumentDirection::IN,
                related_state_var: Some("SyncOffset"),
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
        let adjustment = args.get("Adjustment").ok_or(Error::ArgumentValueInvalid)?;
        let adj: i32 = adjustment.parse().unwrap_or(0);
        let sync_point = args.get("SyncPoint").unwrap_or("0").to_string();
        let input = AdjustSyncOffsetInput {
            instance_id,
            adjustment: adj,
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

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
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

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
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

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        &[]
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
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

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "PlaylistData",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistData"),
            },
            Argument {
                name: "PlaylistDataLength",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistDataLength"),
            },
            Argument {
                name: "PlaylistOffset",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistOffset"),
            },
            Argument {
                name: "PlaylistTotalLength",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistTotalLength"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "PlaylistDataLength",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_PlaylistDataLength"),
            },
            Argument {
                name: "PlaylistOffset",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_PlaylistOffset"),
            },
            Argument {
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

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "InstanceID",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_InstanceID"),
            },
            Argument {
                name: "PlaylistData",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistData"),
            },
            Argument {
                name: "PlaylistDataLength",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistDataLength"),
            },
            Argument {
                name: "PlaylistOffset",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistOffset"),
            },
            Argument {
                name: "PlaylistTotalLength",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistTotalLength"),
            },
            Argument {
                name: "PlaylistMIMEType",
                direction: ArgumentDirection::IN,
                related_state_var: Some("PlaylistMIMEType"),
            },
            Argument {
                name: "PlaylistExtendedType",
                direction: ArgumentDirection::IN,
                related_state_var: Some("PlaylistExtendedType"),
            },
            Argument {
                name: "PlaylistStep",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistStep"),
            },
            Argument {
                name: "PlaylistType",
                direction: ArgumentDirection::IN,
                related_state_var: Some("A_ARG_TYPE_PlaylistType"),
            },
        ];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "PlaylistDataLength",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_PlaylistDataLength"),
            },
            Argument {
                name: "PlaylistOffset",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("A_ARG_TYPE_PlaylistOffset"),
            },
            Argument {
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

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "PlaylistData",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaylistData"),
            },
            Argument {
                name: "PlaylistDataLength",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaylistDataLength"),
            },
            Argument {
                name: "PlaylistOffset",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaylistOffset"),
            },
            Argument {
                name: "PlaylistTotalLength",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaylistTotalLength"),
            },
            Argument {
                name: "PlaylistMIMEType",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaylistMIMEType"),
            },
            Argument {
                name: "PlaylistExtendedType",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaylistExtendedType"),
            },
            Argument {
                name: "PlaylistStep",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaylistStep"),
            },
            Argument {
                name: "PlaylistType",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaylistType"),
            },
            Argument {
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

    fn in_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[Argument {
            name: "InstanceID",
            direction: ArgumentDirection::IN,
            related_state_var: Some("A_ARG_TYPE_InstanceID"),
        }];
        &ARGS
    }

    fn out_args(&self) -> &[Argument<&'static str, &'static str>] {
        static ARGS: &[Argument<&'static str, &'static str>] = &[
            Argument {
                name: "NrTracks",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("NumberOfTracks"),
            },
            Argument {
                name: "MediaDuration",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentMediaDuration"),
            },
            Argument {
                name: "CurrentURI",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("AVTransportURI"),
            },
            Argument {
                name: "CurrentURIMetaData",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("AVTransportURIMetaData"),
            },
            Argument {
                name: "NextURI",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("NextAVTransportURI"),
            },
            Argument {
                name: "NextURIMetaData",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("NextAVTransportURIMetaData"),
            },
            Argument {
                name: "PlayMedium",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("PlaybackStorageMedium"),
            },
            Argument {
                name: "RecordMedium",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("RecordStorageMedium"),
            },
            Argument {
                name: "WriteStatus",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("RecordMediumWriteStatus"),
            },
            Argument {
                name: "CurrentTrackMetaData",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentTrackMetaData"),
            },
            Argument {
                name: "TrackDuration",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentTrackDuration"),
            },
            Argument {
                name: "TrackURI",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("CurrentTrackURI"),
            },
            Argument {
                name: "RelTime",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("RelativeTimePosition"),
            },
            Argument {
                name: "AbsTime",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("AbsoluteTimePosition"),
            },
            Argument {
                name: "RelCount",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("RelativeCounterPosition"),
            },
            Argument {
                name: "AbsCount",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("AbsoluteCounterPosition"),
            },
            Argument {
                name: "TransportState",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("TransportState"),
            },
            Argument {
                name: "TransportStatus",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("TransportStatus"),
            },
            Argument {
                name: "TransportSpeed",
                direction: ArgumentDirection::OUT,
                related_state_var: Some("TransportPlaySpeed"),
            },
            Argument {
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
