//! Device description and SCPD — XML generation and HTTP handlers.
//!
//! Serves:
//! - GET /description.xml — UPnP device description
//! - GET /icon.png — icon image (base64 in XML, raw bytes on HTTP)
//! - GET /{Service}.xml — SCPD documents (generated dynamically)

use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::StatusCode;
use quick_xml::escape::escape as escape_xml_str;

use crate::config::{Icon, SCPD_PATH_PREFIX, SCPD_PATH_SUFFIX, SsdpDevice, UpnpDevice};
use crate::http::{CacheControl, ContentType, HttpHandler, UpnpResponse};
use crate::types::upnp::{ActionMap, StateStore};

/// HTTP handler for device description and icon.
#[derive(Clone)]
pub struct DescriptionHandler {
    upnpdevice: Arc<UpnpDevice>,
    ssdpdevice: Arc<SsdpDevice>,
    server_info: String,
}

impl DescriptionHandler {
    /// Create a new description handler.
    pub fn new(upnpdevice: UpnpDevice, ssdpdevice: SsdpDevice, server_info: String) -> Self {
        Self {
            upnpdevice: Arc::new(upnpdevice),
            ssdpdevice: Arc::new(ssdpdevice),
            server_info,
        }
    }

    /// Build the device description XML.
    pub fn build_description_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(&self.build_root());
        xml
    }

    /// Build the root `<root>` element with all children.
    fn build_root(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\"?>\n");
        xml.push_str(&format!(
            "<root xmlns=\"urn:schemas-upnp-org:device-1-0\" configId=\"{}\">\n",
            self.ssdpdevice.config_id
        ));

        // specVersion
        xml.push_str("  <specVersion>\n");
        xml.push_str("    <major>2</major>\n");
        xml.push_str("    <minor>0</minor>\n");
        xml.push_str("  </specVersion>\n");

        // device
        xml.push_str("  <device>\n");
        xml.push_str(&self.build_device_element());
        xml.push_str("  </device>\n");

        xml.push_str("</root>\n");
        xml
    }

    /// Build the `<device>` element with all child elements.
    fn build_device_element(&self) -> String {
        let mut xml = String::new();

        xml.push_str(&format!(
            "    <deviceType>{}</deviceType>\n",
            self.ssdpdevice.device_type
        ));
        xml.push_str(&format!("    <UDN>uuid:{}</UDN>\n", self.ssdpdevice.udn));
        xml.push_str(&format!(
            "    <friendlyName>{}</friendlyName>\n",
            escape_xml(&self.upnpdevice.friendly_name)
        ));
        xml.push_str(&format!(
            "    <manufacturer>{}</manufacturer>\n",
            escape_xml(&self.upnpdevice.manufacturer)
        ));

        if let Some(url) = &self.upnpdevice.manufacturer_url {
            xml.push_str(&format!("    <manufacturerURL>{}</manufacturerURL>\n", url));
        }

        if let Some(desc) = &self.upnpdevice.model_description {
            xml.push_str(&format!(
                "    <modelDescription>{}</modelDescription>\n",
                escape_xml(desc)
            ));
        }

        xml.push_str(&format!(
            "    <modelName>{}</modelName>\n",
            escape_xml(&self.upnpdevice.model_name)
        ));
        xml.push_str(&format!(
            "    <modelNumber>{}</modelNumber>\n",
            escape_xml(&self.upnpdevice.model_number)
        ));

        if let Some(url) = &self.upnpdevice.model_url {
            xml.push_str(&format!("    <modelURL>{}</modelURL>\n", url));
        }

        if let Some(serial) = &self.upnpdevice.serial_number {
            xml.push_str(&format!("    <serialNumber>{}</serialNumber>\n", serial));
        }

        if let Some(icon) = &self.upnpdevice.icon {
            xml.push_str("    <iconList>\n");
            xml.push_str(&self.build_icon(icon));
            xml.push_str("    </iconList>\n");
        }

        xml.push_str("    <serviceList>\n");
        for service in &self.ssdpdevice.services {
            xml.push_str("      <service>\n");
            xml.push_str(&format!(
                "        <serviceType>{}</serviceType>\n",
                service.full_namespace()
            ));
            xml.push_str(&format!(
                "        <serviceId>urn:upnp-org:serviceId:{}</serviceId>\n",
                service.name()
            ));
            xml.push_str(&format!(
                "        <SCPDURL>{}</SCPDURL>\n",
                service.scpd_url()
            ));
            xml.push_str(&format!(
                "        <controlURL>{}</controlURL>\n",
                service.url_control()
            ));
            xml.push_str(&format!(
                "        <eventSubURL>{}</eventSubURL>\n",
                service.event_url()
            ));
            xml.push_str("      </service>\n");
        }
        xml.push_str("    </serviceList>\n");

        xml
    }

    /// Build an `<icon>` element.
    fn build_icon(&self, icon: &Icon) -> String {
        let mut xml = String::new();
        xml.push_str("        <icon>\n");
        xml.push_str(&format!(
            "          <mimetype>{}</mimetype>\n",
            icon.mime_type
        ));
        xml.push_str(&format!("          <width>{}</width>\n", icon.width));
        xml.push_str(&format!("          <height>{}</height>\n", icon.height));
        xml.push_str(&format!("          <depth>{}</depth>\n", icon.depth));
        xml.push_str("          <url>icon.png</url>\n");
        xml.push_str("        </icon>\n");
        xml
    }

    /// Handle GET request for device description.
    async fn handle_description(&self) -> UpnpResponse {
        UpnpResponse {
            status: StatusCode::OK,
            content_type: ContentType::XmlUtf8,
            cache_control: CacheControl::MaxAge(1800),
            server: self.server_info.clone(),
            body: Body::from(self.build_description_xml()),
        }
    }

    /// Handle GET request for icon image.
    async fn handle_icon(&self) -> UpnpResponse {
        match &self.upnpdevice.icon {
            Some(icon) => UpnpResponse {
                status: StatusCode::OK,
                content_type: ContentType::Custom(icon.mime_type.clone()),
                cache_control: CacheControl::None,
                server: self.server_info.clone(),
                body: Body::from(icon.bytes.clone()),
            },
            None => UpnpResponse {
                status: StatusCode::NOT_FOUND,
                content_type: ContentType::PlainUtf8,
                cache_control: CacheControl::None,
                server: self.server_info.clone(),
                body: Body::from("Not found"),
            },
        }
    }
}

