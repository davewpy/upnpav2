// ===========================================================================
// UPnP AVTransport Domain Enums
// ===========================================================================

use crate::types::upnp::ServiceVersion;

/// AVTransport service version
pub const V3: ServiceVersion = ServiceVersion::V3;

/// Transport state per UPnP-av-AVTransport-v3 spec §4.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransportState {
    Stopped,
    Playing,
    Transitioning,
    PausedPlayback,
    NoMediaPresent,
}

impl TransportState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Stopped => "STOPPED",
            Self::Playing => "PLAYING",
            Self::Transitioning => "TRANSITIONING",
            Self::PausedPlayback => "PAUSED_PLAYBACK",
            Self::NoMediaPresent => "NO_MEDIA_PRESENT",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "STOPPED" => Some(Self::Stopped),
            "PLAYING" => Some(Self::Playing),
            "TRANSITIONING" => Some(Self::Transitioning),
            "PAUSED_PLAYBACK" => Some(Self::PausedPlayback),
            "NO_MEDIA_PRESENT" => Some(Self::NoMediaPresent),
            _ => None,
        }
    }
}

/// Transport status per UPnP-av-AVTransport-v3 spec §4.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransportStatus {
    Ok,
    ErrorOccurred,
}

impl TransportStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::ErrorOccurred => "ERROR_OCCURRED",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "OK" => Some(Self::Ok),
            "ERROR_OCCURRED" => Some(Self::ErrorOccurred),
            _ => None,
        }
    }
}

/// Current play mode per UPnP-av-AVTransport-v3 spec §4.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayMode {
    Normal,
    Shuffle,
    RepeatOne,
    RepeatAll,
    Random,
    Direct1,
    Intro,
}

impl PlayMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => "NORMAL",
            Self::Shuffle => "SHUFFLE",
            Self::RepeatOne => "REPEAT_ONE",
            Self::RepeatAll => "REPEAT_ALL",
            Self::Random => "RANDOM",
            Self::Direct1 => "DIRECT_1",
            Self::Intro => "INTRO",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "NORMAL" => Some(Self::Normal),
            "SHUFFLE" => Some(Self::Shuffle),
            "REPEAT_ONE" => Some(Self::RepeatOne),
            "REPEAT_ALL" => Some(Self::RepeatAll),
            "RANDOM" => Some(Self::Random),
            "DIRECT_1" => Some(Self::Direct1),
            "INTRO" => Some(Self::Intro),
            _ => None,
        }
    }
}

/// Playback/record storage medium per UPnP-av-AVTransport-v3 spec §4.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StorageMedium {
    Unknown,
    Dv,
    MiniDv,
    Vhs,
    WVhs,
    SVhs,
    DVhs,
    VHSC,
    Video8,
    Hi8,
    CdRom,
    CdDa,
    CdR,
    CdRW,
    VideoCd,
    Sacd,
    MdAudio,
    MdPicture,
    DvdRom,
    DvdVideo,
    DvdPlusR,
    DvdMinusR,
    DvdPlusRW,
    DvdMinusRW,
    DvdRAM,
    DdAudio,
    Dat,
    Ld,
    Hdd,
    MicroMv,
    Network,
    None,
    NotImplemented,
    Sd,
    PcCard,
    MMC,
    CF,
    BD,
    MS,
    HDDVD,
}

impl StorageMedium {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unknown => "UNKNOWN",
            Self::Dv => "DV",
            Self::MiniDv => "MINI-DV",
            Self::Vhs => "VHS",
            Self::WVhs => "W-VHS",
            Self::SVhs => "S-VHS",
            Self::DVhs => "D-VHS",
            Self::VHSC => "VHSC",
            Self::Video8 => "VIDEO8",
            Self::Hi8 => "HI8",
            Self::CdRom => "CD-ROM",
            Self::CdDa => "CD-DA",
            Self::CdR => "CD-R",
            Self::CdRW => "CD-RW",
            Self::VideoCd => "VIDEO-CD",
            Self::Sacd => "SACD",
            Self::MdAudio => "MD-AUDIO",
            Self::MdPicture => "MD-PICTURE",
            Self::DvdRom => "DVD-ROM",
            Self::DvdVideo => "DVD-VIDEO",
            Self::DvdPlusR => "DVD+R",
            Self::DvdMinusR => "DVD-R",
            Self::DvdPlusRW => "DVD+RW",
            Self::DvdMinusRW => "DVD-RW",
            Self::DvdRAM => "DVD-RAM",
            Self::DdAudio => "DVD-AUDIO",
            Self::Dat => "DAT",
            Self::Ld => "LD",
            Self::Hdd => "HDD",
            Self::MicroMv => "MICRO-MV",
            Self::Network => "NETWORK",
            Self::None => "NONE",
            Self::NotImplemented => "NOT_IMPLEMENTED",
            Self::Sd => "SD",
            Self::PcCard => "PC-CARD",
            Self::MMC => "MMC",
            Self::CF => "CF",
            Self::BD => "BD",
            Self::MS => "MS",
            Self::HDDVD => "HD_DVD",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "UNKNOWN" => Some(Self::Unknown),
            "DV" => Some(Self::Dv),
            "MINI-DV" => Some(Self::MiniDv),
            "VHS" => Some(Self::Vhs),
            "W-VHS" => Some(Self::WVhs),
            "S-VHS" => Some(Self::SVhs),
            "D-VHS" => Some(Self::DVhs),
            "VHSC" => Some(Self::VHSC),
            "VIDEO8" => Some(Self::Video8),
            "HI8" => Some(Self::Hi8),
            "CD-ROM" => Some(Self::CdRom),
            "CD-DA" => Some(Self::CdDa),
            "CD-R" => Some(Self::CdR),
            "CD-RW" => Some(Self::CdRW),
            "VIDEO-CD" => Some(Self::VideoCd),
            "SACD" => Some(Self::Sacd),
            "MD-AUDIO" => Some(Self::MdAudio),
            "MD-PICTURE" => Some(Self::MdPicture),
            "DVD-ROM" => Some(Self::DvdRom),
            "DVD-VIDEO" => Some(Self::DvdVideo),
            "DVD+R" => Some(Self::DvdPlusR),
            "DVD-R" => Some(Self::DvdMinusR),
            "DVD+RW" => Some(Self::DvdPlusRW),
            "DVD-RW" => Some(Self::DvdMinusRW),
            "DVD-RAM" => Some(Self::DvdRAM),
            "DVD-AUDIO" => Some(Self::DdAudio),
            "DAT" => Some(Self::Dat),
            "LD" => Some(Self::Ld),
            "HDD" => Some(Self::Hdd),
            "MICRO-MV" => Some(Self::MicroMv),
            "NETWORK" => Some(Self::Network),
            "NONE" => Some(Self::None),
            "NOT_IMPLEMENTED" => Some(Self::NotImplemented),
            "SD" => Some(Self::Sd),
            "PC-CARD" => Some(Self::PcCard),
            "MMC" => Some(Self::MMC),
            "CF" => Some(Self::CF),
            "BD" => Some(Self::BD),
            "MS" => Some(Self::MS),
            "HD_DVD" => Some(Self::HDDVD),
            _ => None,
        }
    }
}

