//! SSDP (Simple Service Discovery Protocol) — UPnP 2.0 discovery layer.
//!
//! Handles multicast UDP communication on 239.255.255.250:1900.
//!
//! Per UPnP 2.0 spec (section 1.1.3):
//! - NOTIFY * (alive): device sends multicast UDP
//! - All messages use HTTPU (HTTP over UDP) format

use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use tokio::net::UdpSocket;
use tracing::{debug, error, warn};

/// Handle for SSDP — M-SEARCH listening loop and announcement sending.
///
/// `stop()` sets the shutdown flag and sends ssdp:byebye synchronously.
/// The async tasks observe the flag and exit gracefully.
pub struct Server {
    shutdown: Arc<AtomicBool>,
    /// Cloned std socket for synchronous byebye sends from sync context.
    byebye_socket: std::net::UdpSocket,
}

impl Server {
    /// Signal shutdown and send ssdp:byebye for all notification types.
    ///
    /// The async tasks observe the shutdown flag and exit on their next
    /// iteration. Byebye is sent synchronously before returning.
    pub fn stop(&self, device: &crate::config::SsdpDevice) {
        self.shutdown.store(true, Ordering::Relaxed);
        self.send_byebye(device);
    }

    /// Send ssdp:update notifications for all notification types with a new boot ID.
    /// Used when an interface becomes available/unavailable or IP changes.
    pub fn send_update(&self, device: &crate::config::SsdpDevice, new_boot_id: u32) {
        let msgs = build_update_notifications(device, new_boot_id);
        let addr: SocketAddr = format!("{}:{}", device.ssdp_multicast_addr, device.ssdp_port)
            .parse()
            .unwrap();
        for msg in msgs {
            if let Err(e) = self.byebye_socket.send_to(msg.as_bytes(), addr) {
                error!(error = %e, "Failed to send SSDP update");
            }
        }
    }

    /// Send ssdp:byebye for root device, device type, and each service.
    fn send_byebye(&self, device: &crate::config::SsdpDevice) {
        let msgs = build_byebye_notifications(device);
        let addr: SocketAddr = format!("{}:{}", device.ssdp_multicast_addr, device.ssdp_port)
            .parse()
            .unwrap();
        for msg in msgs {
            if let Err(e) = self.byebye_socket.send_to(msg.as_bytes(), addr) {
                error!(error = %e, "Failed to send SSDP byebye");
            }
        }
        debug!("byebye sent");
    }
}

/// Build all NOTIFY * alive messages for a device.
///
/// Returns one message per notification type:
/// 1. root device: uuid::<UDN>::upnp:rootdevice (with CACHE-CONTROL + LOCATION)
/// 2. device type: uuid::<UDN>::<device_type> (without CACHE-CONTROL + LOCATION)
/// 3. each service: uuid::<UDN>::<service_namespace>:<version> (without CACHE-CONTROL + LOCATION)
fn build_alive_notifications(device: &crate::config::SsdpDevice) -> Vec<String> {
    let mut msgs = Vec::new();

    // 1. Root device alive — spec requires CACHE-CONTROL + LOCATION
    msgs.push(build_rootdevice_alive(device));

    // 2. Device type alive — spec does NOT require CACHE-CONTROL + LOCATION
    msgs.push(build_nt_alive(
        device,
        &device.device_type,
        &format!("uuid:{}::{}", device.udn, device.device_type),
    ));

    // 3. Service alive — spec does NOT require CACHE-CONTROL + LOCATION
    for svc in &device.services {
        let ns = svc.full_namespace();
        msgs.push(build_nt_alive(
            device,
            &ns,
            &format!("uuid:{}::{}", device.udn, ns),
        ));
    }

    msgs
}

