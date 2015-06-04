
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
