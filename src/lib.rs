
mod Alias {
    pub enum Commands {
        NewBeacon = 15,
        DelBeacon = 17,
        RegisterAction = 60,
        Action = 62,
        ActionTxStatus = 64,
    }
}
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
    Action,         // Frame alias
    AliasFrameTxStatus,
    AliasActionTxStatus, // Frame Tx Status

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

    CmdMax

}