/// Record medium write status per UPnP-av-AVTransport-v3 spec §4.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecordMediumWriteStatus {
    Writable,
    Protected,
    NotWritable,
    Unknown,
    NotImplemented,
}

impl RecordMediumWriteStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Writable => "WRITABLE",
            Self::Protected => "PROTECTED",
            Self::NotWritable => "NOT_WRITABLE",
            Self::Unknown => "UNKNOWN",
            Self::NotImplemented => "NOT_IMPLEMENTED",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "WRITABLE" => Some(Self::Writable),
            "PROTECTED" => Some(Self::Protected),
            "NOT_WRITABLE" => Some(Self::NotWritable),
            "UNKNOWN" => Some(Self::Unknown),
            "NOT_IMPLEMENTED" => Some(Self::NotImplemented),
            _ => None,
        }
    }
}

/// Seek mode per UPnP-av-AVTransport-v3 spec §4.2.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeekMode {
    AbsoluteTime,
    RelativeTime,
    TrackNumber,
    TrackAbsoluteTime,
    TimeSeconds,
    ChannelFreq,
    TMSH,
    SAIA,
    SAIB,
    Index,
}

impl SeekMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AbsoluteTime => "ABS_TIME",
            Self::RelativeTime => "REL_TIME",
            Self::TrackNumber => "TRACK_NR",
            Self::TrackAbsoluteTime => "TRACK_TIME",
            Self::TimeSeconds => "TIME+SECS",
            Self::ChannelFreq => "CHANNEL-FREQ",
            Self::TMSH => "TMSH",
            Self::SAIA => "SAIA",
            Self::SAIB => "SAIB",
            Self::Index => "INDEX",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ABS_TIME" => Some(Self::AbsoluteTime),
            "REL_TIME" => Some(Self::RelativeTime),
            "TRACK_NR" => Some(Self::TrackNumber),
            "TRACK_TIME" => Some(Self::TrackAbsoluteTime),
            "TIME+SECS" => Some(Self::TimeSeconds),
            "CHANNEL-FREQ" => Some(Self::ChannelFreq),
            "TMSH" => Some(Self::TMSH),
            "SAIA" => Some(Self::SAIA),
            "SAIB" => Some(Self::SAIB),
            "INDEX" => Some(Self::Index),
            _ => None,
        }
    }
}

/// Media category per UPnP-av-AVTransport-v3 spec §4.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MediaCategory {
    NoMedia,
    TrackAware,
    TrackUnaware,
}

impl MediaCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NoMedia => "NO_MEDIA",
            Self::TrackAware => "TRACK_AWARE",
            Self::TrackUnaware => "TRACK_UNAWARE",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "NO_MEDIA" => Some(Self::NoMedia),
            "TRACK_AWARE" => Some(Self::TrackAware),
            "TRACK_UNAWARE" => Some(Self::TrackUnaware),
            _ => None,
        }
    }
}

// ===========================================================================
// UPnP AVTransport Service Actions
// ===========================================================================

/// AVTransport service actions (UPnP AV 2.0 spec §4.2.10)
///
/// 30 actions total: 18 Required (R), 6 Optional (O), 6 Conditionally Required (CR)

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionName {
    // Required actions
    SetAVTransportURI,
    GetMediaInfo,
    GetTransportInfo,
    GetPositionInfo,
    GetDeviceCapabilities,
    GetTransportSettings,
    Stop,
    Play,
    Seek,
    Next,
    Previous,
    // Optional actions
    SetNextAVTransportURI,
    GetMediaInfoExt,
    Pause,
    Record,
    SetPlayMode,
    SetRecordQualityMode,
    GetCurrentTransportActions,
    GetDRMState,
    GetStateVariables,
    SetStateVariables,
    GetSyncOffset,
    AdjustSyncOffset,
    SetSyncOffset,
    SyncPlay,
    SyncStop,
    SyncPause,
    SetStaticPlaylist,
    SetStreamingPlaylist,
    GetPlaylistInfo,
}

