// ===========================================================================
// UPnP AV 2.0 — Action Primitives
// ===========================================================================
//
// Runtime execution interface for UPnP actions:
// - ActionArgs (key-value store for SOAP arguments)
// - Action trait (runtime execution interface)
// - ActionMap (runtime registry of actions)

use crate::types::argument::Argument;
use crate::types::upnp::Error;

// ---------------------------------------------------------------------------
// ActionArgs — typed key-value store for SOAP arguments
// ---------------------------------------------------------------------------

/// Key-value store for SOAP action arguments.
///
/// Uses `String` values for all types — serialization/deserialization happens
/// at the SOAP boundary. The application layer converts to/from Rust types.
#[derive(Debug, Clone, Default)]
pub struct ActionArgs {
    pairs: Vec<(String, String)>,
}

impl ActionArgs {
    /// Create a new empty argument store.
    pub fn new() -> Self {
        Self { pairs: Vec::new() }
    }

    /// Create from a list of (name, value) pairs.
    pub fn from_pairs(pairs: Vec<(String, String)>) -> Self {
        Self { pairs }
    }

    /// Get a value by argument name.
    pub fn get(&self, name: &str) -> Option<&str> {
        self.pairs
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }

    /// Set a key-value pair.
    pub fn set(&mut self, name: String, value: String) {
        if let Some((_, v)) = self.pairs.iter_mut().find(|(k, _)| k == &name) {
            *v = value;
        } else {
            self.pairs.push((name, value));
        }
    }

    /// Get all pairs.
    pub fn pairs(&self) -> &[(String, String)] {
        &self.pairs
    }

    /// Convert to owned pairs.
    pub fn into_pairs(self) -> Vec<(String, String)> {
        self.pairs
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Action trait — runtime execution interface
// ---------------------------------------------------------------------------

/// Trait for UPnP action implementations.
///
/// Each concrete action (Play, SetVolume, GetBrightness, etc.) implements this trait.
/// Schema methods take `&self` to allow dyn compatibility (`Box<dyn Action>`).
/// The `execute` method also takes `&self` to access per-instance state (trait impls, services).
pub trait Action: Send + Sync {
    /// Returns the UPnP action name (e.g. "Play", "SetVolume", "GetTransportInfo").
    fn name(&self) -> &'static str;

    /// Returns the IN argument schema definitions for this action.
    fn in_args(&self) -> &'static [Argument];

    /// Returns the OUT argument schema definitions for this action.
    fn out_args(&self) -> &'static [Argument];

    /// Execute the action with the given runtime arguments.
    ///
    /// Returns OUT arguments on success, or a UPnP error code on failure.
    fn execute(&self, args: &ActionArgs) -> Result<ActionArgs, Error>;
}

// ---------------------------------------------------------------------------
// ActionMap — runtime registry of actions for a service
// ---------------------------------------------------------------------------

use crate::types::upnp::Services;

/// Runtime registry of actions for a service.
///
/// Maps action names to their implementations. Used by `SoapHandler` to
/// dispatch incoming SOAP requests to the correct action.
pub struct ActionMap {
    namespace: Services,
    actions: Vec<(&'static str, Box<dyn Action>)>,
}

impl ActionMap {
    /// Create a new action map for the given service namespace.
    pub fn new(namespace: Services) -> Self {
        Self {
            namespace,
            actions: Vec::new(),
        }
    }

    /// Register an action implementation.
    ///
    /// The action name is extracted via `Action::name()` — no cloning needed.
    pub fn register(&mut self, action: Box<dyn Action>) {
        let name = action.name();
        self.actions.push((name, action));
    }

    /// Get an action by name for dispatching SOAP requests.
    pub fn get(&self, name: &str) -> Option<&dyn Action> {
        self.actions
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, a)| a.as_ref())
    }

    /// Iterate over all registered actions for SCPD generation.
    pub fn iter(&self) -> impl Iterator<Item = (&'_ str, &'_ dyn Action)> {
        self.actions
            .iter()
            .map(|(name, action)| (*name, action.as_ref()))
    }

    /// Get the service namespace (full URN string).
    pub fn namespace(&self) -> String {
        self.namespace.full_namespace()
    }

    /// Get the service name from the namespace.
    pub fn service_name(&self) -> &str {
        self.namespace.name()
    }
}
