
mod alias {
    pub enum Commands {
        NewBeacon = 15,
        DelBeacon = 17,
        RegisterAction = 60,
        Action = 62,
        ActionTxStatus = 64,
    }
}

// Commands
// Do not change order; acts as ABI
pub enum Commands {
    Unspec,
    GetWiPhy,
    SetWiPhy,
    NewWiPhy,
    DelWiPhy,
    
    GetInterface,
    SetInterface,
    NewInterface,
    DelIntreface,

    GetKey,
    SetKey,
    NewKey,
    DelKey,

    GetBeacon,
    SetBeacon,
    StartAp,
    AliasNewBeacon, // StartAp alias
    StopAp,
    AliasDelBeacon, // StopAp alias

    GetStation,
    SetStation,
    NewStation,
    DelStation,

    GetMPath,
    SetMPath,
    NewMPath,
    DelMPath,

    SetBss,

    SetReg,
    ReqSetReg,

    GetMeshConfig,
    SetMeshConfig,

    SetMgmtExtraIe, // Reserved; not used

    GetReg,

    GetScan,
    TriggerScan,
    NewScanResults,
    ScanAborted,

    RegChange,

    Authenticate,
    Associate,
    Deauthenticate,
    Disassociate,

    MichaelMicFailure,

    RegBeaconHint,

    JoinIbss,
    LeaveIbss,

    TestMode,
    Connect,
    Roam,
    Disconnect,

    SetWiPhyNetNs,

    GetSurvey,
    NewSurveyResults,

    SetPmksa,
    DelPmksa,
    FlushPmksa,

    RemainOnChannel,
    CancelRemainOnChannel,
    
    SetTxBitrateMask,

    RegisterFrame,
    AliasRegisterAction, // RegisterFrame alias
    Frame,
    AliasAction,         // Frame alias
    FrameTxStatus,
    AliasActionTxStatus, // FrameTxStatus

    SetPowerSave,
    GetPowerSave,

    SetCqm,
    NotifyCqm,

    SetChannel,
    SetWdsPeer,

    FrameWaitCancel,

    JoinMesh,
    LeaveMesh,

    UnprotDeathenticate,
    UnProtDisassociate,

    NewPeerCandidate,

    GetWowLan,
    SetWowLan,

    StartSchedScan,
    StopSchedScan,
    SchedScanResults,
    SchedScanStopped,

    SetRekeyOffload,

    PmksaCandidate,

    TdlsOper,
    TdlsMgmt,

    UnexpectedFrame,
    
    ProbeClient,

    RegisterBeacons,

    Unexpected4AddrFrame,

    SetNoAckMap,

    ChSwitchNotify,

    StartP2pDevice,
    StopP2pDevice,

    ConnFailed,

    SetMcastRate,

    SetMacAcl,

    RadarDetect,

    GetProtocolFeatures,

    UpdateFtIes,
    FtEvent,

    CritProtocolState,
    CritProtocolStop,

    GetCoalesce,
    SetCoalesce,

    Vendor,

    SetQosMap,

    AddTxTs,
    DelTxTs,

    GetMpp,

    AfterLast
}


// attributes
// Do not change order; acts as ABI
pub enum Attrs {
    Unspec,

    WiPhy,
    WiPhyName,

    IfIndex,
    IfName,
    IfType,

    Mac,

    KeyData,
    KeyIdx,
    KeyCipher,
    KeySeq,
    KeyDefault,

    BeaconInterval,
    DtimPeriod,
    BeaconHead,
    BeaconTail,

    StaAid,
    StaFlags,
    StaListenInterval,
    StaSupportedRates,
    StaVlan,
    StaInfo,

    WiPhyBands,
    
    MntrFlags,

    MeshId,
    StaPlinkAction,
    MpathNextHop,
    MpathInfo,

    BssCtsProt,
    BssShortPreamble,
    BssShortSlotTime,

    HtCapability,

    SupportedIfTypes,

    RegAlpha2,
    RegRules,

    MeshConfig,

    BssBasicRates,

    WiPhyTxqParams,
    WiPhyFreq,
    WiPhyChannelType,