/// AVTransport argument names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArgumentName {
    InstanceID,
    CurrentURI,
    CurrentURIMetaData,
    Speed,
    Unit,
    Target,
    NrTracks,
    MediaDuration,
    NextURI,
    NextURIMetaData,
    PlayMedium,
    RecordMedium,
    WriteStatus,
    CurrentTransportState,
    CurrentTransportStatus,
    CurrentSpeed,
    Track,
    TrackDuration,
    TrackMetaData,
    TrackURI,
    RelTime,
    AbsTime,
    RelCount,
    AbsCount,
    PlayMedia,
    RecMedia,
    RecQualityModes,
    PlayMode,
    RecQualityMode,
    Actions,
    DRMState,
    StateVariableList,
    StateVariableValuePairs,
    SyncOffset,
    SyncOffsetAdj,
    SyncPoint,
    PlaylistData,
    PlaylistDataLength,
    PlaylistOffset,
    PlaylistTotalLength,
    PlaylistMIMEType,
    PlaylistExtendedType,
    PlaylistStep,
    PlaylistType,
    PlaylistInfo,
}

impl ActionName {
    /// Returns the UPnP action name string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SetAVTransportURI => "SetAVTransportURI",
            Self::SetNextAVTransportURI => "SetNextAVTransportURI",
            Self::GetMediaInfo => "GetMediaInfo",
            Self::GetMediaInfoExt => "GetMediaInfo_Ext",
            Self::GetTransportInfo => "GetTransportInfo",
            Self::GetPositionInfo => "GetPositionInfo",
            Self::GetDeviceCapabilities => "GetDeviceCapabilities",
            Self::GetTransportSettings => "GetTransportSettings",
            Self::Stop => "Stop",
            Self::Play => "Play",
            Self::Pause => "Pause",
            Self::Record => "Record",
            Self::Seek => "Seek",
            Self::Next => "Next",
            Self::Previous => "Previous",
            Self::SetPlayMode => "SetPlayMode",
            Self::SetRecordQualityMode => "SetRecordQualityMode",
            Self::GetCurrentTransportActions => "GetCurrentTransportActions",
            Self::GetDRMState => "GetDRMState",
            Self::GetStateVariables => "GetStateVariables",
            Self::SetStateVariables => "SetStateVariables",
            Self::GetSyncOffset => "GetSyncOffset",
            Self::AdjustSyncOffset => "AdjustSyncOffset",
            Self::SetSyncOffset => "SetSyncOffset",
            Self::SyncPlay => "SyncPlay",
            Self::SyncStop => "SyncStop",
            Self::SyncPause => "SyncPause",
            Self::SetStaticPlaylist => "SetStaticPlaylist",
            Self::SetStreamingPlaylist => "SetStreamingPlaylist",
            Self::GetPlaylistInfo => "GetPlaylistInfo",
        }
    }

    /// Returns true if this is a required action.
    pub fn is_required(&self) -> bool {
        matches!(
            self,
            Self::SetAVTransportURI
                | Self::GetMediaInfo
                | Self::GetTransportInfo
                | Self::GetPositionInfo
                | Self::GetDeviceCapabilities
                | Self::GetTransportSettings
                | Self::Stop
                | Self::Play
                | Self::Seek
                | Self::Next
                | Self::Previous
        )
    }

    /// Returns true if this is an optional action.
    pub fn is_optional(&self) -> bool {
        matches!(
            self,
            Self::SetNextAVTransportURI
                | Self::GetMediaInfoExt
                | Self::Pause
                | Self::Record
                | Self::SetPlayMode
                | Self::GetCurrentTransportActions
                | Self::SetRecordQualityMode
        )
    }

    /// Returns true if this is a conditionally required action.
    pub fn is_conditionally_required(&self) -> bool {
        matches!(
            self,
            Self::GetDRMState
                | Self::GetStateVariables
                | Self::SetStateVariables
                | Self::GetSyncOffset
                | Self::AdjustSyncOffset
                | Self::SetSyncOffset
                | Self::SyncPlay
                | Self::SyncStop
                | Self::SyncPause
                | Self::SetStaticPlaylist
                | Self::SetStreamingPlaylist
                | Self::GetPlaylistInfo
        )
    }
}

