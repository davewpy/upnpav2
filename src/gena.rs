use std::collections::HashMap;
use std::net::UdpSocket;
use std::time::Instant;

use crate::types::upnp;

/// A single subscriber to events.
pub struct Subscriber {
    /// Subscription UUID (uuid:xxx)
    pub sid: String,
    /// Callback URL for NOTIFY delivery
    pub callback_url: String,
    /// Requested timeout in seconds
    pub timeout: u32,
    /// When this subscription expires
    pub expires_at: Instant,
    /// Event sequence counter (32-bit, wraps at 4294967295)
    pub seq: u32,
}

/// GENA event publisher — manages subscriptions and delivers events.
///
/// Each service (AVTransport, RenderingControl, ConnectionManager) owns its own
/// EventPublisher instance. Supports both unicast (HTTP POST) and multicast
/// (UDP broadcast to 239.255.255.246:7900).
pub struct EventPublisher {
    subscribers: HashMap<String, Subscriber>,
    service_url: String,
    multicast_enabled: bool,
    multicast_socket: Option<UdpSocket>,
}

impl EventPublisher {
    /// Create a new EventPublisher.
    ///
    /// - `service_url`: the eventSubURL for this service (e.g., `/upnp/event/AVTransport1`)
    /// - `multicast_enabled`: whether to also broadcast events to the multicast address
    pub fn new(service_url: &str, multicast_enabled: bool) -> Self {
        let multicast_socket = if multicast_enabled {
            UdpSocket::bind("0.0.0.0:0").ok()
        } else {
            None
        };

        EventPublisher {
            subscribers: HashMap::new(),
            service_url: service_url.to_string(),
            multicast_enabled,
            multicast_socket,
        }
    }

    /// Return the eventSubURL for this publisher.
    pub fn service_url(&self) -> &str {
        &self.service_url
    }

    /// Subscribe — create a new subscription.
    ///
    /// Returns `(sid, actual_timeout)`. The actual timeout may differ from requested.
    /// Minimum recommended timeout is 1800 seconds.
    pub fn subscribe(&mut self, callback_url: &str, requested_timeout: u32) -> (String, u32) {
        let sid = uuid::Uuid::new_v4().to_string();
        let sid = format!("uuid:{}", sid);
        // Enforce minimum recommended timeout
        let actual_timeout = requested_timeout.max(1800);
        let expires_at = Instant::now() + std::time::Duration::from_secs(actual_timeout as u64);

        let subscriber = Subscriber {
            sid: sid.clone(),
            callback_url: callback_url.to_string(),
            timeout: requested_timeout,
            expires_at,
            seq: 0,
        };

        self.subscribers.insert(sid.clone(), subscriber);
        (sid, actual_timeout)
    }

    /// Renew — extend an existing subscription's lifetime.
    ///
    /// If timeout is None, uses the subscriber's existing timeout.
    /// Returns error if sid not found or expired.
    pub fn renew(&mut self, sid: &str, timeout: Option<u32>) -> Result<(), upnp::Error> {
        let subscriber = self
            .subscribers
            .get_mut(sid)
            .ok_or(upnp::Error::InvalidAction)?;

        if Instant::now() >= subscriber.expires_at {
            return Err(upnp::Error::InvalidAction);
        }

        let new_timeout = timeout.unwrap_or(subscriber.timeout);
        subscriber.timeout = new_timeout;
        subscriber.expires_at = Instant::now() + std::time::Duration::from_secs(new_timeout as u64);

        Ok(())
    }

    /// Unsubscribe — remove a subscription.
    ///
    /// Returns error if sid not found.
    pub fn unsubscribe(&mut self, sid: &str) -> Result<(), upnp::Error> {
        self.subscribers
            .remove(sid)
            .ok_or(upnp::Error::InvalidAction)?;
        Ok(())
    }

