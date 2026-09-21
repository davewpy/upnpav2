//! DIDL-Lite metadata parsing and serialization.
//!
//! DIDL-Lite is the XML format used for UPnP AV metadata.
//! See UPnP-av-AVTransport-v3-Service §5.7.

use quick_xml::escape::escape as escape_xml_str;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;

// ── Data model ──────────────────────────────────────────────────────────────

/// A single media item inside a DIDL-Lite document.
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub id: String,
    pub parent_id: String,
    pub restricted: bool,
    pub title: String,
    pub creator: String,
    pub artist: String,
    pub album: String,
    pub album_art_uri: Option<String>,
    pub class_: String,
    pub resources: Vec<Resource>,
}

/// A resource (URI) inside a DIDL-Lite item.
#[derive(Debug, Clone, PartialEq)]
pub struct Resource {
    pub protocol_info: String,
    pub uri: String,
    pub duration: Option<String>,
    pub sample_frequency: Option<u32>,
    pub bits_per_sample: Option<u32>,
    pub nr_audio_channels: Option<u32>,
    pub size: Option<u64>,
}

/// A complete DIDL-Lite document.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DidlDocument {
    pub items: Vec<Item>,
}

// ── Parsing ─────────────────────────────────────────────────────────────────

/// Parse a DIDL-Lite XML string into a document.
pub fn parse_didl(xml: &str) -> Result<DidlDocument, String> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut doc = DidlDocument::default();
    let mut current_item: Option<Item> = None;
    let mut in_res = false;
    let mut res_protocol: Option<String> = None;
    let mut res_duration: Option<String> = None;
    let mut res_sample_freq: Option<u32> = None;
    let mut res_bits_per_sample: Option<u32> = None;
    let mut res_nr_audio_channels: Option<u32> = None;
    let mut res_size: Option<u64> = None;
    let mut res_uri_text: String = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = e.name();
                if name == quick_xml::name::QName("item") {
                    current_item = Some(Item {
                        id: attr_val(&e, "id").unwrap_or_default(),
                        parent_id: attr_val(&e, "parentID").unwrap_or_default(),
                        restricted: attr_val(&e, "restricted")
                            .map(|v| v == "1")
                            .unwrap_or(false),
                        title: String::new(),
                        creator: String::new(),
                        artist: String::new(),
                        album: String::new(),
                        album_art_uri: None,
                        class_: String::new(),
                        resources: Vec::new(),
                    });
                } else if name == quick_xml::name::QName("res") {
                    in_res = true;
                    res_protocol = Some(attr_val(&e, "protocolInfo").unwrap_or_default());
                    res_duration = attr_val(&e, "duration");
                    res_sample_freq = attr_val(&e, "sampleFrequency").and_then(|s| s.parse().ok());
                    res_bits_per_sample =
                        attr_val(&e, "bitsPerSample").and_then(|s| s.parse().ok());
                    res_nr_audio_channels =
                        attr_val(&e, "nrAudioChannels").and_then(|s| s.parse().ok());
                    res_size = attr_val(&e, "size").and_then(|s| s.parse().ok());
                    res_uri_text.clear();
                } else if current_item.is_some() {
                    let item = current_item.as_mut().unwrap();
                    if name == quick_xml::name::QName("title")
                        || name == quick_xml::name::QName("dc:title")
                    {
                        item.title = text_content(&mut reader, &mut buf);
                    } else if name == quick_xml::name::QName("creator")
                        || name == quick_xml::name::QName("dc:creator")
                    {
                        item.creator = text_content(&mut reader, &mut buf);
                    } else if name == quick_xml::name::QName("artist")
                        || name == quick_xml::name::QName("upnp:artist")
                    {
                        item.artist = text_content(&mut reader, &mut buf);
                    } else if name == quick_xml::name::QName("album")
                        || name == quick_xml::name::QName("upnp:album")
                    {
                        item.album = text_content(&mut reader, &mut buf);
                    } else if name == quick_xml::name::QName("albumArtURI")
                        || name == quick_xml::name::QName("upnp:albumArtURI")
                    {
                        item.album_art_uri = Some(text_content(&mut reader, &mut buf));
                    } else if name == quick_xml::name::QName("class")
                        || name == quick_xml::name::QName("upnp:class")
                    {
                        item.class_ = text_content(&mut reader, &mut buf);
                    }
                }
            }

            Ok(Event::Text(t)) => {
                if in_res {
                    res_uri_text.push_str(&t);
                }
            }

            Ok(Event::End(e)) => {
                let name = e.name();
                if name == quick_xml::name::QName("res") {
                    in_res = false;
                    if let Some(item) = &mut current_item {
                        item.resources.push(Resource {
                            protocol_info: res_protocol.take().unwrap_or_default(),
                            uri: std::mem::take(&mut res_uri_text),
                            duration: res_duration.take(),
                            sample_frequency: res_sample_freq.take(),
                            bits_per_sample: res_bits_per_sample.take(),
                            nr_audio_channels: res_nr_audio_channels.take(),
                            size: res_size.take(),
                        });
                    }
                } else if name == quick_xml::name::QName("item") {
                    if let Some(item) = current_item.take() {
                        doc.items.push(item);
                    }
                }
            }

            Ok(Event::Eof) => break,
            _ => {}
        }
    }

    Ok(doc)
}

