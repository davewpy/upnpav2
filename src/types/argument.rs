// ===========================================================================
// UPnP AV 2.0 — Argument Primitives
// ===========================================================================
//
// Compile-time type definitions for action arguments:
// - ArgumentDirection (IN/OUT)
// - Argument (schema definition for a single argument)

/// Direction of an action argument in the UPnP wire protocol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentDirection {
    /// Input argument (sent to the service).
    IN,
    /// Output argument (returned by the service).
    OUT,
}

/// Schema definition for a single UPnP action argument.
///
/// All fields are static — known at compile time. This struct is used
/// by the `Action` trait to expose schema metadata without allocations.
#[derive(Debug, Clone)]
pub struct Argument {
    /// Argument name (e.g., "InstanceID", "Speed", "Volume").
    pub name: &'static str,
    /// Direction: IN for input arguments, OUT for output arguments.
    pub direction: ArgumentDirection,
    /// If set, the related state variable this argument writes to.
    /// Per UPnP spec §5 "State Effects", IN arguments with a relatedStateVariable
    /// implicitly write that state variable on action execution.
    pub related_state_var: Option<&'static str>,
}
