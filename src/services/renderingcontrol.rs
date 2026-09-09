pub mod actions;
pub mod r#static;
pub mod traits;

pub use r#static::{ActionName, ArgumentName, Error, StateVariableName};
pub use traits::*;

use crate::gena::EventPublisher;
use crate::{
    renderingcontrol::r#static::V3,
    types::upnp::{Action, ActionMap, Services, StateSchema, StateStore, StateValue},
};
use std::sync::Arc;

/// RenderingControl service — owns action registry, state variables, and eventing.
///
/// The application registers trait implementations (GetVolumeAction, SetMuteAction, etc.)
/// via `register_action()`. The SOAP handler dispatches incoming requests
/// to the correct trait implementation.
///
/// State variables are initialized in `init_state_vars()` and owned by this struct.
/// When state changes via `set_state_var()`, the service automatically triggers GENA events.
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
            actions: Arc::new(std::sync::Mutex::new(ActionMap::new(Services::RenderingControl(V3)))),
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
        use r#static::StateVariableName;

        // LastChange — only evented variable
        state_store.register(StateSchema {
            name: StateVariableName::LastChange,
            data_type: crate::types::upnp::DataType::String,
            send_events: true,
            default: StateValue::String(String::new()),
            ..Default::default()
        });

        // PresetNameList
        state_store.register(StateSchema {
            name: StateVariableName::PresetNameList,
            data_type: crate::types::upnp::DataType::String,
            send_events: false,
            default: StateValue::String("FactoryDefaults".to_string()),
            ..Default::default()
        });

        // Brightness — ui2 [0..255]
        state_store.register(StateSchema {
            name: StateVariableName::Brightness,
            data_type: crate::types::upnp::DataType::UnsignedShort,
            send_events: false,
            default: StateValue::Ui2(128),
            allowed_value_range: Some(("0".to_string(), "255".to_string(), "1".to_string())),
            ..Default::default()
        });

        // Contrast — ui2 [0..255]
        state_store.register(StateSchema {
            name: StateVariableName::Contrast,
            data_type: crate::types::upnp::DataType::UnsignedShort,
            send_events: false,
            default: StateValue::Ui2(128),
            allowed_value_range: Some(("0".to_string(), "255".to_string(), "1".to_string())),
            ..Default::default()
        });

        // Sharpness — ui2 [0..255]
        state_store.register(StateSchema {
            name: StateVariableName::Sharpness,
            data_type: crate::types::upnp::DataType::UnsignedShort,
            send_events: false,
            default: StateValue::Ui2(128),
            allowed_value_range: Some(("0".to_string(), "255".to_string(), "1".to_string())),
            ..Default::default()
        });

        // RedVideoGain — ui2 [0..vendor-max]
        state_store.register(StateSchema {
            name: StateVariableName::RedVideoGain,
            data_type: crate::types::upnp::DataType::UnsignedShort,
            send_events: false,
            default: StateValue::Ui2(0),
            allowed_value_range: Some(("0".to_string(), "65535".to_string(), "1".to_string())),
            ..Default::default()
        });

        // GreenVideoGain — ui2 [0..vendor-max]
        state_store.register(StateSchema {
            name: StateVariableName::GreenVideoGain,
            data_type: crate::types::upnp::DataType::UnsignedShort,
            send_events: false,
            default: StateValue::Ui2(0),
            allowed_value_range: Some(("0".to_string(), "65535".to_string(), "1".to_string())),
            ..Default::default()
        });

        // BlueVideoGain — ui2 [0..vendor-max]
        state_store.register(StateSchema {
            name: StateVariableName::BlueVideoGain,
            data_type: crate::types::upnp::DataType::UnsignedShort,
            send_events: false,
            default: StateValue::Ui2(0),
            allowed_value_range: Some(("0".to_string(), "65535".to_string(), "1".to_string())),
            ..Default::default()
        });

        // RedVideoBlackLevel — ui2 [0..vendor-max]
        state_store.register(StateSchema {
            name: StateVariableName::RedVideoBlackLevel,
            data_type: crate::types::upnp::DataType::UnsignedShort,
            send_events: false,
            default: StateValue::Ui2(0),
            allowed_value_range: Some(("0".to_string(), "65535".to_string(), "1".to_string())),
            ..Default::default()
        });

        // GreenVideoBlackLevel — ui2 [0..vendor-max]
        state_store.register(StateSchema {
            name: StateVariableName::GreenVideoBlackLevel,
            data_type: crate::types::upnp::DataType::UnsignedShort,
            send_events: false,
            default: StateValue::Ui2(0),
            allowed_value_range: Some(("0".to_string(), "65535".to_string(), "1".to_string())),
            ..Default::default()
        });

        // BlueVideoBlackLevel — ui2 [0..vendor-max]
        state_store.register(StateSchema {
            name: StateVariableName::BlueVideoBlackLevel,
            data_type: crate::types::upnp::DataType::UnsignedShort,
            send_events: false,
            default: StateValue::Ui2(0),
            allowed_value_range: Some(("0".to_string(), "65535".to_string(), "1".to_string())),
            ..Default::default()
        });

        // ColorTemperature — ui2 [0..vendor-max]
        state_store.register(StateSchema {
            name: StateVariableName::ColorTemperature,
            data_type: crate::types::upnp::DataType::UnsignedShort,
            send_events: false,
            default: StateValue::Ui2(0),
            allowed_value_range: Some(("0".to_string(), "65535".to_string(), "1".to_string())),
            ..Default::default()
        });

        // HorizontalKeystone — i2 [vendor-min(≤0)..vendor-max]
        state_store.register(StateSchema {
            name: StateVariableName::HorizontalKeystone,
            data_type: crate::types::upnp::DataType::Short,
            send_events: false,
            default: StateValue::I2(0),
            allowed_value_range: Some(("-32768".to_string(), "32767".to_string(), "1".to_string())),
            ..Default::default()
        });

        // VerticalKeystone — i2 [vendor-min(≤0)..vendor-max]
        state_store.register(StateSchema {
            name: StateVariableName::VerticalKeystone,
            data_type: crate::types::upnp::DataType::Short,
            send_events: false,
            default: StateValue::I2(0),
            allowed_value_range: Some(("-32768".to_string(), "32767".to_string(), "1".to_string())),
            ..Default::default()
        });

        // Mute — boolean
        state_store.register(StateSchema {
            name: StateVariableName::Mute,
            data_type: crate::types::upnp::DataType::Boolean,
            send_events: false,
            default: StateValue::Boolean(false),
            ..Default::default()
        });

        // Volume — ui2 [0..vendor-max]
        state_store.register(StateSchema {
            name: StateVariableName::Volume,
            data_type: crate::types::upnp::DataType::UnsignedShort,
            send_events: false,
            default: StateValue::Ui2(50),
            allowed_value_range: Some(("0".to_string(), "65535".to_string(), "1".to_string())),
            ..Default::default()
        });

        // VolumeDB — i2 (signed, 1/256 dB resolution, range +127.9961 to -127.9961 dB)
        // Integer value has decimal point between MSB and LSB. Resolution = 1/256 dB.
        // 0x8000 is invalid. Example: -72 dB = 0xB800 = -29491
        state_store.register(StateSchema {
            name: StateVariableName::VolumeDB,
            data_type: crate::types::upnp::DataType::Short,
            send_events: false,
            default: StateValue::I2(0),
            allowed_value_range: Some(("-32767".to_string(), "32767".to_string(), "1".to_string())),
            ..Default::default()
        });

        // Loudness — boolean
        state_store.register(StateSchema {
            name: StateVariableName::Loudness,
            data_type: crate::types::upnp::DataType::Boolean,
            send_events: false,
            default: StateValue::Boolean(false),
            ..Default::default()
        });

        // AllowedTransformSettings — string (XML)
        state_store.register(StateSchema {
            name: StateVariableName::AllowedTransformSettings,
            data_type: crate::types::upnp::DataType::String,
            send_events: false,
            default: StateValue::String("NOT_IMPLEMENTED".to_string()),
            ..Default::default()
        });

        // TransformSettings — string (XML)
        state_store.register(StateSchema {
            name: StateVariableName::TransformSettings,
            data_type: crate::types::upnp::DataType::String,
            send_events: false,
            default: StateValue::String("NOT_IMPLEMENTED".to_string()),
            ..Default::default()
        });

        // AllowedDefaultTransformSettings — string (XML), directly evented per spec §7.1
        state_store.register(StateSchema {
            name: StateVariableName::AllowedDefaultTransformSettings,
            data_type: crate::types::upnp::DataType::String,
            send_events: true,
            default: StateValue::String("NOT_IMPLEMENTED".to_string()),
            ..Default::default()
        });

        // DefaultTransformSettings — string (XML), directly evented per spec §7.1
        state_store.register(StateSchema {
            name: StateVariableName::DefaultTransformSettings,
            data_type: crate::types::upnp::DataType::String,
            send_events: true,
            default: StateValue::String("NOT_IMPLEMENTED".to_string()),
            ..Default::default()
        });

        // =========================================================================
        // A_ARG_TYPE variables — type definitions for action arguments
        // =========================================================================

        // A_ARG_TYPE_InstanceID — ui4, virtual RCS instance
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_InstanceID,
            data_type: crate::types::upnp::DataType::UnsignedInt,
            send_events: false,
            default: StateValue::Ui4(0),
            argument_type: true,
            ..Default::default()
        });

        // A_ARG_TYPE_Channel — audio channel name
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_Channel,
            data_type: crate::types::upnp::DataType::String,
            send_events: false,
            default: StateValue::String(String::new()),
            allowed_values: Some(vec![
                "Master".to_string(),
                "LF".to_string(),
                "RF".to_string(),
                "CF".to_string(),
                "LFE".to_string(),
                "LS".to_string(),
                "RS".to_string(),
                "LFC".to_string(),
                "RFC".to_string(),
                "SD".to_string(),
                "SL".to_string(),
                "SR".to_string(),
                "T".to_string(),
                "B".to_string(),
                "BC".to_string(),
                "BL".to_string(),
                "BR".to_string(),
            ]),
            argument_type: true,
            ..Default::default()
        });

        // A_ARG_TYPE_PresetName — preset name
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_PresetName,
            data_type: crate::types::upnp::DataType::String,
            send_events: false,
            default: StateValue::String(String::new()),
            allowed_values: Some(vec![
                "FactoryDefaults".to_string(),
                "InstallationDefaults".to_string(),
            ]),
            argument_type: true,
            ..Default::default()
        });

        // A_ARG_TYPE_DeviceUDN — MediaRenderer UDN
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_DeviceUDN,
            data_type: crate::types::upnp::DataType::String,
            send_events: false,
            default: StateValue::String(String::new()),
            argument_type: true,
            ..Default::default()
        });

        // A_ARG_TYPE_ServiceType — service type string (e.g., "RenderingControl:3")
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_ServiceType,
            data_type: crate::types::upnp::DataType::String,
            send_events: false,
            default: StateValue::String(String::new()),
            argument_type: true,
            ..Default::default()
        });

        // A_ARG_TYPE_ServiceID — service ID string
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_ServiceID,
            data_type: crate::types::upnp::DataType::String,
            send_events: false,
            default: StateValue::String(String::new()),
            argument_type: true,
            ..Default::default()
        });

        // A_ARG_TYPE_StateVariableValuePairs — XML structure
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_StateVariableValuePairs,
            data_type: crate::types::upnp::DataType::String,
            send_events: false,
            default: StateValue::String(String::new()),
            argument_type: true,
            ..Default::default()
        });

        // A_ARG_TYPE_StateVariableList — CSV of state variable names
        state_store.register(StateSchema {
            name: StateVariableName::A_ARG_TYPE_StateVariableList,
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
            "urn:schemas-upnp-org:metadata-1-0/RC/",
            &evented,
        )
    }

    /// Trigger GENA events for all evented state variables.
    fn trigger_events(&mut self) {
        let last_change_xml = self.build_last_change();

        let mut store = self.state_store.lock().unwrap();
        if let Some(last_change_var) = store.get_mut(r#static::StateVariableName::LastChange) {
            last_change_var.current_value = StateValue::String(last_change_xml);
        }

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
