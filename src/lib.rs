
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