/// Build all NOTIFY * byebye messages for a device.
///
/// Returns one message per notification type (same as alive).
/// Per spec section 3.3: byebye MUST NOT include CACHE-CONTROL, LOCATION, BOOTID, or CONFIGID.
fn build_byebye_notifications(device: &crate::config::SsdpDevice) -> Vec<String> {
    let mut msgs = Vec::new();

    // 1. Root device byebye
    msgs.push(build_byebye_notification(
        device,
        "upnp:rootdevice",
        &format!("uuid:{}::upnp:rootdevice", device.udn),
    ));

    // 2. Device type byebye
    msgs.push(build_byebye_notification(
        device,
        &device.device_type,
        &format!("uuid:{}::{}", device.udn, device.device_type),
    ));

    // 3. Service byebye
    for svc in &device.services {
        let ns = svc.full_namespace();
        msgs.push(build_byebye_notification(
            device,
            &ns,
            &format!("uuid:{}::{}", device.udn, ns),
        ));
    }

    msgs
}

/// Build all NOTIFY * update messages for a device with a new boot ID.
///
/// Returns one message per notification type with NEXTBOOTID.UPNP.ORG header.
fn build_update_notifications(device: &crate::config::SsdpDevice, new_boot_id: u32) -> Vec<String> {
    let mut msgs = Vec::new();

    // 1. Root device update
    msgs.push(build_update_notification(
        device,
        "upnp:rootdevice",
        &format!("uuid:{}::upnp:rootdevice", device.udn),
        new_boot_id,
    ));

    // 2. Device type update
    msgs.push(build_update_notification(
        device,
        &device.device_type,
        &format!("uuid:{}::{}", device.udn, device.device_type),
        new_boot_id,
    ));

    // 3. Service updates
    for svc in &device.services {
        let ns = svc.full_namespace();
        msgs.push(build_update_notification(
            device,
            &ns,
            &format!("uuid:{}::{}", device.udn, ns),
            new_boot_id,
        ));
    }

    msgs
}

/// Build a NOTIFY * update message.
fn build_update_notification(
    device: &crate::config::SsdpDevice,
    nt: &str,
    usn: &str,
    new_boot_id: u32,
) -> String {
    format!(
        "NOTIFY * HTTP/1.1\r\n\
         HOST: {}:{}\r\n\
         NT: {}\r\n\
         NTS: ssdp:update\r\n\
         SERVER: {}\r\n\
         USN: {}\r\n\
         NEXTBOOTID.UPNP.ORG: {}\r\n\
         \r\n",
        device.ssdp_multicast_addr, device.ssdp_port, nt, device.server_string, usn, new_boot_id
    )
}

/// Build a NOTIFY * alive message for upnp:rootdevice.
///
/// Per UPnP 2.0 spec section 3.2: rootdevice alive MUST include
/// CACHE-CONTROL and LOCATION headers.
fn build_rootdevice_alive(device: &crate::config::SsdpDevice) -> String {
    let mut msg = format!(
        "NOTIFY * HTTP/1.1\r\n\
         HOST: {}:{}\r\n\
         CACHE-CONTROL: max-age={}\r\n\
         LOCATION: {}\r\n\
         NT: upnp:rootdevice\r\n\
         NTS: ssdp:alive\r\n\
         SERVER: {}\r\n\
         USN: uuid:{}::upnp:rootdevice\r\n\
         BOOTID.UPNP.ORG: {}\r\n\
         CONFIGID.UPNP.ORG: {}\r\n",
        device.ssdp_multicast_addr,
        device.ssdp_port,
        device.max_age,
        device.description_url,
        device.server_string,
        device.udn,
        device.boot_id,
        device.config_id,
    );
    if device.ssdp_port != 1900 {
        msg.push_str(&format!("SEARCHPORT.UPNP.ORG: {}\r\n", device.ssdp_port));
    }
    msg.push_str("\r\n");
    msg
}