    KeyDefaultMgmt,

    MgmtSubtype,
    Ie,

    MaxNumScanSsids,

    ScanFrequencies,
    ScanSsids,
    Generation,
    Bss,

    RegInitiator,
    RegType,

    SupportedCommands,

    Frame,
    Ssid,
    AuthType,
    ReasonCode,

    KeyType,

    MaxScanIeLen,
    CipherSuites,

    FreqBefore,
    FreqAfter,

    FreqFixed,

    WiPhyRetryShort,
    WiPhyRetryLong,
    WiPhyFragThreshold,
    WiPhyRtsThreshold,

    TimedOut,

    UseMfp,

    StaFlags2,

    ControlPort,

    TestData,

    Privacy,

    DisconnectedByAp,

    StatusCode,

    CipherSuitesPairwise,
    CipherSuiteGroup,
    WpaVersions,
    AkmSuites,

    ReqIe,
    RespIe,

    PrevBssid,

    Key,
    Keys,

    Pid,

    Addr4,

    SurveyInfo,

    PmkId,
    MaxNumPmkIds,

    Duration,

    Cookie,

    WiPhyCoverageClass,

    TxRates,

    FrameMatch,

    Ack,

    PsState,
    
    Cqm,

    LocalStateChange,

    ApIsolate,

    WiPhyTxPowerSetting,
    WiPhyTxPowerLevel,

    TxFrameTypes,
    RxFrameTypes,
    FrameType,

    ControlPortEthertype,
    ControlPortNoEncrypt,

    SupportIbssRsn,

    WiPhyAntennaTx,
    WiPhyAntennaRx,

    McastRate,

    OffChannelTxOk,

    BssHtOpmode,

    KeyDefaultTypes,

    MaxRemainOnChannelDuration,

    MeshSetup,

    WiPhyAntennaAvailTx,
    WiPhyAntennaAvailRx,

    SupportMeshAuth,
    StaPlinkState,

    WowLanTriggers,
    WowLanTriggersSupported,

    SchedScanInterval,

    InterfaceCombinations,
    SoftwareIfTypes,

    RekeyData,

    MaxNumSchedScanSsids,
    MaxSchedScanIeLen,

    ScanSuppRates,

    HiddenSsid,

    IeProbeResp,
    IeAssocResp,

    StaWme,
    SupportApUapsd,

    RoamSupport,

    AttrSchedScanMatch,
    MaxMatchSets,

    PmksaCandidate,

    TxNoCckRate,

    TdlsAction,
    TdlsDialogToken,
    TdlsOperation,
    TdlsSupport,
    TdlsExternalSetup,

    DeviceApSme,

    DontWaitForAck,

    FeatureFlags,

    ProbeRespOffload,

    ProbeResp,

    DfsRegion,

    DisableHt,
    HtCapabilityMask,

    NoackMap,

    InactivityTimeout,

    RxSignalDbm,

    BgScanPeriod,

    Wdev,

    UserRegHintType,

    ConnFailedReason,

    SaeData,

    VhtCapability,

    ScanFlags,

    ChannelWidth,
    CenterFreq1,
    CenterFreq2,

    P2pCtwindow,
    P2pOppps,

    LocalMeshPowerMode,

    AclPolicy,

    MacAddrs,

    MacAclMax,

    RadarEvent,

    ExtCapa,

    ExtCapaMask,

    StaCapability,
    StaExtCapability,

    ProtocolFeatures,
    SplitWiphyDump,

    DisableVht,
    VhtCapabilityMask,

    Mdid,

    IeRic,

    CritProtId,
    MaxCritProtDuration,

    PeerAid,

    CoalesceRule,

    ChSwitchCount,
    ChSwitchBlockTx,
    CsaIes,
    CsaCOffBeacon,
    CsaCOffPresp,

    RxmgmtFlags,

    StaSupportedChannels,
    StaSipportedOperClasses,

    HandleDfs,

    Support5Mhz,
    Support10Mhz,

    OpmodeNotif,

    VendorId,
    VendorSubcmd,
    VendorData,
    VendorEvents,

