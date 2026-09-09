use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::Request;
use axum::http::StatusCode;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::reader::Reader;

use crate::http::{CacheControl, ContentType, HttpHandler, UpnpResponse};
use crate::types::{ActionArgs, ActionMap, Error};

/// HTTP handler that processes SOAP requests for a single service.
///
/// Holds the service URL path, an Arc<ActionMap>, and the SERVER header value.
/// Implements HttpHandler so it can be registered with the HTTP server.
pub struct SoapHandler {
    path: &'static str,
    actions: Arc<std::sync::Mutex<ActionMap>>,
    server_header: String,
}

impl SoapHandler {
    /// Create a new handler for the given path, action map, and server info.
    pub fn new(
        path: &'static str,
        actions: Arc<std::sync::Mutex<ActionMap>>,
        server_header: String,
    ) -> Self {
        Self {
            path,
            actions,
            server_header,
        }
    }
}

impl HttpHandler for SoapHandler {
    fn path(&self) -> &'static str {
        self.path
    }

    fn register(self, router: Router) -> Router {
        let actions = self.actions.clone();
        let server_info = self.server_header.clone();
        let path = self.path;
        let router = router.route(
            path,
            axum::routing::post(move |req: Request<Body>| {
                tracing::debug!("request: POST {}", path);
                let actions = actions.clone();
                let server_info = server_info.clone();
                async move { handle_request(actions, req, server_info).await.build() }
            }),
        );
        tracing::debug!("route mounted: {}", path);
        router
    }
}

async fn handle_request(
    actions: Arc<std::sync::Mutex<ActionMap>>,
    req: Request<Body>,
    server_info: String,
) -> UpnpResponse {
    let headers: Vec<(String, String)> = req
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();

    let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX)
        .await
        .unwrap_or_default();
    let body = String::from_utf8(body_bytes.to_vec()).unwrap_or_default();

    let actions = actions.lock().unwrap();

    let (status, body) = match parse_request(&headers, &body) {
        Err(e) => {
            tracing::debug!("request parse failed: {:?}", e);
            (StatusCode::OK, build_fault(e as i32, "Invalid request"))
        }
        Ok((ns, _action_name, ref args)) if ns != actions.namespace() => {
            tracing::debug!(
                "namespace mismatch: expected {}, got {}",
                actions.namespace(),
                ns
            );
            (StatusCode::OK, build_fault(401, "Invalid Action"))
        }
        Ok((ns, action_name, args)) => {
            tracing::debug!(
                "received action: {} with {} args",
                action_name,
                args.pairs().len()
            );
            match actions.get(&action_name) {
                None => {
                    tracing::debug!("action not implemented: {}", action_name);
                    (StatusCode::OK, build_fault(501, "Not Implemented"))
                }
                Some(action) => match action.execute(&args) {
                    Ok(returns) => {
                        tracing::debug!(
                            "response: {} with {} returns",
                            action_name,
                            returns.pairs().len()
                        );
                        (
                            StatusCode::OK,
                            build_response(&action_name, &returns.into_pairs(), &ns),
                        )
                    }
                    Err(e) => {
                        tracing::debug!("error: {} -> code {}", action_name, e as i32);
                        (StatusCode::OK, build_fault(e as i32, "Action failed"))
                    }
                },
            }
        }
    };

    tracing::debug!("response with status: {}", status);
    UpnpResponse {
        status,
        content_type: ContentType::XmlUtf8,
        cache_control: CacheControl::None,
        server: server_info,
        body: Body::from(body),
    }
}

/// Parse a complete SOAP request — validates Content-Type, extracts SOAPACTION, parses XML.
///
/// Returns (namespace, action_name, args).
///
/// This is the single entry point for all SOAP parsing. Callers pass raw HTTP headers
/// and body; this function validates protocol requirements and extracts the action + args.
pub fn parse_request(
    headers: &[(String, String)],
    body: &str,
) -> Result<(String, String, ActionArgs), Error> {
    // Validate Content-Type — UPnP spec requires text/xml
    let content_type = headers
        .iter()
        .find(|(k, _)| k.to_lowercase() == "content-type")
        .map(|(_, v)| v.as_str())
        .unwrap_or("");
    if !content_type.starts_with("text/xml") {
        return Err(Error::InvalidAction);
    }

    // Extract SOAPACTION header — UPnP spec requires "namespace#ActionName"
    let soap_action = headers
        .iter()
        .find(|(k, _)| k.to_lowercase() == "soapaction")
        .map(|(_, v)| v.as_str())
        .unwrap_or("");
    let parts: Vec<&str> = soap_action.split('#').collect();
    let (ns, act) = match parts.as_slice() {
        [ns, act] if !ns.is_empty() && !act.is_empty() => (ns.to_string(), act.to_string()),
        _ => return Err(Error::InvalidAction),
    };

    // Parse XML body for arguments
    let args = parse_xml_body(body)?;

    Ok((ns, act, ActionArgs::from_pairs(args)))
}