/// Build a NOTIFY * alive message for a device-type or service-type NT.
///
/// Per UPnP 2.0 spec section 3.2: device-type and service-type alive
/// MUST NOT include CACHE-CONTROL or LOCATION headers.
fn build_nt_alive(device: &crate::config::SsdpDevice, nt: &str, usn: &str) -> String {
    let mut msg = format!(
        "NOTIFY * HTTP/1.1\r\n\
         HOST: {}:{}\r\n\
         NT: {}\r\n\
         NTS: ssdp:alive\r\n\
         SERVER: {}\r\n\
         USN: {}\r\n\
         BOOTID.UPNP.ORG: {}\r\n\
         CONFIGID.UPNP.ORG: {}\r\n",
        device.ssdp_multicast_addr,
        device.ssdp_port,
        nt,
        device.server_string,
        usn,
        device.boot_id,
        device.config_id,
    );
    if device.ssdp_port != 1900 {
        msg.push_str(&format!("SEARCHPORT.UPNP.ORG: {}\r\n", device.ssdp_port));
    }
    msg.push_str("\r\n");
    msg
}

/// Build a NOTIFY * byebye message.
///
/// Per UPnP 2.0 spec section 3.3: byebye MUST NOT include CACHE-CONTROL,
/// LOCATION, BOOTID, or CONFIGID headers.
fn build_byebye_notification(device: &crate::config::SsdpDevice, nt: &str, usn: &str) -> String {
    format!(
        "NOTIFY * HTTP/1.1\r\n\
         HOST: {}:{}\r\n\
         NT: {}\r\n\
         NTS: ssdp:byebye\r\n\
         SERVER: {}\r\n\
         USN: {}\r\n\
         \r\n",
        device.ssdp_multicast_addr, device.ssdp_port, nt, device.server_string, usn
    )
}

/// Parse an M-SEARCH request and return headers.
///
/// Returns None if the request is not a valid M-SEARCH * HTTP/1.1.
fn parse_msearch_request(data: &[u8]) -> Option<Vec<(String, String)>> {
    let text = std::str::from_utf8(data).ok()?;
    let mut lines = text.lines();

    // Parse start-line: "M-SEARCH * HTTP/1.1"
    let start_line = lines.next()?;
    let parts: Vec<&str> = start_line.split_whitespace().collect();
    if parts.len() < 3 || parts[0] != "M-SEARCH" || parts[2] != "HTTP/1.1" {
        return None;
    }

    // Parse headers
    let mut headers = Vec::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(':') {
            headers.push((key.trim().to_string(), value.trim().to_string()));
        }
    }

    Some(headers)
}

/// Extract a header value by name (case-insensitive).
fn get_header<'a>(headers: &'a [(String, String)], name: &str) -> Option<&'a str> {
    headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(name))
        .map(|(_, v)| v.as_str())
}

/// Build HTTP 200 OK responses to an M-SEARCH request.
///
/// Returns one response per matching USN. Per UPnP 2.0 spec section 3.6:
/// - ST header must echo the request ST value
/// - EXT header must be present with no value (no trailing space)
fn build_search_responses(device: &crate::config::SsdpDevice, st: &str) -> Vec<String> {
    // Determine which USNs to respond with based on the search target
    let mut usns = Vec::new();

    match st {
        "ssdp:all" => {
            usns.push(format!("uuid:{}::upnp:rootdevice", device.udn));
            usns.push(format!("uuid:{}::{}", device.udn, device.device_type));
            for svc in &device.services {
                let ns = svc.full_namespace();
                usns.push(format!("uuid:{}::{}", device.udn, ns));
            }
        }
        "upnp:rootdevice" => {
            usns.push(format!("uuid:{}::upnp:rootdevice", device.udn));
        }
        _ => {
            // Check if ST matches device type or any service type
            let matches_device = st == device.device_type;
            let matches_service = device.services.iter().any(|svc| st == svc.full_namespace());

            if matches_device {
                usns.push(format!("uuid:{}::{}", device.udn, device.device_type));
            } else if matches_service {
                usns.push(format!("uuid:{}::{}", device.udn, st));
            } else {
                // No match — don't respond
                return Vec::new();
            }
        }
    }

    // Build one response per matching USN, echoing the request ST
    let date = chrono::Utc::now()
        .format("%a, %d %b %Y %H:%M:%S GMT")
        .to_string();

    usns.into_iter()
        .map(|usn| {
            tracing::debug!(usn = %usn, st = %st, "response");
            let mut resp = format!(
                "HTTP/1.1 200 OK\r\n\
                 CACHE-CONTROL: max-age={}\r\n\
                 DATE: {}\r\n\
                 EXT:\r\n\
                 LOCATION: {}\r\n\
                 SERVER: {}\r\n\
                 ST: {}\r\n\
                 USN: {}\r\n\
                 BOOTID.UPNP.ORG: {}\r\n\
                 CONFIGID.UPNP.ORG: {}\r\n",
                device.max_age,
                date,
                device.description_url,
                device.server_string,
                st,
                usn,
                device.boot_id,
                device.config_id,
            );

            if device.ssdp_port != 1900 {
                resp.push_str(&format!("SEARCHPORT.UPNP.ORG: {}\r\n", device.ssdp_port));
            }
            resp.push_str("\r\n");
            resp
        })
        .collect()
}

