//! UPnP server lifecycle — SSDP + HTTP orchestration.
//!
//! This module provides a high-level `UpnpServer` that owns the complete
//! UPnP device stack: SSDP discovery, HTTP serving, and service registration.
//!
//! # Usage
//!
//! ```ignore
//! let config = DeviceConfig::new(device);
//! let ssdp = SsdpDevice::new(...);
//! let (av, cm, rc) = instantiate_services();
//!
//! let server = UpnpServer::builder(config)
//!     .with_ssdp(ssdp)
//!     .with_av_transport(av)
//!     .with_connection_manager(cm)
//!     .with_rendering_control(rc)
//!     .build()
//!     .expect("failed to build server");
//!
//! server.start().await?;
//! // ... run app ...
//! server.stop();
//! ```

use std::net::SocketAddr;
use std::sync::Arc;

use tokio::task::JoinHandle;
use tracing::{debug, error, info};

use crate::config::{SsdpDevice, UpnpDevice};
use crate::http::HttpServer;
use crate::scpd::DescriptionHandler;
use crate::services::avtransport::AvTransportService;
use crate::services::connectionmanager::ConnectionManagerService;
use crate::services::renderingcontrol::RenderingControlService;
use crate::soap::SoapHandler;
use crate::ssdp::{Server as SsdpServer, start as start_ssdp};
use crate::types::upnp::{ServiceVersion, Services};

// ===========================================================================
// Builder — fluent construction of UpnpServer
// ===========================================================================

/// Builder for [`UpnpServer`].
///
/// Each `with_*` method takes ownership of a component and returns the builder
/// for chaining. All components except `device` are optional but required by
/// `build()` — it returns an error if any required field is missing.
pub struct UpnpServerBuilder {
    device: Option<UpnpDevice>,
    ssdp_device: Option<SsdpDevice>,
    av_transport: Option<Arc<AvTransportService>>,
    connection_manager: Option<Arc<ConnectionManagerService>>,
    rendering_control: Option<Arc<RenderingControlService>>,
}

impl UpnpServerBuilder {
    /// Create a builder with the device metadata.
    pub fn new(device: UpnpDevice) -> Self {
        Self {
            device: Some(device),
            ssdp_device: None,
            av_transport: None,
            connection_manager: None,
            rendering_control: None,
        }
    }

    /// Set the SSDP device configuration.
    pub fn with_ssdp(mut self, ssdp: SsdpDevice) -> Self {
        self.ssdp_device = Some(ssdp);
        self
    }

    /// Set the AV Transport service.
    pub fn with_av_transport(mut self, svc: Arc<AvTransportService>) -> Self {
        self.av_transport = Some(svc);
        self
    }

    /// Set the Connection Manager service.
    pub fn with_connection_manager(mut self, svc: Arc<ConnectionManagerService>) -> Self {
        self.connection_manager = Some(svc);
        self
    }

    /// Set the Rendering Control service.
    pub fn with_rendering_control(mut self, svc: Arc<RenderingControlService>) -> Self {
        self.rendering_control = Some(svc);
        self
    }

    /// Build the server, returning an error if required fields are missing.
    pub fn build(self) -> Result<UpnpServer, String> {
        Ok(UpnpServer {
            device: self.device.ok_or("device is required")?,
            ssdp_device: self.ssdp_device.ok_or("ssdp_device is required")?,
            av_transport: self.av_transport.ok_or("av_transport is required")?,
            connection_manager: self.connection_manager.ok_or("connection_manager is required")?,
            rendering_control: self.rendering_control.ok_or("rendering_control is required")?,
            _ssdp_handle: None,
            _http_handle: None,
        })
    }
}

// ===========================================================================
// UpnpServer — owns the complete UPnP device stack
// ===========================================================================

/// Complete UPnP device server — SSDP + HTTP + services.
///
/// `start()` spawns background tasks for SSDP and HTTP.
/// `stop()` sends byebye notifications and aborts all tasks.
pub struct UpnpServer {
    /// Device metadata (name, model, IP, etc.)
    device: UpnpDevice,

