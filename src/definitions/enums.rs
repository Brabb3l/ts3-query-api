use crate::macros::ts_enum;

ts_enum! {
    HostMessageMode<i32> {
        None = 0,
        Log = 1,
        Modal = 2,
        ModalQuit = 3
    }
}

ts_enum! {
    HostBannerMode<i32> {
        NoAdjust = 0,
        AdjustIgnoreAspect = 1,
        AdjustKeepAspect = 2
    }
}

ts_enum! {
    Codec<i32> {
        SpeexNarrowband = 0,
        SpeexWideband = 1,
        SpeexUltraWideband = 2,
        Celt = 3,
        OpusVoice = 4,
        OpusMusic = 5
    }
}

ts_enum! {
    CodecEncryptionMode<i32> {
        Individual = 0,
        Disabled = 1,
        Enabled = 2
    }
}

ts_enum! {
    TextMessageTargetMode<i32> {
        Client = 1,
        Channel = 2,
        Server = 3
    }
}

ts_enum! {
    ReasonId<i32> {
        JoinChannel = 0,
        Move = 1,
        // ???
        Timeout = 3,
        ChannelKick = 4,
        ServerKick = 5,
        Ban = 6,
        // ???
        Leave = 8,
        // ???
        Edit = 10,
        ServerShutdown = 11,
    }
}

ts_enum! {
    Scope<str> {
        MANAGE = "manage",
        WRITE = "write",
        READ = "read",
    }
}