/// Start the SSDP server: bind UDP, join multicast, spawn M-SEARCH listener
/// and alive notification tasks.
///
/// Returns a `Server` handle that can be used to stop the server and send
/// byebye/update notifications.
pub fn start(device: crate::config::SsdpDevice) -> Server {
    let multicast_addr: SocketAddr = format!("{}:{}", device.ssdp_multicast_addr, device.ssdp_port)
        .parse()
        .expect("invalid SSDP multicast address");

    // Bind std socket for sync byebye sends
    let bind_addr: SocketAddr = format!("0.0.0.0:{}", device.ssdp_port).parse().unwrap();
    let std_socket = std::net::UdpSocket::bind(bind_addr)
        .unwrap_or_else(|e| panic!("failed to bind SSDP socket to {}: {}", bind_addr, e));

    // Set TTL for multicast (default 2 per UPnP 2.0 spec)
    if let Ok(ip) = multicast_addr.ip().to_string().parse::<Ipv4Addr>() {
        if let Err(e) = std_socket.set_multicast_ttl_v4(2) {
            warn!(error = %e, "failed to set multicast TTL");
        }
        // Join multicast group on the device's interface
        if let Ok(iface_ip) = device.ip_addr.to_string().parse::<Ipv4Addr>() {
            if let Err(e) = std_socket.join_multicast_v4(&ip, &iface_ip) {
                warn!(error = %e, ip = ?device.ip_addr, "failed to join multicast group");
            }
        }
    }

    // Convert to tokio socket for async tasks
    std_socket
        .set_nonblocking(true)
        .expect("set_nonblocking failed");
    let tokio_socket =
        Arc::new(UdpSocket::from_std(std_socket.try_clone().unwrap()).expect("from_std failed"));

    let shutdown = Arc::new(AtomicBool::new(false));

    // Spawn M-SEARCH listener
    let msearch_socket = Arc::clone(&tokio_socket);
    msearch_listener(msearch_socket, device.clone(), shutdown.clone());

    // Spawn alive notification loop
    let announce_socket = Arc::clone(&tokio_socket);
    notify_loop(
        announce_socket,
        device.clone(),
        shutdown.clone(),
        multicast_addr,
    );

    Server {
        shutdown,
        byebye_socket: std_socket,
    }
}