    QodMap,

    MacHint,
    WiPhyFreqHint,

    MaxApAssocSta,

    TdlsPeerCapability,

    IfaceSocketOwnder,
    CsaCOffsetsTx,
    MaxCsaCounters,

    TdlsInitiator,

    UseRrm,

    WiPhyDynAck,

    Tsid,
    UserPrio,
    AdmittedTime,

    SmpsMode,

    AfterLast
}

// Interface types
pub enum IfType {
    Unspecified,
    AdHoc,
    Station,
    Ap,
    ApVlan,
    Wds,
    Monitor,
    MeshPoint,
    P2pClient,
    P2pGo,
    P2pDevice,

    NumIfTypes
}

// Station flags
pub enum StaFlags {
    __Invalid,
    Authorized,
    ShortPreamble,
    Wme,
    Mfp,
    Authenticated,
    TdlsPeer,
    Associated,
    AfterLast
}

#[repr(C, packed)]
pub struct StaFlagUpdate {
    mask: u32,
    set: u32,
}

// attrubtes for getting station bitrate information
pub enum RateInfo {
    __Invalid,
    Bitrate,
    Mcs,
    Width40Mhz,
    ShortGi,
    Bitrate32,
    VhtMcs,
    VhtNss,
    Width80Mhz,
    Width80P80Mhz,
    Width160Mhz,

    AfterLast
}

pub enum StaBssParam {
    __Invalid,
    CtsProt,
    ShortPreamble,
    ShortSlotTime,
    DtimPeriod,
    BeaconInterval,

    AfterLast
}

pub enum StaInfo {
    __Invalid,
    InactiveTime,
    RxBytes,
    TxBytes,
    Llid,
    Plid,
    PlinkState,
    Signal,
    TxBitrate,
    RxPackets,
    TxPackets,
    TxRetries,
    TxFailed,
    SignalAvg,
    RxBitrate,
    BssParam,
    ConnectedTime,
    StaFlags,
    BeaconLoss,
    TOffset,
    LocalPm,
    PeerPm,
    NonpeerPm,
    RxBytes64,
    TxBytes64,
    ChainSignal,
    ChainSignalAvg,
    ExpectedThroughput,

    AfterLast
}

pub enum MpathFlags {
    Active = 1,
    Resolving = 2,
    SnValid = 4,
    Fixed = 8,
    Resolved = 16
}

pub enum MpathInfo {
    __Invalid,
    FrameQlen,
    Sn,
    Metric,
    Exptime,
    Flags,
    DiscoveryTimeout,
    DiscoveryRetries,

    AfterLast,
}

pub enum BandAttr {
    __Invalid,
    Freq,
    Rates,

    HtMcsSet,
    HdCapa,
    HtAmpduFactor,
    HtAmpduDensity,

    VhtMcsSet,
    VhtCapa,

    AfterLast,
}

pub enum FrequencyAttr {
    __Invalid,
    Freq,
    Disabled,
    NoIr,
    NoIbss,
    Radar,
    MaxTxPower,
    DfsState,
    DfsTime,
    NoHt40Minus,
    NoHt40Plus,
    No80Mhz,
    No160Mhz,
    DfsCacTime,
    IndoorOnly,
    GoConcurrent,
    No20Mhz,
    No10Mhz,

    AfterLast
}


pub enum BitrateAttr {
    __Invalid,
    Rate,
    ShortPreamble2Ghz,

    AfterLast
}

pub enum RegInitiator {
    SetByCore,
    SetByUser,
    SetByDriver,
    SetByCountryIe,
}

pub enum RegType {
    Country,
    World,
    CustomWorld,
    Intersection,
}

pub enum RegRuleAttr {
    __Invalid,
    RuleFlags,

    FreqRangeState,
    FreqRangeEnd,
    FreqRangeMaxBw,

    PowerRuleMaxAntGain,
    PowerRuleMaxEirp,

    DfsCacTime,

    AfterLast,
}

pub enum SchedScanMatAttr {
    __Invalid,
    Ssid,
    Rssi,

    AfterLast,
}