impl ArgumentName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InstanceID => "InstanceID",
            Self::CurrentURI => "CurrentURI",
            Self::CurrentURIMetaData => "CurrentURIMetaData",
            Self::Speed => "Speed",
            Self::Unit => "Unit",
            Self::Target => "Target",
            Self::NrTracks => "NrTracks",
            Self::MediaDuration => "MediaDuration",
            Self::NextURI => "NextURI",
            Self::NextURIMetaData => "NextURIMetaData",
            Self::PlayMedium => "PlayMedium",
            Self::RecordMedium => "RecordMedium",
            Self::WriteStatus => "WriteStatus",
            Self::CurrentTransportState => "CurrentTransportState",
            Self::CurrentTransportStatus => "CurrentTransportStatus",
            Self::CurrentSpeed => "CurrentSpeed",
            Self::Track => "Track",
            Self::TrackDuration => "TrackDuration",
            Self::TrackMetaData => "TrackMetaData",
            Self::TrackURI => "TrackURI",
            Self::RelTime => "RelTime",
            Self::AbsTime => "AbsTime",
            Self::RelCount => "RelCount",
            Self::AbsCount => "AbsCount",
            Self::PlayMedia => "PlayMedia",
            Self::RecMedia => "RecMedia",
            Self::RecQualityModes => "RecQualityModes",
            Self::PlayMode => "PlayMode",
            Self::RecQualityMode => "RecQualityMode",
            Self::Actions => "Actions",
            Self::DRMState => "DRMState",
            Self::StateVariableList => "StateVariableList",
            Self::StateVariableValuePairs => "StateVariableValuePairs",
            Self::SyncOffset => "SyncOffset",
            Self::SyncOffsetAdj => "SyncOffsetAdj",
            Self::SyncPoint => "SyncPoint",
            Self::PlaylistData => "PlaylistData",
            Self::PlaylistDataLength => "PlaylistDataLength",
            Self::PlaylistOffset => "PlaylistOffset",
            Self::PlaylistTotalLength => "PlaylistTotalLength",
            Self::PlaylistMIMEType => "PlaylistMIMEType",
            Self::PlaylistExtendedType => "PlaylistExtendedType",
            Self::PlaylistStep => "PlaylistStep",
            Self::PlaylistType => "PlaylistType",
            Self::PlaylistInfo => "PlaylistInfo",
        }
    }
}

/// UPnP AVTransport service error codes.
///
/// Per UPnP-av-AVTransport-v3-Service specification.
/// Error codes 400-699 are defined by the UPnP Device Architecture.
/// Error codes 700-799 are service-specific.
/// Error codes 800-899 are reserved for vendor extensions (not permitted for standard actions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Error {
    /// The immediate transition from current transport state to desired transport state is not supported.
    /// Used by: Stop(), Play(), Pause(), Record(), Seek(), Next(), Previous(), SetPlayMode(), SyncPlay(), SyncStop(), SyncPause()
    TransitionNotAvailable = 701,
    /// The media does not contain any contents that can be played.
    /// Used by: Play(), SyncPlay()
    NoContents = 702,
    /// The media cannot be read (e.g., because of dust or a scratch).
    /// Used by: Play(), SyncPlay()
    ReadError = 703,
    /// The storage format of the currently loaded media is not supported for playback by this device.
    /// Used by: Play(), SyncPlay()
    FormatNotSupportedForPlayback = 704,
    /// The transport is hold locked (e.g., hold lock switch ON).
    /// Used by: Stop(), Play(), Pause(), Record(), Seek(), Next(), Previous(), SetPlayMode(), SyncPlay(), SyncStop(), SyncPause()
    TransportLocked = 705,
    /// The media cannot be written (e.g., because of dust or a scratch).
    /// Used by: Record()
    WriteError = 706,
    /// The media is write-protected or is of a not writable type.
    /// Used by: Record()
    MediaProtectedOrNotWritable = 707,
    /// The storage format of the currently loaded media is not supported for recording.
    /// Used by: Record()
    FormatNotSupportedForRecording = 708,
    /// There is no free space left on the loaded media.
    /// Used by: Record()
    MediaFull = 709,
    /// The specified seek mode is not supported by the device.
    /// Used by: Seek(), SyncPlay()
    SeekModeNotSupported = 710,
    /// The specified seek target is not present on the media or is not specified in terms of the seek mode.
    /// Used by: Seek(), Next(), Previous(), SyncPlay()
    IllegalSeekTarget = 711,
    /// The specified play mode is not supported by the device.
    /// Used by: SetPlayMode()
    PlayModeNotSupported = 712,
    /// The specified record quality is not supported by the device.
    /// Used by: SetRecordQualityMode()
    RecordQualityNotSupported = 713,
    /// The specified resource has a MIME-type which is not supported by the AVTransport service.
    /// Used by: SetAVTransportURI(), SetNextAVTransportURI(), Play(), SetStaticPlaylist(), SetStreamingPlaylist(), GetPlaylistInfo()
    IllegalMime = 714,
    /// The resource is already in use at this time.
    /// Used by: SetAVTransportURI(), SetNextAVTransportURI(), Play()
    ContentBusy = 715,
    /// The specified resource cannot be found in the network.
    /// Used by: SetAVTransportURI(), SetNextAVTransportURI(), Play()
    ResourceNotFound = 716,
    /// The specified playback speed is not supported by the AVTransport service.
    /// Used by: Play(), SyncPlay()
    PlaySpeedNotSupported = 717,
    /// The specified InstanceID is invalid for this AVTransport.
    /// Used by: All AVTransport actions
    InvalidInstanceID = 718,
    /// An unspecified DRM error occurred.
    /// Used by: SetAVTransportURI(), SetNextAVTransportURI(), Play(), Record(), Seek(), Next(), Previous(), SyncPlay()
    DrmError = 719,
    /// The content use validity interval has expired.
    /// Used by: SetAVTransportURI(), SetNextAVTransportURI(), Play(), Record(), Seek(), Next(), Previous(), SyncPlay()
    ExpiredContent = 720,
    /// The requested content use is disallowed.
    /// Used by: SetAVTransportURI(), SetNextAVTransportURI(), Play(), Record(), Seek(), Next(), Previous(), SyncPlay()
    NonAllowedUse = 721,
    /// The allowed content uses cannot be verified.
    /// Used by: SetAVTransportURI(), SetNextAVTransportURI(), Play(), Record(), Seek(), Next(), Previous(), SyncPlay()
    CantDetermineAllowedUses = 722,
    /// The number of times this content has been used has reached the maximum allowed.
    /// Used by: SetAVTransportURI(), SetNextAVTransportURI(), Play(), Record(), Seek(), Next(), Previous(), SyncPlay()
    ExhaustedAllowedUse = 723,
    /// Device authentication failure between media source and sink devices.
    /// Used by: SetAVTransportURI(), SetNextAVTransportURI(), Play(), Record(), Seek(), Next(), Previous(), SyncPlay()
    DeviceAuthenticationFailure = 724,
    /// Either the media source or sink device has been revoked.
    /// Used by: SetAVTransportURI(), SetNextAVTransportURI(), Play(), Record(), Seek(), Next(), Previous(), SyncPlay()
    DeviceRevocation = 725,
    /// Some of the variables in the StateVariableList are invalid.
    /// Used by: GetStateVariables()
    InvalidStateVariableList = 726,
    /// The CSV list is not well formed.
    /// Used by: GetStateVariables()
    IllFormedCsvList = 727,
    /// One of the StateVariableValuePairs contains an invalid value.
    /// Used by: SetStateVariables()
    InvalidStateVariableValue = 728,
    /// The specified ServiceType is invalid.
    /// Used by: SetStateVariables()
    InvalidServiceType = 729,
    /// The specified ServiceId is invalid.
    /// Used by: SetStateVariables()
    InvalidServiceId = 730,
    /// The supplied time, offset, or position value for an argument was not valid.
    /// Used by: SetSyncOffset(), AdjustSyncOffset(), SyncPlay(), SyncStop(), SyncPause()
    InvalidTimeOffsetOrPosition = 731,
    /// The system was not able to calculate a synchronization point using the supplied time, offset, or position information.
    /// Used by: SyncPlay()
    UnableToCalculateSyncPoint = 732,
    /// The specified or calculated synchronization point, time, or position occurred too quickly (or in the past) for the device to complete the action.
    /// Used by: SyncStop(), SyncPause()
    SyncPositionOrOffsetTooEarly = 733,
    /// The PlaylistOffset specified would result in a missing section of a playlist.
    /// Used by: SetStaticPlaylist()
    IllegalPlaylistOffset = 734,
    /// Playlist length is incorrect or exceeds storage capacity of device.
    /// Used by: SetStaticPlaylist(), SetStreamingPlaylist()
    IncorrectPlaylistLength = 735,
    /// The playlist delivered failed syntactic or semantic checks.
    /// Used by: SetStaticPlaylist(), SetStreamingPlaylist()
    IllegalPlaylist = 736,
    /// The DNS Server is not available (HTTP error 503).
    /// Used by: SetAVTransportURI(), SetNextAVTransportURI()
    NoDnsServer = 737,
    /// Unable to resolve the Fully Qualified Domain Name (HTTP error 502).
    /// Used by: SetAVTransportURI(), SetNextAVTransportURI()
    BadDomainName = 738,
    /// The server that hosts the resource is unreachable or unresponsive (HTTP error 404/410).
    /// Used by: SetAVTransportURI(), SetNextAVTransportURI()
    ServerError = 739,
}