/// M-SEARCH listener loop: receive requests, validate, respond.
fn msearch_listener(
    socket: Arc<UdpSocket>,
    device: crate::config::SsdpDevice,
    shutdown: Arc<AtomicBool>,
) {
    tokio::spawn(async move {
        let mut buf = [0u8; 65535];
        loop {
            if shutdown.load(Ordering::Relaxed) {
                debug!("listener shutting down");
                break;
            }
            match socket.recv_from(&mut buf).await {
                Ok((len, peer)) => {
                    let text = match std::str::from_utf8(&buf[..len]) {
                        Ok(t) => t,
                        Err(e) => {
                            debug!(error = %e, peer = %peer, "invalid UTF-8 payload");
                            continue;
                        }
                    };

                    let headers = match parse_msearch_request(text.as_bytes()) {
                        Some(h) => h,
                        None => continue,
                    };

                    // Check MAN header — spec requires quoted value
                    let man = get_header(&headers, "MAN");
                    if man != Some("\"ssdp:discover\"") {
                        debug!(?man, "invalid MAN header, ignoring");
                        continue;
                    }

                    // Check ST header
                    let st = get_header(&headers, "ST").unwrap_or("").to_string();
                    let mx_str = get_header(&headers, "MX").unwrap_or("0");
                    let mx: u64 = match mx_str.parse::<u64>() {
                        Ok(0..=5) => mx_str.parse().unwrap(),
                        _ => {
                            debug!(mx = %mx_str, "invalid MX, ignoring");
                            continue;
                        }
                    };

                    debug!(st = %st, mx = mx, peer = %peer, "received");

                    // Spawn a task per request so delays don't block other requests
                    let socket = Arc::clone(&socket);
                    let device = device.clone();
                    tokio::spawn(async move {
                        // Random delay between 0 and MX seconds (spec requirement)
                        // MX=0 means respond immediately
                        if mx > 0 {
                            let delay_ms = fastrand::u64(0..(mx + 1) * 1000);
                            tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                        }

                        // Build and send all matching responses
                        let responses = build_search_responses(&device, &st);
                        for resp in responses {
                            tracing::trace!(resp = %resp, "response");
                            if let Err(e) = socket.send_to(resp.as_bytes(), peer).await {
                                error!(error = %e, peer = %peer, "failed to send response");
                            } else {
                                debug!(peer = %peer, "response");
                            }
                        }
                    });
                }
                Err(e) => {
                    error!(error = %e, "recv_from error");
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        }
    });
}

/// Alive notification loop: initial 3 sets + periodic repeats with jitter.
fn notify_loop(
    socket: Arc<UdpSocket>,
    device: crate::config::SsdpDevice,
    shutdown: Arc<AtomicBool>,
    multicast_addr: SocketAddr,
) {
    tokio::spawn(async move {
        let max_age = device.max_age as u64;

        // Initial announcement: 3 sets with spreading delays
        for set in 0..3 {
            if shutdown.load(Ordering::Relaxed) {
                break;
            }

            // Random delay 0-100ms before each set
            let initial_delay = Duration::from_millis(fastrand::u64(0..101));
            tokio::time::sleep(initial_delay).await;

            // Send alive notifications
            let msgs = build_alive_notifications(&device);
            for msg in msgs {
                if let Err(e) = socket.send_to(msg.as_bytes(), multicast_addr).await {
                    error!(error = %e, "failed to send initial alive announcement");
                }
            }
            debug!(set = set + 1, "initial alive set {} sent", set + 1);

            // Spread delay between sets (increases linearly)
            if set < 2 {
                let spread = Duration::from_millis(100 * (set as u64 + 1));
                tokio::time::sleep(spread).await;
            }
        }

        // Repeat announcements: every (max_age / 2) seconds ± 10% jitter
        loop {
            if shutdown.load(Ordering::Relaxed) {
                break;
            }

            let base = max_age / 2;
            let jitter = base as i64 / 10;
            let offset = fastrand::i64(-jitter..=jitter);
            let interval = Duration::from_secs((base as i64 + offset) as u64);
            tokio::time::sleep(interval).await;

            let msgs = build_alive_notifications(&device);
            for msg in msgs {
                if let Err(e) = socket.send_to(msg.as_bytes(), multicast_addr).await {
                    error!(error = %e, "failed to send repeat alive announcement");
                }
            }
            debug!("repeat alive announcement sent");
        }
    });
}