/// Parse the XML body to extract action arguments.
fn parse_xml_body(xml: &str) -> Result<Vec<(String, String)>, Error> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut in_body = false;
    let mut action_name: Option<String> = None;
    let mut args = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let tag = e.name().as_ref().to_string();

                if tag == "s:Body" || tag == "Body" {
                    in_body = true;
                } else if in_body && action_name.is_none() {
                    // First element inside Body is the action
                    let action = tag.split(':').last().unwrap_or(&tag);
                    action_name = Some(action.to_string());
                } else if in_body && action_name.is_some() {
                    // Child elements are arguments
                    let arg_name = tag.split(':').last().unwrap_or(&tag);

                    // Read text content
                    let mut text_buf = Vec::new();
                    loop {
                        match reader.read_event_into(&mut text_buf) {
                            Ok(Event::Text(t)) => {
                                let value = t.as_ref().to_string();
                                args.push((arg_name.to_string(), value));
                            }
                            Ok(Event::End(_)) => break,
                            Ok(_) => {}
                            Err(_) => break,
                        }
                        text_buf.clear();
                    }
                }
            }
            Ok(Event::End(e)) => {
                let tag = e.name().as_ref().to_string();
                if tag == "s:Body" || tag == "Body" {
                    in_body = false;
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(args)
}

/// Build a SOAP response envelope from action name and return values.
///
/// The response element is the action name + "Response" suffix.
pub fn build_response(action: &str, returns: &[(String, String)], namespace: &str) -> String {
    let mut buf = Vec::new();
    let mut writer = quick_xml::writer::Writer::new_with_indent(&mut buf, b' ', 2);

    let response_name = format!("{}Response", action);

    // s:Envelope
    let mut envelope = BytesStart::new("s:Envelope");
    envelope.push_attribute(("xmlns:s", "http://schemas.xmlsoap.org/soap/envelope/"));
    writer.write_event(Event::Start(envelope)).unwrap();

    // s:Body
    let mut body = BytesStart::new("s:Body");
    body.push_attribute(("xmlns:u", namespace));
    writer.write_event(Event::Start(body)).unwrap();

    // u:ActionResponse
    let mut response = BytesStart::new(&response_name);
    response.push_attribute(("xmlns:u", namespace));
    writer.write_event(Event::Start(response)).unwrap();

    // Return values
    for (key, value) in returns {
        writer
            .write_event(Event::Start(BytesStart::new(key)))
            .unwrap();
        writer
            .write_event(Event::Text(BytesText::new(value)))
            .unwrap();
        writer.write_event(Event::End(BytesEnd::new(key))).unwrap();
    }

    writer
        .write_event(Event::End(BytesEnd::new(&response_name)))
        .unwrap();
    writer
        .write_event(Event::End(BytesEnd::new("s:Body")))
        .unwrap();
    writer
        .write_event(Event::End(BytesEnd::new("s:Envelope")))
        .unwrap();

    String::from_utf8(buf).unwrap()
}

/// Build a SOAP fault envelope for error responses.
pub fn build_fault(error_code: i32, description: &str) -> String {
    let mut buf = Vec::new();
    let mut writer = quick_xml::writer::Writer::new_with_indent(&mut buf, b' ', 2);

    // s:Envelope
    let mut envelope = BytesStart::new("s:Envelope");
    envelope.push_attribute(("xmlns:s", "http://schemas.xmlsoap.org/soap/envelope/"));
    writer.write_event(Event::Start(envelope)).unwrap();

    // s:Body
    writer
        .write_event(Event::Start(BytesStart::new("s:Body")))
        .unwrap();

    // s:Fault
    writer
        .write_event(Event::Start(BytesStart::new("s:Fault")))
        .unwrap();

    // faultcode
    writer
        .write_event(Event::Start(BytesStart::new("faultcode")))
        .unwrap();
    writer
        .write_event(Event::Text(BytesText::new("s:Client")))
        .unwrap();
    writer
        .write_event(Event::End(BytesEnd::new("faultcode")))
        .unwrap();

    // faultstring
    writer
        .write_event(Event::Start(BytesStart::new("faultstring")))
        .unwrap();
    writer
        .write_event(Event::Text(BytesText::new("UPnPError")))
        .unwrap();
    writer
        .write_event(Event::End(BytesEnd::new("faultstring")))
        .unwrap();

    // detail
    writer
        .write_event(Event::Start(BytesStart::new("detail")))
        .unwrap();

    // UPnPError
    let mut upnp_error = BytesStart::new("UPnPError");
    upnp_error.push_attribute(("xmlns", "urn:schemas-upnp-org:control-1-0"));
    writer.write_event(Event::Start(upnp_error)).unwrap();

    // errorCode
    writer
        .write_event(Event::Start(BytesStart::new("errorCode")))
        .unwrap();
    writer
        .write_event(Event::Text(BytesText::new(&error_code.to_string())))
        .unwrap();
    writer
        .write_event(Event::End(BytesEnd::new("errorCode")))
        .unwrap();

    // errorDescription
    writer
        .write_event(Event::Start(BytesStart::new("errorDescription")))
        .unwrap();
    writer
        .write_event(Event::Text(BytesText::new(description)))
        .unwrap();
    writer
        .write_event(Event::End(BytesEnd::new("errorDescription")))
        .unwrap();

    writer
        .write_event(Event::End(BytesEnd::new("UPnPError")))
        .unwrap();
    writer
        .write_event(Event::End(BytesEnd::new("detail")))
        .unwrap();
    writer
        .write_event(Event::End(BytesEnd::new("s:Fault")))
        .unwrap();
    writer
        .write_event(Event::End(BytesEnd::new("s:Body")))
        .unwrap();
    writer
        .write_event(Event::End(BytesEnd::new("s:Envelope")))
        .unwrap();

    String::from_utf8(buf).unwrap()
}