impl Error {
    /// Returns the numeric error code.
    pub const fn code(&self) -> u16 {
        *self as u16
    }

    /// Returns the human-readable error description.
    pub const fn description(&self) -> &'static str {
        match self {
            Self::TransitionNotAvailable => {
                "The immediate transition from current transport state to desired transport state is not supported by this device"
            }
            Self::NoContents => "The media does not contain any contents that can be played",
            Self::ReadError => {
                "The media cannot be read (for example, because of dust or a scratch)"
            }
            Self::FormatNotSupportedForPlayback => {
                "The storage format of the currently loaded media is not supported for playback by this device"
            }
            Self::TransportLocked => {
                "The transport is hold locked (for example, by a mechanical hold lock switch)"
            }
            Self::WriteError => {
                "The media cannot be written (for example, because of dust or a scratch)"
            }
            Self::MediaProtectedOrNotWritable => {
                "The media is write-protected or is of a not writable type"
            }
            Self::FormatNotSupportedForRecording => {
                "The storage format of the currently loaded media is not supported for recording by this device"
            }
            Self::MediaFull => "There is no free space left on the loaded media",
            Self::SeekModeNotSupported => "The specified seek mode is not supported by the device",
            Self::IllegalSeekTarget => {
                "The specified seek target is not present on the media or is not specified in terms of the seek mode"
            }
            Self::PlayModeNotSupported => "The specified play mode is not supported by the device",
            Self::RecordQualityNotSupported => {
                "The specified record quality is not supported by the device"
            }
            Self::IllegalMime => {
                "The specified resource has a MIME-type which is not supported by the AVTransport service"
            }
            Self::ContentBusy => "This indicates that the resource is already in use at this time",
            Self::ResourceNotFound => "The specified resource cannot be found in the network",
            Self::PlaySpeedNotSupported => {
                "The specified playback speed is not supported by the AVTransport service"
            }
            Self::InvalidInstanceID => "The specified InstanceID is invalid for this AVTransport",
            Self::DrmError => "The action failed because an unspecified DRM error occurred",
            Self::ExpiredContent => {
                "The action failed because the content use validity interval has expired"
            }
            Self::NonAllowedUse => {
                "The action failed because the requested content use is disallowed"
            }
            Self::CantDetermineAllowedUses => {
                "The action failed because the allowed content uses cannot be verified"
            }
            Self::ExhaustedAllowedUse => {
                "The action failed because the number of times this content has been used as requested has reached the maximum allowed number of uses"
            }
            Self::DeviceAuthenticationFailure => {
                "The action failed because of a device authentication failure between the media source device and the media sink device"
            }
            Self::DeviceRevocation => {
                "The action failed because either the media source device or the media sink device has been revoked"
            }
            Self::InvalidStateVariableList => "Some of the variables are invalid",
            Self::IllFormedCsvList => "The CSV list is not well formed",
            Self::InvalidStateVariableValue => {
                "One of the StateVariableValuePairs contains an invalid value"
            }
            Self::InvalidServiceType => "The specified ServiceType is invalid",
            Self::InvalidServiceId => "The specified ServiceId is invalid",
            Self::InvalidTimeOffsetOrPosition => {
                "The action failed because the supplied time, offset, or position value for an argument was not valid"
            }
            Self::UnableToCalculateSyncPoint => {
                "The action failed because the system was not able to calculate a synchronization point using the supplied time, offset, or position information"
            }
            Self::SyncPositionOrOffsetTooEarly => {
                "The action failed because the specified or calculated synchronization point, time, or position occurred too quickly (or in the past) for the device to complete the action"
            }
            Self::IllegalPlaylistOffset => {
                "The PlaylistOffset specified would result in a missing section of a playlist"
            }
            Self::IncorrectPlaylistLength => {
                "Playlist length is incorrect or exceeds storage capacity of device"
            }
            Self::IllegalPlaylist => "The playlist delivered failed syntactic or semantic checks",
            Self::NoDnsServer => "The DNS Server is not available (HTTP error 503)",
            Self::BadDomainName => {
                "Unable to resolve the Fully Qualified Domain Name (HTTP error 502)"
            }
            Self::ServerError => {
                "The server that hosts the resource is unreachable or unresponsive (HTTP error 404/410)"
            }
        }
    }

    /// Lookup an error by its numeric code.
    pub const fn from_code(code: u16) -> Option<Self> {
        match code {
            701 => Some(Self::TransitionNotAvailable),
            702 => Some(Self::NoContents),
            703 => Some(Self::ReadError),
            704 => Some(Self::FormatNotSupportedForPlayback),
            705 => Some(Self::TransportLocked),
            706 => Some(Self::WriteError),
            707 => Some(Self::MediaProtectedOrNotWritable),
            708 => Some(Self::FormatNotSupportedForRecording),
            709 => Some(Self::MediaFull),
            710 => Some(Self::SeekModeNotSupported),
            711 => Some(Self::IllegalSeekTarget),
            712 => Some(Self::PlayModeNotSupported),
            713 => Some(Self::RecordQualityNotSupported),
            714 => Some(Self::IllegalMime),
            715 => Some(Self::ContentBusy),
            716 => Some(Self::ResourceNotFound),
            717 => Some(Self::PlaySpeedNotSupported),
            718 => Some(Self::InvalidInstanceID),
            719 => Some(Self::DrmError),
            720 => Some(Self::ExpiredContent),
            721 => Some(Self::NonAllowedUse),
            722 => Some(Self::CantDetermineAllowedUses),
            723 => Some(Self::ExhaustedAllowedUse),
            724 => Some(Self::DeviceAuthenticationFailure),
            725 => Some(Self::DeviceRevocation),
            726 => Some(Self::InvalidStateVariableList),
            727 => Some(Self::IllFormedCsvList),
            728 => Some(Self::InvalidStateVariableValue),
            729 => Some(Self::InvalidServiceType),
            730 => Some(Self::InvalidServiceId),
            731 => Some(Self::InvalidTimeOffsetOrPosition),
            732 => Some(Self::UnableToCalculateSyncPoint),
            733 => Some(Self::SyncPositionOrOffsetTooEarly),
            734 => Some(Self::IllegalPlaylistOffset),
            735 => Some(Self::IncorrectPlaylistLength),
            736 => Some(Self::IllegalPlaylist),
            737 => Some(Self::NoDnsServer),
            738 => Some(Self::BadDomainName),
            739 => Some(Self::ServerError),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AVTransport Error {}: {}",
            self.code(),
            self.description()
        )
    }
}