    /// Runtime SSDP configuration (UDN, ports, multicast settings)
    ssdp_device: SsdpDevice,

    /// UPnP AV services — shared via Arc across HTTP handlers
    av_transport: Arc<AvTransportService>,
    connection_manager: Arc<ConnectionManagerService>,
    rendering_control: Arc<RenderingControlService>,

    /// Lifecycle handles — set by `start()`, cleared by `stop()`
    _ssdp_handle: Option<SsdpServer>,
    _http_handle: Option<JoinHandle<()>>,
}

impl UpnpServer {
    /// Create a builder for this server.
    pub fn builder(device: UpnpDevice) -> UpnpServerBuilder {
        UpnpServerBuilder::new(device)
    }

    /// Start SSDP discovery and HTTP server.
    ///
    /// Spawns background tasks for both. Returns immediately — the server
    /// runs until `stop()` is called or an error occurs.
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Start SSDP (sync call, spawns internal async tasks)
        let ssdp = self.ssdp_device.clone();
        self._ssdp_handle = Some(start_ssdp(ssdp));
        info!("SSDP started");

        // Build HTTP server with all routes
        let server_header = self.device.server_header.clone();
        let addr: SocketAddr = format!("{}:{}", self.ssdp_device.ip_addr, self.ssdp_device.http_port)
            .parse()?;

        let mut http_server = HttpServer::new(addr, &server_header);

        // Description handler — serves /upnp/description.xml
        http_server = http_server.register(DescriptionHandler::new(
            self.device.clone(),
            self.ssdp_device.clone(),
            server_header.clone(),
        ));

        // SCPD handlers — serve service capability documents
        let config_id = self.ssdp_device.config_id;
        http_server = http_server.register(crate::scpd::ScpdHandler::new(
            self.av_transport.actions(),
            self.av_transport.state_store(),
            server_header.clone(),
            config_id,
        ));
        http_server = http_server.register(crate::scpd::ScpdHandler::new(
            self.connection_manager.actions(),
            self.connection_manager.state_store(),
            server_header.clone(),
            config_id,
        ));
        http_server = http_server.register(crate::scpd::ScpdHandler::new(
            self.rendering_control.actions(),
            self.rendering_control.state_store(),
            server_header.clone(),
            config_id,
        ));

        // SOAP handlers — service control points
        http_server = http_server.register(SoapHandler::new(
            Services::AVTransport(ServiceVersion::V3).url_control(),
            self.av_transport.actions(),
            server_header.clone(),
        ));
        http_server = http_server.register(SoapHandler::new(
            Services::ConnectionManager(ServiceVersion::V3).url_control(),
            self.connection_manager.actions(),
            server_header.clone(),
        ));
        http_server = http_server.register(SoapHandler::new(
            Services::RenderingControl(ServiceVersion::V3).url_control(),
            self.rendering_control.actions(),
            server_header.clone(),
        ));

        // Spawn HTTP task — http_server already owns all handlers with Arc-wrapped services
        self._http_handle = Some(tokio::spawn(async move {
            if let Err(e) = http_server.serve().await {
                error!(error = %e, "HTTP server failed");
            }
        }));

        info!("HTTP server started on {}:{}", self.ssdp_device.ip_addr, self.ssdp_device.http_port);
        Ok(())
    }

    /// Stop all background tasks gracefully.
    ///
    /// 1. Sends SSDP byebye notifications
    /// 2. Aborts the HTTP task (forceful — may drop in-flight requests)
    pub fn stop(&mut self) {
        if let Some(handle) = self._ssdp_handle.take() {
            handle.stop(&self.ssdp_device);
            debug!("SSDP stopped");
        }

        if let Some(handle) = self._http_handle.take() {
            handle.abort();
            debug!("HTTP server aborted");
        }
    }
}
