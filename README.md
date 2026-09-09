# upnpav — UPnP AV 2.0 Device Architecture

A Rust library implementing the UPnP AV 2.0 Device Architecture specification. Provides a trait-based abstraction layer for building UPnP AV devices (media renderers, bridges, servers) with clean separation between application logic and UPnP protocol handling.

## Architecture

```
┌─────────────────────────────────────────────────┐
│              Application Layer                   │
│  (implements Play, Stop, GetVolume, etc.)       │
├─────────────────────────────────────────────────┤
│              Bridge Layer                        │
│  (implements ActionMap, maps trait → SOAP)      │
├─────────────────────────────────────────────────┤
│              UPnP AV 2.0 Protocol Layer          │
│  SSDP · GENA · SOAP · HTTP · DIDL · SCPD        │
└─────────────────────────────────────────────────┘
```

**Trait-based adapter pattern:** The application implements high-level media control traits. Bridge structs implement `upnpav::Action` to translate SOAP actions into trait calls. An `ActionMap` dispatches incoming SOAP requests to the correct bridge.

## Features

- **Trait-based application interface** — Implement `Play`, `Stop`, `GetVolume`, etc. No UPnP protocol knowledge required.
- **Typed outputs** — Dedicated output structs per action with proper UPnP error code mapping.
- **Domain enums** — `TransportState`, `Channel`, `PlayMode`, `TransportStatus` match the UPnP AV 2.0 spec exactly.
- **State management** — `StateStore` with type validation, LastChange tracking, and SCPD XML generation.
- **Protocol modules** — SSDP discovery, GENA eventing, SOAP action handling, HTTP serving, DIDL-Lite metadata.
- **Service implementations** — AVTransport, RenderingControl, ConnectionManager service scaffolding.

## Modules

| Module | Purpose |
|--------|---------|
| `config` | Device configuration, `DeviceConfig`, `SsdpDevice`, `UpnpDevice` |
| `didl` | DIDL-Lite metadata parsing/serialization (`DidlDocument`, `Item`, `Resource`) |
| `gena` | GENA eventing — `EventPublisher`, `Subscriber` management |
| `http` | HTTP server and handler for SOAP/HTTP transport |
| `scpd` | SCPD (Service Control Protocol Description) XML handling |
| `services` | UPnP AV 2.0 service implementations |
| `soap` | SOAP request parsing and response/fault building |
| `ssdp` | SSDP discovery — `Server` and `start` for announce/search |
| `state` | State management — `StateSchema`, `StateVariable`, `StateStore` |
| `types` | Shared types — `DataType`, `StateValue`, `Action`, `ActionMap`, `ServiceVersion` |

### Services

| Service | Description |
|---------|-------------|
| `avtransport` | AVTransport service — 27 state variables, transport control actions |
| `connectionmanager` | ConnectionManager service — connection tracking, port management |
| `renderingcontrol` | RenderingControl service — volume, balance, mute, brightness |
| `lastchange` | LastChange event serialization for evented state variables |

## Quick Start

### 1. Define your application traits

```rust
use upnpav::types::upnp::*;
use upnpav::types::upnp::domain::avtransport::*;

struct MyPlayer;

impl Play for MyPlayer {
    type PlayError = PlayError;

    fn play(&self, speed: Speed) -> Result<PlayResponse, Self::PlayError> {
        // Start media playback at the given speed
        Ok(PlayResponse {})
    }
}

impl Stop for MyPlayer {
    type StopError = StopError;

    fn stop(&self) -> Result<StopResponse, Self::StopError> {
        // Stop media playback
        Ok(StopResponse {})
    }
}
```

### 2. Wire up the bridge

```rust
use upnpav::services::avtransport::AvTransportActionMap;
use upnpav::state::StateStore;

let mut state_store = StateStore::<AvTransportStateVariableName>::new();
// Register state variables via StateSchema...

let bridge = AvTransportBridge::new(MyPlayer, state_store);
```

### 3. Start the protocol stack

```rust
use upnpav::ssdp;
use upnpav::http;

// SSDP discovery server
let ssdp_server = ssdp::start(config).await?;

// HTTP server for SOAP/HTTP transport
let http_server = http::HttpServer::new(handler).await?;
```

## Design Principles

1. **Compile-time type safety** — `StateSchema` defines the compile-time contract per state variable. `StateStore<S: StateVariableName>` validates all writes against the schema.
2. **Event-driven state** — `StateVariable` tracks `has_changes` for GENA eventing. Writing the same value does NOT set `has_changes` (per UPnP spec).
3. **Instance scoping** — State variables are scoped per `InstanceID`. `InstanceID=0` is global/post-mix, `InstanceID>0` is per-stream.
4. **Separation of concerns** — SSDP, HTTP, SOAP, GENA, DIDL are separate modules with clear boundaries.

## Data Types

The library supports all UPnP AV 2.0 data types:

```rust
pub enum DataType {
    Boolean,
    String,
    I2,
    U2,
    I4,
    U4,
    DateTime,
    Time,
    Uri,
    MediaClock,
    TransportState,
    TransportStatus,
    NRChannel,
    NRPlayMode,
    RelTime,
    AbsTime,
    RelCounter,
    AbsCounter,
    Number,
    Ratio,
    Block,
    Base64,
    Binary,
}
```

## Error Handling

Actions return typed error enums (e.g., `PlayError`, `StopError`) that map to UPnP AV 2.0 error codes. The bridge layer converts these to SOAP `ActionFailed` responses.

Common error codes:
- `716` — Invalid TransitionAction
- `717` — InvalidTransportState
- `706` — IllegalSeekTarget

## Current Status

**Foundation complete, integration in progress.**

The library provides a strong architectural foundation:
- ✅ Trait-based application boundary
- ✅ Typed action outputs
- ✅ State management with validation
- ✅ Protocol modules (SSDP, GENA, SOAP, HTTP, DIDL)
- ✅ Service scaffolding (AVTransport, RenderingControl, ConnectionManager)

**Known gaps:**
- Event notification from StateStore to application
- State-aware action validation
- InstanceID allocation mechanism
- DIDL metadata parsing integration
- Application-level connection root wiring

## Dependencies

```toml
[dependencies]
upnpav = { path = "lib/upnpav" }
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
serde = { version = "1", features = ["derive"] }
```

## License

GPL-3.0