impl std::error::Error for Error {}

// ===========================================================================
// UPnP AVTransport State Variable Names
// ===========================================================================

/// AVTransport state variable names per UPnP-av-AVTransport-v3 spec §4.1.
///
/// 30 state variables total:
/// - 3 core transport state (TransportState, TransportStatus, CurrentMediaCategory)
/// - 6 media identity (AVTransportURI, AVTransportURIMetaData, NextAVTransportURI,
///   NextAVTransportURIMetaData, CurrentTrackURI, CurrentTrackMetaData)
/// - 8 playback position (CurrentTrack, NumberOfTracks, CurrentTrackDuration,
///   CurrentMediaDuration, RelativeTimePosition, AbsoluteTimePosition,
///   RelativeCounterPosition, AbsoluteCounterPosition)
/// - 4 storage & capabilities (PlaybackStorageMedium, RecordStorageMedium,
///   PossiblePlaybackStorageMedia, PossibleRecordStorageMedia)
/// - 2 play control (CurrentPlayMode, TransportPlaySpeed)
/// - 4 recording (RecordMediumWriteStatus, CurrentRecordQualityMode,
///   PossibleRecordQualityModes, DRMState)
/// - 2 eventing (LastChange, CurrentTransportActions)
/// - 10 A_ARG_TYPE variables for action argument type definitions
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StateVariableName {
    // Core transport state
    #[default]
    TransportState,
    TransportStatus,
    CurrentMediaCategory,
    // Media identity
    AVTransportURI,
    AVTransportURIMetaData,
    NextAVTransportURI,
    NextAVTransportURIMetaData,
    CurrentTrackURI,
    CurrentTrackMetaData,
    // Playback position
    CurrentTrack,
    NumberOfTracks,
    CurrentTrackDuration,
    CurrentMediaDuration,
    RelativeTimePosition,
    AbsoluteTimePosition,
    RelativeCounterPosition,
    AbsoluteCounterPosition,
    // Storage & capabilities
    PlaybackStorageMedium,
    RecordStorageMedium,
    PossiblePlaybackStorageMedia,
    PossibleRecordStorageMedia,
    // Play control
    CurrentPlayMode,
    TransportPlaySpeed,
    // Recording
    RecordMediumWriteStatus,
    CurrentRecordQualityMode,
    PossibleRecordQualityModes,
    DRMState,
    // Eventing
    LastChange,
    CurrentTransportActions,
    // A_ARG_TYPE variables — type definitions for action arguments
    A_ARG_TYPE_InstanceID,
    A_ARG_TYPE_SeekMode,
    A_ARG_TYPE_SeekTarget,
    A_ARG_TYPE_RecordMedium,
    A_ARG_TYPE_StreamFormat,
}

