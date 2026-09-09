//! HTTP transport — pure server (inbound) + client (outbound).
//!
//! This module knows nothing about UPnP, SOAP, or GENA.
//! It only hosts routes and sends HTTP requests.

use std::net::SocketAddr;
use std::time::SystemTime;

use axum::Router;
use axum::body::Body;
use axum::http::StatusCode;
use axum::http::response::Response;

// ---------------------------------------------------------------------------
// Enums — handlers can only use these allowed content types and cache policies
// ---------------------------------------------------------------------------

/// MIME content type for UPnP HTTP responses.
pub enum ContentType {
    XmlUtf8,
    JsonUtf8,
    PlainUtf8,
    ImagePng,
    ImageJpeg,
    ImageSvg,
    Custom(String),
}

impl ContentType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::XmlUtf8 => "text/xml; charset=\"utf-8\"",
            Self::JsonUtf8 => "application/json; charset=\"utf-8\"",
            Self::PlainUtf8 => "text/plain; charset=utf-8",
            Self::ImagePng => "image/png",
            Self::ImageJpeg => "image/jpeg",
            Self::ImageSvg => "image/svg+xml",
            Self::Custom(s) => s,
        }
    }
}

/// HTTP Cache-Control policy.
pub enum CacheControl {
    MaxAge(u32),
    NoCache,
    NoStore,
    Public,
    Private,
    None,
}

impl CacheControl {
    pub fn as_header(&self) -> Option<String> {
        match self {
            Self::MaxAge(n) => Some(format!("max-age={}", n)),
            Self::NoCache => Some("no-cache".to_string()),
            Self::NoStore => Some("no-store".to_string()),
            Self::Public => Some("public".to_string()),
            Self::Private => Some("private".to_string()),
            Self::None => None,
        }
    }
}

// ---------------------------------------------------------------------------
// UpnpResponse — the single return type for all UPnP HTTP handlers
// ---------------------------------------------------------------------------

/// A UPnP-compliant HTTP response.
///
/// Handlers build this and return it from route closures.
/// `Response::builder()` is used so that `Server` and `Date` headers
/// can be set alongside `Content-Type` and `Cache-Control`.
pub struct UpnpResponse {
    pub status: StatusCode,
    pub content_type: ContentType,
    pub cache_control: CacheControl,
    pub server: String,
    pub body: Body,
}

impl UpnpResponse {
    /// Build an axum `Response` with all UPnP headers set.
    pub fn build(self) -> Response<Body> {
        let mut builder = Response::builder()
            .status(self.status)
            .header("Content-Type", self.content_type.as_str())
            .header("Server", self.server);

        if let Some(val) = self.cache_control.as_header() {
            builder = builder.header("Cache-Control", val);
        }

        builder
            .header("Date", date_header())
            .body(self.body)
            .unwrap_or_else(|_| {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::empty())
                    .unwrap_or_default()
            })
    }
}

/// Generate a UPnP-compliant `Date` header value.
pub fn date_header() -> String {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| {
            // RFC 7231 format: Sun, 06 Nov 1994 08:49:37 GMT
            // Simplified approximation using epoch seconds
            let secs = d.as_secs() % 86400;
            let days = d.as_secs() / 86400;
            let hours = secs / 3600;
            let mins = (secs % 3600) / 60;
            let seconds = secs % 60;
            format!("{:02} {:02}:{:02}:{:02} GMT", days, hours, mins, seconds)
        })
        .unwrap_or_else(|_| "0 00:00:00 GMT".to_string())
}

// ---------------------------------------------------------------------------
// HttpHandler trait — unchanged
// ---------------------------------------------------------------------------

/// Trait for HTTP route handlers.
///
/// Implementors declare their URL path and provide a registration
/// function that adds their route to the router.
pub trait HttpHandler: Send + Sync + 'static {
    /// The URL path this handler responds to.
    fn path(&self) -> &'static str;

    /// Register this handler's route on the given router.
    /// Returns the modified router with the new route added.
    fn register(self, router: Router) -> Router;
}

// ---------------------------------------------------------------------------
// HttpServer — holds Router, server info, and serves requests
// ---------------------------------------------------------------------------

/// HTTP server that holds an axum Router.
pub struct HttpServer {
    router: Router,
    addr: SocketAddr,
    server_info: String,
}

impl HttpServer {
    /// Create a new server bound to the given address.
    ///
    /// `server_info` is the value used for the UPnP SERVER header.
    pub fn new(addr: SocketAddr, server_info: &str) -> Self {
        Self {
            router: Router::new(),
            addr,
            server_info: server_info.to_string(),
        }
    }

    /// Register a handler on the server's router.
    pub fn register<H: HttpHandler>(mut self, handler: H) -> Self {
        self.router = handler.register(self.router);
        self
    }

    /// Consume the server and return the built router.
    pub fn build(self) -> Router {
        self.router
    }

    /// The SERVER header value used by handlers.
    pub fn server_info(&self) -> &str {
        &self.server_info
    }

    /// Bind a listener and serve requests.
    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let listener = tokio::net::TcpListener::bind(self.addr).await?;
        tracing::info!("server started on {}", self.addr);
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}