impl HttpHandler for DescriptionHandler {
    fn path(&self) -> &'static str {
        self.ssdpdevice.description_path
    }

    fn register(self, router: Router) -> Router {
        let handler_desc = self.clone();
        let handler_icon = self.clone();
        let desc_path = self.ssdpdevice.description_path;
        let router = router.route(
            desc_path,
            axum::routing::get(move || {
                tracing::debug!("request: GET {}", desc_path);
                let handler = handler_desc.clone();
                async move { handler.handle_description().await.build() }
            }),
        );
        tracing::debug!("route mounted: {}", desc_path);
        let router = router.route(
            "/icon.png",
            axum::routing::get(move || {
                tracing::debug!("request: GET /icon.png");
                let handler = handler_icon.clone();
                async move { handler.handle_icon().await.build() }
            }),
        );
        tracing::debug!("route mounted: /icon.png");
        router
    }
}

// ---------------------------------------------------------------------------
// ScpdHandler — SCPD document serving
// ---------------------------------------------------------------------------

/// HTTP handler for SCPD documents.
///
/// Serves dynamically generated SCPD XML from the action map and state variables.
/// Holds only Arc references — no owned data.
pub struct ScpdHandler<
    S: std::fmt::Display + Clone + std::hash::Hash + crate::state::StateVariableName,
> {
    path: &'static str,
    actions: Arc<std::sync::Mutex<ActionMap>>,
    state_store: Arc<std::sync::Mutex<StateStore<S>>>,
    server_header: String,
    config_id: u32,
}