    /// Notify — send event to all active subscribers.
    ///
    /// - `properties`: list of (var_name, value) pairs to include in the event body
    /// - Returns a list of results (one per subscriber)
    ///
    /// For each subscriber:
    ///   1. Increment seq (wraps at 4294967295)
    ///   2. Build NOTIFY headers
    ///   3. Build event body XML
    ///   4. Send HTTP POST to callback URL (unicast)
    ///   5. If multicast enabled, broadcast to multicast address
    pub fn notify(&mut self, properties: &[(String, String)]) -> Vec<Result<(), String>> {
        let event_body = Self::build_event_body(properties);
        let results: Vec<Result<(), String>> = self
            .subscribers
            .iter_mut()
            .filter_map(|(sid, subscriber)| {
                // Skip expired subscribers
                if Instant::now() >= subscriber.expires_at {
                    return None;
                }

                // Increment seq (wraps at 4294967295)
                subscriber.seq = subscriber.seq.wrapping_add(1);
                if subscriber.seq == 0 {
                    subscriber.seq = 1;
                }

                let seq = subscriber.seq;

                // Build NOTIFY headers
                let headers = Self::build_notify_headers(sid, seq);

                // Send to unicast subscriber
                let unicast_result =
                    Self::send_notify_unicast(&subscriber.callback_url, &headers, &event_body);

                // Send to multicast if enabled
                let multicast_result = if self.multicast_enabled {
                    Self::send_notify_multicast(&event_body)
                } else {
                    Ok(())
                };

                match (unicast_result, multicast_result) {
                    (Ok(()), Ok(())) => Some(Ok(())),
                    (Err(e), _) => Some(Err(e)),
                    (_, Err(e)) => Some(Err(e)),
                }
            })
            .collect();

        results
    }

    /// Cleanup — remove all expired subscriptions.
    ///
    /// Returns list of removed subscription IDs.
    pub fn cleanup_expired(&mut self) -> Vec<String> {
        let now = Instant::now();
        let expired: Vec<String> = self
            .subscribers
            .iter()
            .filter(|(_, sub)| now >= sub.expires_at)
            .map(|(sid, _)| sid.clone())
            .collect();

        for sid in &expired {
            self.subscribers.remove(sid);
        }

        expired
    }

    /// Build the GENA event body XML.
    ///
    /// Format: `<e:propertyset xmlns:e="urn:schemas-upnp-org:event-1-0">...</e:propertyset>`
    pub fn build_event_body(properties: &[(String, String)]) -> String {
        let mut xml = String::from(
            r#"<?xml version="1.0"?>
<e:propertyset xmlns:e="urn:schemas-upnp-org:event-1-0">"#,
        );

        for (var_name, value) in properties {
            xml.push_str(&format!(
                "<e:property><{}>{}</{}></e:property>",
                var_name, value, var_name
            ));
        }

        xml.push_str("</e:propertyset>");
        xml
    }

    /// Build NOTIFY headers string.
    ///
    /// Headers: NT, NTS, SID, SEQ, CONTENT-TYPE
    pub fn build_notify_headers(sid: &str, seq: u32) -> String {
        format!(
            "NT: upnp:event\r\nNTS: upnp:propchange\r\nSID: {}\r\nSEQ: {}\r\nCONTENT-TYPE: text/xml; charset=\"utf-8\"\r\n",
            sid, seq
        )
    }

    /// Send NOTIFY via HTTP POST to a callback URL (unicast).
    fn send_notify_unicast(callback_url: &str, headers: &str, body: &str) -> Result<(), String> {
        // TODO: implement HTTP POST using reqwest or hyper
        // For now, return Ok as a placeholder
        let _ = (callback_url, headers, body);
        Ok(())
    }

    /// Send NOTIFY via UDP broadcast (multicast).
    fn send_notify_multicast(body: &str) -> Result<(), String> {
        // TODO: implement UDP broadcast to 239.255.255.246:7900
        // For now, return Ok as a placeholder
        let _ = body;
        Ok(())
    }
}
