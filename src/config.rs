use std::net::SocketAddr;

use crate::scpd::ScpdHandler;
use crate::soap::SoapHandler;
use crate::types::upnp;

/// Default device description path for HTTP routing and SSDP LOCATION headers.
pub const DESCRIPTION_PATH: &str = "/upnp/description.xml";

/// SCPD path prefix — all service SCPD documents are served at `/upnp/{Service}.xml`.
pub const SCPD_PATH_PREFIX: &str = "/upnp/";

/// Default SCPD path suffix for service name resolution.
pub const SCPD_PATH_SUFFIX: &str = ".xml";

/// Library configuration — wired from main.rs.
///
/// This is the single source of truth for device identity and network settings.
/// All modules (SSDP, HTTP, service) derive their behavior from this.

/// UPnP device metadata — used for device description XML generation.
///
/// Fields derive sensible defaults from Cargo package metadata.
#[derive(Clone)]
pub struct UpnpDevice {
    /// Human-readable device name (default: CARGO_PKG_NAME)
    pub friendly_name: String,
    /// Manufacturer name
    pub manufacturer: String,
    /// Manufacturer website URL
    pub manufacturer_url: Option<String>,
    /// Human-readable model description
    pub model_description: Option<String>,
    /// Model name (default: CARGO_PKG_NAME)
    pub model_name: String,
    /// Model number (default: CARGO_PKG_VERSION)
    pub model_number: String,
    /// Model information URL
    pub model_url: Option<String>,
    /// Device serial number
    pub serial_number: Option<String>,
    /// Icon metadata
    pub icon: Option<Icon>,
    /// SERVER header value (auto-built from model_name + model_number)
    pub server_header: String,
    /// IP address the device binds to (default: 0.0.0.0)
    pub ip_addr: std::net::IpAddr,
}

impl UpnpDevice {
    /// Create a new UpnpDevice with optional overrides.
    ///
    /// All parameters are optional — defaults are applied when `None`:
    /// - `friendly_name`: CARGO_PKG_NAME
    /// - `manufacturer`: None
    /// - `model_name`: CARGO_PKG_NAME
    /// - `model_number`: CARGO_PKG_VERSION
    /// - `server_header`: auto-built as `"{model_name} / {model_number} UPnP/2.0"`
    pub fn new(
        friendly_name: Option<String>,
        manufacturer: Option<String>,
        model_name: Option<String>,
        model_number: Option<String>,
        manufacturer_url: Option<String>,
        model_description: Option<String>,
        model_url: Option<String>,
        serial_number: Option<String>,
        icon: Option<Icon>,
        ip_addr: Option<std::net::IpAddr>,
    ) -> Self {
        let friendly_name = friendly_name.unwrap_or_else(|| env!("CARGO_PKG_NAME").to_string());
        let manufacturer = manufacturer.unwrap_or_else(|| "Interweave Consumer Group".to_string());
        let model_name = model_name.unwrap_or_else(|| env!("CARGO_PKG_NAME").to_string());
        let model_number = model_number.unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_string());
        let server_header = format!("{} / {} UPnP/2.0", model_name, model_number);
        let ip_addr =
            ip_addr.unwrap_or_else(|| std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED));

        Self {
            friendly_name,
            manufacturer,
            manufacturer_url,
            model_description,
            model_name,
            model_number,
            model_url,
            serial_number,
            icon,
            server_header,
            ip_addr,
        }
    }
}

/// Icon metadata for UPnP device description.
#[derive(Clone)]
pub struct Icon {
    /// MIME type (e.g., "image/png")
    pub mime_type: String,
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Color depth in bits
    pub depth: u32,
    /// Raw image bytes
    pub bytes: Vec<u8>,
}

#[derive(Clone)]
pub struct SsdpDevice {
    /// Unique Device Name (UDN) — UUID v5 derived from MAC address
    pub udn: uuid::Uuid,
    /// LAN IP address the device binds to
    pub ip_addr: std::net::IpAddr,
    /// HTTP server port
    pub http_port: u16,
    /// SSDP multicast address (default: 239.255.255.250)
    pub ssdp_multicast_addr: String,
    /// SSDP port (default: 1900)
    pub ssdp_port: u16,
    /// SERVER header value in SSDP responses
    pub server_string: String,
    /// Device type URN (e.g., "urn:schemas-upnp-org:device:MediaRenderer:1")
    pub device_type: String,
    /// Cache-Control max-age in seconds (>= 1800 per UPnP 2.0 spec)
    pub max_age: u32,
    /// Boot ID — monotonic integer, increments on reboot
    pub boot_id: u32,
    /// Config ID — configuration version number
    pub config_id: u32,
    /// UPnP services this device exposes
    pub services: Vec<crate::types::upnp::Services>,
    /// Maximum service version for backward-compatible SSDP announcements (always 3)
    pub max_version: u32,

    /// Device description path (e.g., "/description.xml")
    pub description_path: &'static str,

    /// Full device description URL (e.g., "http://192.168.1.100:49152/description.xml")
    pub description_url: String,
}