impl<S: std::fmt::Display + Clone + std::hash::Hash + crate::state::StateVariableName>
    ScpdHandler<S>
{
    /// Create a new SCPD handler.
    pub fn new(
        actions: Arc<std::sync::Mutex<ActionMap>>,
        state_store: Arc<std::sync::Mutex<StateStore<S>>>,
        server_header: String,
        config_id: u32,
    ) -> Self {
        let service_name = {
            let guard = actions.lock().unwrap();
            guard.service_name().to_owned()
        };
        let path = format!("{}{}{}", SCPD_PATH_PREFIX, service_name, SCPD_PATH_SUFFIX);
        // Safe: path is constructed once and lives for the program lifetime.
        let path = Box::leak(path.into_boxed_str());
        Self {
            path,
            actions,
            state_store,
            server_header,
            config_id,
        }
    }

    /// Build the complete SCPD XML document.
    pub fn build_scpd_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(&self.build_scpd_header());
        xml.push_str(&self.build_action_list());
        xml.push_str(&self.build_state_table());
        xml.push_str("</scpd>\n");
        xml
    }

    /// Build the `<scpd>` root element with namespace and specVersion.
    fn build_scpd_header(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\"?>\n");
        xml.push_str(&format!(
            "<scpd xmlns=\"urn:schemas-upnp-org:service-1-0\" configId=\"{}\">\n",
            self.config_id
        ));
        xml.push_str("  <specVersion>\n");
        xml.push_str("    <major>2</major>\n");
        xml.push_str("    <minor>0</minor>\n");
        xml.push_str("  </specVersion>\n");
        xml
    }

    /// Build the `<actionList>` element from registered actions.
    ///
    /// Emits full `<argumentList>` with name, direction, and relatedStateVariable
    /// for each IN and OUT argument, as required by UPnP SCPD spec.
    fn build_action_list(&self) -> String {
        let mut xml = String::new();
        xml.push_str("  <actionList>\n");
        let actions = self.actions.lock().unwrap();
        for (name, action) in actions.iter() {
            xml.push_str("    <action>\n");
            xml.push_str(&format!("      <name>{}</name>\n", escape_xml(name)));
            xml.push_str("      <argumentList>\n");
            for arg in action.in_args() {
                xml.push_str("        <argument>\n");
                xml.push_str(&format!(
                    "          <name>{}</name>\n",
                    escape_xml(arg.name)
                ));
                xml.push_str("          <direction>in</direction>\n");
                if let Some(var) = &arg.related_state_var {
                    xml.push_str(&format!(
                        "          <relatedStateVariable>{}</relatedStateVariable>\n",
                        escape_xml(var)
                    ));
                }
                xml.push_str("        </argument>\n");
            }
            for arg in action.out_args() {
                xml.push_str("        <argument>\n");
                xml.push_str(&format!(
                    "          <name>{}</name>\n",
                    escape_xml(arg.name)
                ));
                xml.push_str("          <direction>out</direction>\n");
                if let Some(var) = &arg.related_state_var {
                    xml.push_str(&format!(
                        "          <relatedStateVariable>{}</relatedStateVariable>\n",
                        escape_xml(var)
                    ));
                }
                xml.push_str("        </argument>\n");
            }
            xml.push_str("      </argumentList>\n");
            xml.push_str("    </action>\n");
        }
        xml.push_str("  </actionList>\n");
        xml
    }

    /// Build the `<serviceStateTable>` element from registered state variables.
    ///
    /// Filters: only emit state variables that are:
    /// 1. Referenced by at least one registered action's argument, OR
    /// 2. Evented (sendEvents="yes"), OR
    /// 3. A_ARG_TYPE type definitions (argument_type=true)
    fn build_state_table(&self) -> String {
        // Collect all relatedStateVariable refs from registered actions
        let mut referenced_vars: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        {
            let actions = self.actions.lock().unwrap();
            for (_, action) in actions.iter() {
                for arg in action.in_args() {
                    if let Some(var) = &arg.related_state_var {
                        referenced_vars.insert(var.to_string());
                    }
                }
                for arg in action.out_args() {
                    if let Some(var) = &arg.related_state_var {
                        referenced_vars.insert(var.to_string());
                    }
                }
            }
        }

        let mut xml = String::new();
        xml.push_str("  <serviceStateTable>\n");
        let store = self.state_store.lock().unwrap();
        for name in store.names() {
            if let Some(def) = store.schema(&name) {
                // Filter: only emit if referenced by an action, evented, or A_ARG_TYPE
                let is_referenced = referenced_vars.contains(&name.to_string());
                let is_evented = def.name.is_evented() || def.name.via_lastchange();
                let is_arg_type = def.argument_type;

                if !is_referenced && !is_evented && !is_arg_type {
                    continue; // Skip unregistered state variables
                }

                xml.push_str("    <stateVariable");
                if def.name.is_evented() || def.name.via_lastchange() {
                    xml.push_str(" sendEvents=\"yes\"");
                }
                xml.push_str(">\n");
                xml.push_str(&format!("      <name>{}</name>\n", name));
                xml.push_str(&format!(
                    "      <dataType>{}</dataType>\n",
                    def.name.data_type_name()
                ));

                // A_ARG_TYPE variables are type definitions — no defaultValue
                if !def.argument_type {
                    xml.push_str(&format!(
                        "      <defaultValue>{}</defaultValue>\n",
                        escape_xml(&def.default.as_value_str())
                    ));
                }

                if let Some(allowed) = &def.allowed_values {
                    if !allowed.is_empty() {
                        xml.push_str("      <allowedValueList>\n");
                        for val in allowed {
                            xml.push_str(&format!(
                                "        <allowedValue>{}</allowedValue>\n",
                                escape_xml(val)
                            ));
                        }
                        xml.push_str("      </allowedValueList>\n");
                    }
                }

                if let Some((min, max, _)) = &def.allowed_value_range {
                    xml.push_str("      <allowedValueRange>\n");
                    xml.push_str(&format!("        <minimum>{}</minimum>\n", min));
                    xml.push_str(&format!("        <maximum>{}</maximum>\n", max));
                    xml.push_str("      </allowedValueRange>\n");
                }

                xml.push_str("    </stateVariable>\n");
            }
        }
        xml.push_str("  </serviceStateTable>\n");
        xml
    }

    /// Handle GET request for SCPD document.
    async fn handle_scpd(&self) -> UpnpResponse {
        UpnpResponse {
            status: StatusCode::OK,
            content_type: ContentType::XmlUtf8,
            cache_control: CacheControl::MaxAge(1800),
            server: self.server_header.clone(),
            body: Body::from(self.build_scpd_xml()),
        }
    }
}

impl<
    S: std::fmt::Display
        + Clone
        + std::hash::Hash
        + Send
        + Sync
        + 'static
        + crate::state::StateVariableName,
> Clone for ScpdHandler<S>
{
    fn clone(&self) -> Self {
        Self {
            path: self.path,
            actions: self.actions.clone(),
            state_store: self.state_store.clone(),
            server_header: self.server_header.clone(),
            config_id: self.config_id,
        }
    }
}

impl<
    S: std::fmt::Display
        + Clone
        + std::hash::Hash
        + Send
        + Sync
        + 'static
        + crate::state::StateVariableName,
> HttpHandler for ScpdHandler<S>
{
    fn path(&self) -> &'static str {
        self.path
    }

    fn register(self, router: Router) -> Router {
        let handler = self.clone();
        let path = self.path;
        let router = router.route(
            path,
            axum::routing::get(move || {
                tracing::debug!("request: GET {}", path);
                let handler = handler.clone();
                async move { handler.handle_scpd().await.build() }
            }),
        );
        tracing::debug!("route mounted: {}", path);
        router
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Escape XML special characters using quick-xml's built-in escape.
pub fn escape_xml(s: &str) -> String {
    escape_xml_str(s).into_owned()
}