pub enum RegRuleFlags {
    NoOfdm = 1,
    NoCck = 2,
    NoIndoor = 4,
    NoOutdoor = 8,
    Dfs = 16,
    PtpOnly = 32,
    PtmpOnly = 64,
    NoIr = 128,
    NoIbss = 256,

    AutoBw = 2048
}

pub enum DfsRegions {
    Unset,
    Fcc,
    Etsi,
    Jp,
}

pub enum UserRegHintType {
    User,
    CellBase,
    Indoor
}

pub enum SurveyInfo {
    __Invalid,
    Frequency,
    Noise,
    InUse,
    ChannelTime,
    ChannelTimeBusy,
    ChannelTimeExtBusy,
    ChannelTimeRx,
    ChannelTimTx,

    AfterLast
}

pub enum MntrFlags {
    __Invalid,
    FcsFail,
    PlcpFail,
    Control,
    OtherBss,
    CookFrames,
    Active,

    AfterLast
}

pub enum MeshPowerMode {
    Unkown,
    Active,
    LightSleep,
    DeepSleep,

    AfterLast
}

pub enum MechConfParams {
    __Invalid,
    RetryTimeout,
    ConfirmTimeout,
    HoldingTimeout,
    MaxPeerLinks,
    MaxRetries,
    Ttl,
    AutoOpenPlinks,
    HwmpMaxPreqRetries,
    PathRefreshTime,
    MinDiscoveryTimeout,
    HwmpActivePathTimeout,
    HwmpPreqMinInterval,
    HwmpNetDiamTrvsTime,
    HwmpRootmode,
    ElementTtl,
    HwmpRannInterval,
    GateAnnouncement,
    HwmpPerrMinInterval,
    Forwarding,
    RssiThreshold,
    SyncOffsetMaxNeighbor,
    HtOpmode,
    HwmpPathToRootTimeout,
    HwmpRootInterval,
    HwmpConfirmationInterval,
    PowerMode,
    AwakeWindow,
    PlinkTimeout,

    AfterLast
}

pub enum MeshSetupParams {
    __Invalid,
    EnableVendorPathSel,
    EnableVendorMetric,
    Ie,
    UserspaceAuth,
    UserspaceAmpe,
    EnableVendorSync,
    UserspaceMpm,
    AuthProtocol,

    AfterLast
}

pub enum TxqAttr {
    __Invalid,

    Ac,
    Txop,
    Cwmin,
    Cwmax,
    Aifs,

    AfterLast
    
}

pub enum Ac {
    Vo,
    Vi,
    Be,
    Bk,
    NumAcs
}

pub enum ChannelType {
    NoHt,
    Ht20,
    Ht40Minus,
    Ht40Plus,
}

pub enum ChannelWidth {
    Width20NoHt,
    Width20,
    Width40,
    Width80,
    Width80P80,
    Width160,
    Width5,
    Width10,
}

pub enum BssScanWidth {
    Width20,
    Width10,
    Width5
}

pub enum Bss {
    __Invalid,
    Bssid,
    Frequency,
    Tsf,
    BeaconInterval,
    Capability,
    InformationElements,
    SignalMbm,
    SignalUnspec,
    Status,
    SeenMsAgo,
    BeaconIes,
    ChanWidth,
    BeaconTsf,
    PrespData,

    AfterLast
}

pub enum  BssStatus {
    Authenticated,
    Associated,
    IbssJoined,
}

pub enum AuthType {
    OpenSystem,
    SharedKey,
    Ft,
    NetworkEap,
    Sae,

    Num,
    Max,
    Automatic
}

pub enum KeyType {
    Group,
    Pairwise,
    Peerkey,

    Num
}

pub enum Mpf {
    No,
    Required
}

pub enum WpaVersions {
    Version1 = 1,
    Version2 = 2
}

pub enum KeyDefaultTypes {
    __Invalid,
    Unicast,
    Multicast,

    Num
}

pub enum KeyAttributes {
    __Invalid,
    Data,
    Idx,
    Cipher,
    Seq,
    Default,
    DefaultMgmt,
    Type,
    DefaultTypes,