impl SsdpDevice {
    /// Create a new device config.
    ///
    /// MAC address is derived from `ip_addr` by looking up the matching network
    /// interface. `http_port` is required and must match the actual HTTP server port.
    /// Optional fields use UPnP 2.0 defaults when `None`:
    /// boot_id=1, config_id=1, ssdp_port=1900,
    /// ssdp_multicast_addr="239.255.255.250", max_age=1800.
    pub fn new(
        ip_addr: std::net::IpAddr,
        http_port: u16,
        server_string: String,
        device_type: upnp::Device,
        services: Vec<upnp::Services>,
        boot_id: Option<u32>,
        config_id: Option<u32>,
        ssdp_port: Option<u16>,
        ssdp_multicast_addr: Option<String>,
        max_age: Option<u32>,
    ) -> Self {
        // Look up MAC address for the given IP using mac_address crate
        let mac = mac_address::get_mac_address()
            .ok()
            .flatten()
            .map(|m| m.to_string())
            .unwrap_or_else(|| "00:00:00:00:00:00".to_string());

        tracing::debug!(mac = %mac, "MAC address resolved");

        // Generate UUID v5 from MAC address using "upnpav.iw" namespace
        let namespace = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_DNS, b"upnpav.iw");
        let udn = uuid::Uuid::new_v5(&namespace, mac.as_bytes());

        tracing::debug!(udn = %udn, "UDN generated via UUID v5");

        // Per UPnP 2.0 spec: BOOTID and CONFIGID are 31-bit integers (max 2,147,483,647)
        const MAX_31BIT: u32 = 2_147_483_647;
        // Per UPnP 2.0 spec: CACHE-CONTROL max-age >= 1800
        const MIN_MAX_AGE: u32 = 1800;

        // Compute max service version across all services (always 3 for our device)
        let max_version = services
            .iter()
            .map(|svc| match svc.version() {
                upnp::ServiceVersion::V1 => 1,
                upnp::ServiceVersion::V2 => 2,
                upnp::ServiceVersion::V3 => 3,
            })
            .max()
            .unwrap_or(3);

        Self {
            udn,
            ip_addr,
            http_port,
            ssdp_multicast_addr: ssdp_multicast_addr
                .unwrap_or_else(|| "239.255.255.250".to_string()),
            ssdp_port: ssdp_port.unwrap_or(1900),
            server_string,
            device_type: device_type.urn(),
            max_age: max_age.map(|v| v.max(MIN_MAX_AGE)).unwrap_or(MIN_MAX_AGE),
            boot_id: boot_id.map(|v| v.min(MAX_31BIT)).unwrap_or(1),
            config_id: config_id.map(|v| v.min(MAX_31BIT)).unwrap_or(1),
            services,
            max_version,
            description_path: DESCRIPTION_PATH,
            description_url: format!("http://{}:{}{}", ip_addr, http_port, DESCRIPTION_PATH),
        }
    }
}

/// Combined device configuration — holds UpnpDevice.
///
/// SsdpDevice is built at start() time from runtime network info.
#[derive(Clone)]
pub struct DeviceConfig {
    pub device: UpnpDevice,
}

impl DeviceConfig {
    /// Create a combined config from UpnpDevice.
    ///
    /// SsdpDevice is built at start() time from runtime network info.
    pub fn new(device: UpnpDevice) -> Self {
        Self { device }
    }

    /// Start the UPnP device — SSDP, HTTP server with all handlers, and serve.
    ///
    /// Takes the SsdpDevice built at start() time from runtime network info,
    /// and the three service instances for SOAP/SCPD/eventing handlers.
    pub async fn start(
        &self,
        ssdp: SsdpDevice,
        av_transport: crate::services::avtransport::AvTransportService,
        connection_manager: crate::services::connectionmanager::ConnectionManagerService,
        rendering_control: crate::services::renderingcontrol::RenderingControlService,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Start SSDP discovery
        let _ssdp_server = crate::ssdp::start(ssdp.clone());

        let server_header = self.device.server_header.clone();

        // Description handler
        let description = crate::DescriptionHandler::new(
            self.device.clone(),
            ssdp.clone(),
            server_header.clone(),
        );

        // HTTP server with all routes
        let addr: SocketAddr = format!("{}:{}", ssdp.ip_addr, ssdp.http_port).parse()?;
        let mut server =
            crate::http::HttpServer::new(addr, &self.device.server_header).register(description);

        // Register SCPD handlers
        server = server.register(ScpdHandler::new(
            av_transport.actions(),
            av_transport.state_store(),
            server_header.clone(),
            ssdp.config_id,
        ));
        server = server.register(ScpdHandler::new(
            connection_manager.actions(),
            connection_manager.state_store(),
            server_header.clone(),
            ssdp.config_id,
        ));
        server = server.register(ScpdHandler::new(
            rendering_control.actions(),
            rendering_control.state_store(),
            server_header.clone(),
            ssdp.config_id,
        ));

        // Register SOAP handlers
        server = server.register(SoapHandler::new(
            upnp::Services::AVTransport(upnp::ServiceVersion::V3).url_control(),
            av_transport.actions(),
            server_header.clone(),
        ));
        server = server.register(SoapHandler::new(
            upnp::Services::ConnectionManager(upnp::ServiceVersion::V3).url_control(),
            connection_manager.actions(),
            server_header.clone(),
        ));
        server = server.register(SoapHandler::new(
            upnp::Services::RenderingControl(upnp::ServiceVersion::V3).url_control(),
            rendering_control.actions(),
            server_header.clone(),
        ));

        // Serve
        server.serve().await
    }
}
