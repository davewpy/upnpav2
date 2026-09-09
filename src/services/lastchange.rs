/// Shared LastChange XML builder for UPnP AV services.
///
/// All three services (AVTransport, ConnectionManager, RenderingControl)
/// generate identical LastChange XML — only the event namespace differs.
/// This module provides a single implementation.
use crate::didl::escape_xml;

/// Build the GENA LastChange XML propertyset.
///
/// - `event_namespace`: the metadata namespace for this service
///   - AVTransport: `urn:schemas-upnp-org:metadata-1-0/AVT/`
///   - ConnectionManager: `urn:schemas-upnp-org:metadata-1-0/CM/`
///   - RenderingControl: `urn:schemas-upnp-org:metadata-1-0/RC/`
/// - `evented`: list of (state_variable_name, value) pairs that changed
///
/// Returns a complete `<e:propertyset>` document per UPnP GENA spec.
pub fn build_last_change(event_namespace: &str, evented: &[(String, String)]) -> String {
    if evented.is_empty() {
        return format!(
            r#"<?xml version="1.0"?>
<e:propertyset xmlns:e="urn:schemas-upnp-org:event-1-0">
  <e:property>
    <LastChange>
      <Event xmlns="{}">
      </Event>
    </LastChange>
  </e:property>
</e:propertyset>"#,
            event_namespace
        );
    }

    let mut xml = format!(
        r#"<?xml version="1.0"?>
<e:propertyset xmlns:e="urn:schemas-upnp-org:event-1-0">
  <e:property>
    <LastChange>
      <Event xmlns="{}">
        <PropertyChange>"#,
        event_namespace
    );

    for (var_name, value) in evented {
        xml.push_str(&format!(
            "<{}>{}</{}>",
            escape_xml(var_name),
            escape_xml(value),
            escape_xml(var_name)
        ));
    }

    xml.push_str(
        "</PropertyChange>\n      </Event>\n    </LastChange>\n  </e:property>\n</e:propertyset>",
    );
    xml
}