    Afterlast,
}

pub enum TxRateAttributes {
    __Invalid,
    Legacy,
    Ht,
    Vht,
    Gi,

    AfterLast
}

pub struct TxrateVht {
    mcs: [u16;8]
}

pub enum TxrateGi {
    DefaultGi,
    ForceSgi,
    ForceLgi
}

pub enum Band {
    Band2Ghz,
    Band5Ghz,
    Band60Ghz
}

pub enum PsState {
    Disabled,
    Enabled,
}

pub enum AttrCqm {
    __Invalid,
    RssiThold,
    RssiHyst,
    RssiThresholdEvent,
    PktLossEvent,
    TxeRate,
    TxePkts,
    TxeIntvl,

    AfterLast
}

pub enum CqmRssiThesholdEvent {
    Low,
    High,
    BeaconLoss
}

pub enum TxPowerSettings {
    Automatic,
    Limited,
    Fixed
}

pub enum PacketPatternAttr {
    __Invalid,
    Mask,
    Pattern,
    Offset,

    Num,
}

#[repr(C, packed)]
pub struct PatternSupport {
    max_patterns: u32,
    min_pattern_len: u32,
    max_pattern_len: u32,
    max_pkt_offset: u32
}

#[repr(C)]
pub enum WoWlanTriggers {
    __Invalid,
    Any,
    Disconnect,
    MagicPkt,
    PktPattern,
    GtkRekeySupported,
    GtkRekeyFailure,
    EapIdentRequest,
    Handshake4Way,
    RfkillRelease,
    WakeupPkt80211,
    WakeupPkt80211Len,
    WakeupPkt8023,
    WakeupPkt8023Len,
    TcpConnection,
    WakeupTcpMatch,
    WakeupTcpConnLost,
    WakeupTcpNoMoreTokens,

    Num
}

#[repr(C)]
pub struct WoWlanTcpDataSeq {
    start: u32,
    offset: u32, 
    len: u32
}

#[repr(C)]
pub struct WoWlanTcpDataToken {
    offset: u32,
    len: u32,

    token_stream: [u8]
}

#[repr(C)]
pub struct WoWlanTcpDataTokenFeature {
    min_len: u32,
    max_len: u32,
    bufsize: u32,
}

pub enum WoWlanTcpAttr {
    __Invalid,
    SrcIpv4,
    DstIpv4,
    DstMac,
    SrcPort,
    DstPort,
    DataPayload,
    DataPayloadSeq,
    DataPayloadToken,
    DataInterval,
    WakePayload,
    WakeMask,

    Num
}

#[repr(C, packed)]
pub struct CoalesceRuleSupport {
    max_rules: u32,
    pat: PatternSupport,
    max_delay: u32,
}

pub enum CoalesceRule {
    __Invalid,
    Delay,
    Condition,
    PktPattern,

    Num
}

pub enum CoalesceCondition {
    Match,
    NoMatch
}

pub enum IfaceLimitAttr {
    Unspec,
    Max,
    Types,

    Num
}

pub enum IfCombinationAttr {
    Unspec,
    Limits,
    Maxnum,
    StaApBiMatch,
    NumChannels,
    RadarDetectWidths,
    RadarDetectRegions,

    Num
}

pub enum PlinkState {
    Listen,
    OpnSnt,
    OpnRcvd,
    CnfRcvd,
    Estab,
    Holding,
    Blocked,

    Num
}

pub enum PlinkActions {
    NoAction,
    Open,
    Block,

    Num
}

pub enum RekeyData {
    __Invalid,
    Kek,
    Kck,
    ReplayCtr,

    Num
}

pub enum HiddenSsid {
    NotInUse,
    ZeroLen,
    ZeroContents,
}

pub enum StaWmeAttr {
    __Invalid,
    UapsdQueues,
    MaxSp,

    AfterLast
}

pub enum PmksaCandidateAttr {
    __Invalid,
    Index,
    Bssid,
    Preauth,

    Num
}

pub enum TldsOperation {
    DiscoverReq,
    Setup,
    Teardown,
    EnableLink,
    DisableLink,
}