impl std::fmt::Display for StateVariableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TransportState => write!(f, "TransportState"),
            Self::TransportStatus => write!(f, "TransportStatus"),
            Self::CurrentMediaCategory => write!(f, "CurrentMediaCategory"),
            Self::AVTransportURI => write!(f, "AVTransportURI"),
            Self::AVTransportURIMetaData => write!(f, "AVTransportURIMetaData"),
            Self::NextAVTransportURI => write!(f, "NextAVTransportURI"),
            Self::NextAVTransportURIMetaData => write!(f, "NextAVTransportURIMetaData"),
            Self::CurrentTrackURI => write!(f, "CurrentTrackURI"),
            Self::CurrentTrackMetaData => write!(f, "CurrentTrackMetaData"),
            Self::CurrentTrack => write!(f, "CurrentTrack"),
            Self::NumberOfTracks => write!(f, "NumberOfTracks"),
            Self::CurrentTrackDuration => write!(f, "CurrentTrackDuration"),
            Self::CurrentMediaDuration => write!(f, "CurrentMediaDuration"),
            Self::RelativeTimePosition => write!(f, "RelativeTimePosition"),
            Self::AbsoluteTimePosition => write!(f, "AbsoluteTimePosition"),
            Self::RelativeCounterPosition => write!(f, "RelativeCounterPosition"),
            Self::AbsoluteCounterPosition => write!(f, "AbsoluteCounterPosition"),
            Self::PlaybackStorageMedium => write!(f, "PlaybackStorageMedium"),
            Self::RecordStorageMedium => write!(f, "RecordStorageMedium"),
            Self::PossiblePlaybackStorageMedia => write!(f, "PossiblePlaybackStorageMedia"),
            Self::PossibleRecordStorageMedia => write!(f, "PossibleRecordStorageMedia"),
            Self::CurrentPlayMode => write!(f, "CurrentPlayMode"),
            Self::TransportPlaySpeed => write!(f, "TransportPlaySpeed"),
            Self::RecordMediumWriteStatus => write!(f, "RecordMediumWriteStatus"),
            Self::CurrentRecordQualityMode => write!(f, "CurrentRecordQualityMode"),
            Self::PossibleRecordQualityModes => write!(f, "PossibleRecordQualityModes"),
            Self::DRMState => write!(f, "DRMState"),
            Self::LastChange => write!(f, "LastChange"),
            Self::CurrentTransportActions => write!(f, "CurrentTransportActions"),
            Self::A_ARG_TYPE_InstanceID => write!(f, "A_ARG_TYPE_InstanceID"),
            Self::A_ARG_TYPE_SeekMode => write!(f, "A_ARG_TYPE_SeekMode"),
            Self::A_ARG_TYPE_SeekTarget => write!(f, "A_ARG_TYPE_SeekTarget"),
            Self::A_ARG_TYPE_RecordMedium => write!(f, "A_ARG_TYPE_RecordMedium"),
            Self::A_ARG_TYPE_StreamFormat => write!(f, "A_ARG_TYPE_StreamFormat"),
        }
    }
}