/// Extract an attribute value from an element.
fn attr_val(e: &BytesStart, name: &str) -> Option<String> {
    e.attributes().find_map(|a| {
        let a = a.ok()?;
        if a.key.as_ref() == name {
            Some(a.value.to_string())
        } else {
            None
        }
    })
}

/// Read text content until the next End/Eof event.
fn text_content(reader: &mut Reader<&[u8]>, buf: &mut Vec<u8>) -> String {
    buf.clear();
    loop {
        match reader.read_event_into(buf) {
            Ok(Event::Text(t)) => {
                return t.to_string();
            }
            Ok(Event::End(_) | Event::Eof) => break,
            _ => {}
        }
    }
    String::new()
}

// ── XML utilities ─────────────────────────────────────────────────────────────

/// Escape XML special characters in a string using quick-xml's built-in escape.
///
/// Replaces: `<` → `&lt;`, `>` → `&gt;`, `&` → `&amp;`, `"` → `&quot;`, `'` → `&apos;`
pub fn escape_xml(s: &str) -> String {
    escape_xml_str(s).into_owned()
}

// ── Serialization ───────────────────────────────────────────────────────────

/// Serialize a DIDL-Lite document to XML string.
pub fn serialize_didl(doc: &DidlDocument) -> String {
    let mut out = Vec::new();
    let mut writer = Writer::new_with_indent(&mut out, b' ', 2);

    let mut envelope = BytesStart::new("DIDL-Lite");
    envelope.push_attribute(("xmlns:dc", "http://purl.org/dc/elements/1.1/"));
    envelope.push_attribute(("xmlns:upnp", "urn:schemas-upnp-org:metadata-1-0/upnp/"));
    envelope.push_attribute(("xmlns", "urn:schemas-upnp-org:metadata-1-0/DIDL-Lite/"));
    writer.write_event(Event::Start(envelope)).ok();

    for item in &doc.items {
        serialize_item(&mut writer, item);
    }

    writer
        .write_event(Event::End(BytesEnd::new("DIDL-Lite")))
        .ok();

    std::str::from_utf8(&out).unwrap_or("").to_string()
}

fn serialize_item(writer: &mut Writer<&mut Vec<u8>>, item: &Item) {
    let mut item_start = BytesStart::new("item");
    item_start.push_attribute(("id", item.id.as_str()));
    item_start.push_attribute(("parentID", item.parent_id.as_str()));
    item_start.push_attribute(("restricted", if item.restricted { "1" } else { "0" }));
    writer.write_event(Event::Start(item_start)).ok();

    write_element(writer, "dc:title", &item.title);
    write_element(writer, "dc:creator", &item.creator);
    write_element(writer, "upnp:class", &item.class_);

    for res in &item.resources {
        let mut res_start = BytesStart::new("res");
        res_start.push_attribute(("protocolInfo", res.protocol_info.as_str()));
        writer.write_event(Event::Start(res_start)).ok();
        writer
            .write_event(Event::Text(BytesText::new(&res.uri)))
            .ok();
        writer.write_event(Event::End(BytesEnd::new("res"))).ok();
    }

    writer.write_event(Event::End(BytesEnd::new("item"))).ok();
}

fn write_element(writer: &mut Writer<&mut Vec<u8>>, name: &str, content: &str) {
    writer.write_event(Event::Start(BytesStart::new(name))).ok();
    writer
        .write_event(Event::Text(BytesText::new(content)))
        .ok();
    writer.write_event(Event::End(BytesEnd::new(name))).ok();
}

// ── Helpers ─────────────────────────────────────────────────────────────────

/// Extract a single arg value from a parsed args list.
pub fn arg_value(args: &[(String, String)], key: &str) -> Option<String> {
    args.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone())
}