pub enum FeatureFlags {
    SkTxStatus              = 1 << 0,
    HtIbss                  = 1 << 1,
    InactivityTimer         = 1 << 2,
    CellBaseRegHints        = 1 << 3,
    P2pDeviceNeedsChannel   = 1 << 4,
    Sae                     = 1 << 5,
    LowPriorityScan         = 1 << 6,
    ScanFlush               = 1 << 7,
    ApScan                  = 1 << 8,
    VifTxPower              = 1 << 9,
    NeedObssScan            = 1 << 10,
    P2pGoCtwim              = 1 << 11,
    P2pGoOppps              = 1 << 12,

    AdvertiseChanLimits     = 1 << 14,
    FullApClientState       = 1 << 15,
    UserspaceMpm            = 1 << 16,
    ActiveMonitor           = 1 << 17,
    ApModeChanWidthChange   = 1 << 18,
    DsParamSetIeInProbes    = 1 << 19,
    WfaTpcIeInProbes        = 1 << 20,
    Quiet                   = 1 << 21,
    TxPowerInsertion        = 1 << 22,
    AcktoEstimation         = 1 << 23,
    StaticSmps              = 1 << 24,
    DynamicSmps             = 1 << 25,
    SupportsWmmAdmission    = 1 << 26,
    MacOnCreate             = 1 << 27
}

pub enum ProbeRespOffloadSupportAttr {
    Wps = 1,
    Wps2 = 2,
    P2p = 4,
    U80211 = 8
}

pub enum ConnectFailReason {
    MaxClients,
    BlockedClient,
}

pub enum ScanFlags {
    LowPriority = 1,
    Flush = 2,
    Ap = 4
}

pub enum AclPolicy {
    AcceptUnlessListed,
    DenyUnlessListed
}

pub enum SmpsMode {
    Off,
    Static,
    Dynamic,

    AfterLast
}

pub enum RadarEvent {
    Detected,
    CacFinished,
    CacAborted,
    NopFinished
}

pub enum DfsState {
    Usable,
    Unavialable,
    Available
}

pub enum ProtocolFeatures {
    SplitWiphyDump = 1
}

pub enum CritProtoId {
    Unspec,
    Dhcp,
    Eapol,
    Apipa,
    // Other protocols go here
    Num
}

pub enum RxmgmtFlags {
    Answered = 1
}

pub struct VendorCmdInfo {
    vendor_id: u32,
    subcmd: u32,
}

pub enum TdlsPeerCapability {
    Ht = 1,
    Vht = 2,
    Wmm = 4
}


pub mod constants {
    pub const GENL_NAME : &'static str = "nl80211";

    pub const MAX_SUPP_RATES: u8 = 32;
    pub const MAX_SUPP_HT_RATE: u8 = 77;
    pub const MAX_SUPP_REG_RULES: u8 = 32;
    pub const TKIP_DATA_OFFSET_ENCR_KEY: u8 = 0;
    pub const TKIP_DATA_OFFSET_TX_MIC_KEY: u8 = 16;
    pub const TKIP_DATA_OFFSET_RX_MIC_KEY: u8 = 24;
    pub const HT_CAPABILITY_LEN: u8 = 26;
    pub const VHT_CAPABILITY_LEN: u8 = 12;

    pub const MAX_NR_CIPHER_SUITES: u8 = 5;
    pub const MAX_NR_AKM_SUITES: u8 = 2;

    pub const MIN_REMAIN_ON_CHANNEL_TIME: u8 = 10;

    pub const SCAN_RSSI_THOLD_OFF: i16 = -300;

    pub const CQM_TXE_MAX_INTVL: u32 = 1800;

    pub const VENDORIDISLINUX :u32 = 0x80000000;

    pub const VHT_NSS_MAX: u8 = 8;

    pub const KCK_LEN: u8 = 16;
    pub const KEK_LEN: u8 = 16;
    pub const REPLAT_CTR_LEN: u8 = 16;

    pub const CRIT_PROTO_MAX_DURATION: u32 = 5000; // msec
}