impl crate::state::StateVariableName for StateVariableName {
    fn as_str(&self) -> &'static str {
        match self {
            Self::TransportState => "TransportState",
            Self::TransportStatus => "TransportStatus",
            Self::CurrentMediaCategory => "CurrentMediaCategory",
            Self::AVTransportURI => "AVTransportURI",
            Self::AVTransportURIMetaData => "AVTransportURIMetaData",
            Self::NextAVTransportURI => "NextAVTransportURI",
            Self::NextAVTransportURIMetaData => "NextAVTransportURIMetaData",
            Self::CurrentTrackURI => "CurrentTrackURI",
            Self::CurrentTrackMetaData => "CurrentTrackMetaData",
            Self::CurrentTrack => "CurrentTrack",
            Self::NumberOfTracks => "NumberOfTracks",
            Self::CurrentTrackDuration => "CurrentTrackDuration",
            Self::CurrentMediaDuration => "CurrentMediaDuration",
            Self::RelativeTimePosition => "RelativeTimePosition",
            Self::AbsoluteTimePosition => "AbsoluteTimePosition",
            Self::RelativeCounterPosition => "RelativeCounterPosition",
            Self::AbsoluteCounterPosition => "AbsoluteCounterPosition",
            Self::PlaybackStorageMedium => "PlaybackStorageMedium",
            Self::RecordStorageMedium => "RecordStorageMedium",
            Self::PossiblePlaybackStorageMedia => "PossiblePlaybackStorageMedia",
            Self::PossibleRecordStorageMedia => "PossibleRecordStorageMedia",
            Self::CurrentPlayMode => "CurrentPlayMode",
            Self::TransportPlaySpeed => "TransportPlaySpeed",
            Self::RecordMediumWriteStatus => "RecordMediumWriteStatus",
            Self::CurrentRecordQualityMode => "CurrentRecordQualityMode",
            Self::PossibleRecordQualityModes => "PossibleRecordQualityModes",
            Self::DRMState => "DRMState",
            Self::LastChange => "LastChange",
            Self::CurrentTransportActions => "CurrentTransportActions",
            Self::A_ARG_TYPE_InstanceID => "A_ARG_TYPE_InstanceID",
            Self::A_ARG_TYPE_SeekMode => "A_ARG_TYPE_SeekMode",
            Self::A_ARG_TYPE_SeekTarget => "A_ARG_TYPE_SeekTarget",
            Self::A_ARG_TYPE_RecordMedium => "A_ARG_TYPE_RecordMedium",
            Self::A_ARG_TYPE_StreamFormat => "A_ARG_TYPE_StreamFormat",
        }
    }

    fn is_evented(&self) -> bool {
        matches!(self, Self::LastChange)
    }

    fn is_instance_scoped(&self) -> bool {
        // Per-InstanceID variables (InstanceID > 0)
        matches!(
            self,
            Self::AVTransportURI
                | Self::AVTransportURIMetaData
                | Self::NextAVTransportURI
                | Self::NextAVTransportURIMetaData
                | Self::CurrentTrackURI
                | Self::CurrentTrackMetaData
                | Self::CurrentTrack
                | Self::NumberOfTracks
                | Self::CurrentTrackDuration
                | Self::CurrentMediaDuration
                | Self::RelativeTimePosition
                | Self::AbsoluteTimePosition
                | Self::RelativeCounterPosition
                | Self::AbsoluteCounterPosition
                | Self::PlaybackStorageMedium
                | Self::RecordStorageMedium
                | Self::PossiblePlaybackStorageMedia
                | Self::PossibleRecordStorageMedia
                | Self::CurrentPlayMode
                | Self::TransportPlaySpeed
                | Self::RecordMediumWriteStatus
                | Self::CurrentRecordQualityMode
                | Self::PossibleRecordQualityModes
                | Self::DRMState
        )
    }

    fn data_type(&self) -> crate::types::upnp::DataType {
        match self {
            Self::TransportState => crate::types::upnp::DataType::String,
            Self::TransportStatus => crate::types::upnp::DataType::String,
            Self::CurrentMediaCategory => crate::types::upnp::DataType::String,
            Self::AVTransportURI => crate::types::upnp::DataType::Uri,
            Self::AVTransportURIMetaData => crate::types::upnp::DataType::String,
            Self::NextAVTransportURI => crate::types::upnp::DataType::Uri,
            Self::NextAVTransportURIMetaData => crate::types::upnp::DataType::String,
            Self::CurrentTrackURI => crate::types::upnp::DataType::Uri,
            Self::CurrentTrackMetaData => crate::types::upnp::DataType::String,
            Self::CurrentTrack => crate::types::upnp::DataType::UnsignedInt,
            Self::NumberOfTracks => crate::types::upnp::DataType::UnsignedInt,
            Self::CurrentTrackDuration => crate::types::upnp::DataType::String,
            Self::CurrentMediaDuration => crate::types::upnp::DataType::String,
            Self::RelativeTimePosition => crate::types::upnp::DataType::String,
            Self::AbsoluteTimePosition => crate::types::upnp::DataType::String,
            Self::RelativeCounterPosition => crate::types::upnp::DataType::Int,
            Self::AbsoluteCounterPosition => crate::types::upnp::DataType::UnsignedInt,
            Self::PlaybackStorageMedium => crate::types::upnp::DataType::String,
            Self::RecordStorageMedium => crate::types::upnp::DataType::String,
            Self::PossiblePlaybackStorageMedia => crate::types::upnp::DataType::String,
            Self::PossibleRecordStorageMedia => crate::types::upnp::DataType::String,
            Self::CurrentPlayMode => crate::types::upnp::DataType::String,
            Self::TransportPlaySpeed => crate::types::upnp::DataType::String,
            Self::RecordMediumWriteStatus => crate::types::upnp::DataType::String,
            Self::CurrentRecordQualityMode => crate::types::upnp::DataType::String,
            Self::PossibleRecordQualityModes => crate::types::upnp::DataType::String,
            Self::DRMState => crate::types::upnp::DataType::String,
            Self::LastChange => crate::types::upnp::DataType::String,
            Self::CurrentTransportActions => crate::types::upnp::DataType::String,
            Self::A_ARG_TYPE_InstanceID => crate::types::upnp::DataType::UnsignedInt,
            Self::A_ARG_TYPE_SeekMode => crate::types::upnp::DataType::String,
            Self::A_ARG_TYPE_SeekTarget => crate::types::upnp::DataType::String,
            Self::A_ARG_TYPE_RecordMedium => crate::types::upnp::DataType::String,
            Self::A_ARG_TYPE_StreamFormat => crate::types::upnp::DataType::String,
        }
    }
}
