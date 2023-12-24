use crate::macros::ts_response;
use super::*;

ts_response! {
    TextMessageEvent {
        invoker_id("invokerid"): u32,
        invoker_name("invokername"): String,
        invoker_uid("invokeruid"): Option<String>,

        target_mode("targetmode"): u32,
        message("msg"): String,
    }
}

ts_response! {
    ClientMoveEvent {
        invoker_id("invokerid"): Option<u32>,
        invoker_name("invokername"): Option<String>,
        invoker_uid("invokeruid"): Option<String>,

        client_id("clid"): u32,
        reason_id("reasonid"): ReasonId,
        reason_msg("reasonmsg"): Option<String>,

        channel_from_id("cfid"): u32,
        channel_to_id("ctid"): u32,
    }
}

ts_response! {
    ClientEnterViewEvent {
        client_id("clid"): u32,

        reason_id("reasonid"): ReasonId,
        reason_msg("reasonmsg"): Option<String>,

        channel_from_id("cfid"): u32,
        channel_to_id("ctid"): u32,

        client_nickname: String,
        client_unique_identifier: String,
        client_database_id: u32,

        client_nickname_phonetic: Option<String>,
        client_description: Option<String>,
        client_icon_id: u32,
        client_country: Option<String>,
        client_badges: Badges,
        client_signed_badges: Vec<String>,
        client_myteamspeak_id: String,
        client_myteamspeak_avatar: Option<String>,
        client_integrations: Option<String>,
        client_flag_avatar: Option<String>,

        client_away: bool,
        client_away_message: Option<String>,

        client_meta_data: Option<String>,
        client_unread_messages: u32,

        client_channel_group_id: u32,
        client_servergroups: Vec<u32>,

        is_query("client_type"): bool,

        client_input_muted: bool,
        client_output_muted: bool,
        client_outputonly_muted: bool,
        client_input_hardware: bool,
        client_output_hardware: bool,
        client_talk_power: u32,
        client_talk_request: bool,
        client_talk_request_msg: Option<String>,
        client_is_talker: bool,
        client_is_priority_speaker: bool,
        client_is_channel_commander: bool,
        client_is_recording: bool,

        client_needed_serverquery_view_power: u32,
        client_channel_group_inherited_channel_id: u32,
    }
}

ts_response! {
    ClientLeftViewEvent {
        invoker_id("invokerid"): u32,
        invoker_name("invokername"): String,
        invoker_uid("invokeruid"): String,

        client_id("clid"): u32,
        reason_id("reasonid"): ReasonId,
        reason_msg("reasonmsg"): Option<String>,

        channel_from_id("cfid"): Option<u32>,
        channel_to_id("ctid"): Option<u32>,
        ban_time("bantime"): Option<u32>,
    }
}

ts_response! {
    ChannelCreateEvent {
        invoker_id("invokerid"): u32,
        invoker_name("invokername"): String,
        invoker_uid("invokeruid"): Option<String>,

        channel_id("cid"): u32,
        channel_parent_id("cpid"): u32,

        channel_name: Option<String>,
        channel_topic: Option<String>,
        channel_codec: Option<Codec>,
        channel_codec_quality: Option<u8>,
        channel_maxclients: Option<u32>,
        channel_maxfamilyclients: Option<u32>,
        channel_order: Option<u32>,
        channel_flag_permanent: Option<bool>,
        channel_flag_semi_permanent: Option<bool>,
        channel_flag_default: Option<bool>,
        channel_flag_password: Option<bool>,
        channel_codec_latency_factor: Option<u32>,
        channel_codec_is_unencrypted: Option<bool>,
        channel_delete_delay: Option<u32>,
        channel_flag_maxclients_unlimited: Option<bool>,
        channel_flag_maxfamilyclients_unlimited: Option<bool>,
        channel_flag_maxfamilyclients_inherited: Option<bool>,
        channel_needed_talk_power: Option<u32>,
        channel_name_phonetic: Option<String>,
        channel_icon_id: Option<u32>,
    }
}

ts_response! {
    ChannelDeleteEvent {
        invoker_id("invokerid"): u32,
        invoker_name("invokername"): String,
        invoker_uid("invokeruid"): Option<String>,

        channel_id("cid"): u32,
    }
}

ts_response! {
    ChannelEditEvent {
        invoker_id("invokerid"): u32,
        invoker_name("invokername"): String,
        invoker_uid("invokeruid"): Option<String>,

        channel_id("cid"): u32,
        reason_id("reasonid"): ReasonId,

        channel_name: Option<String>,
        channel_topic: Option<String>,
        channel_codec: Option<Codec>,
        channel_codec_quality: Option<u8>,
        channel_maxclients: Option<u32>,
        channel_maxfamilyclients: Option<u32>,
        channel_order: Option<u32>,
        channel_flag_permanent: Option<bool>,
        channel_flag_semi_permanent: Option<bool>,
        channel_flag_default: Option<bool>,
        channel_flag_password: Option<bool>,
        channel_codec_latency_factor: Option<u32>,
        channel_codec_is_unencrypted: Option<bool>,
        channel_delete_delay: Option<u32>,
        channel_flag_maxclients_unlimited: Option<bool>,
        channel_flag_maxfamilyclients_unlimited: Option<bool>,
        channel_flag_maxfamilyclients_inherited: Option<bool>,
        channel_needed_talk_power: Option<u32>,
        channel_name_phonetic: Option<String>,
        channel_icon_id: Option<u32>,
    }
}

ts_response! {
    ChannelMoveEvent {
        invoker_id("invokerid"): u32,
        invoker_name("invokername"): String,
        invoker_uid("invokeruid"): Option<String>,

        channel_id("cid"): u32,
        reason_id("reasonid"): ReasonId,

        channel_parent_id("cpid"): u32,
        order: u32,
    }
}

ts_response! {
    ChannelDescriptionChangeEvent {
        channel_id("cid"): u32,
    }
}

ts_response! {
    ChannelPasswordChangeEvent {
        channel_id("cid"): u32,
    }
}

ts_response! {
    ServerEditEvent {
        invoker_id("invokerid"): u32,
        invoker_name("invokername"): String,
        invoker_uid("invokeruid"): Option<String>,

        reason_id("reasonid"): ReasonId,

        virtualserver_name: Option<String>,
        virtualserver_nickname: Option<String>,
        virtualserver_codec_encryption_mode: Option<CodecEncryptionMode>,
        virtualserver_default_server_group: Option<u32>,
        virtualserver_default_channel_group: Option<u32>,
        virtualserver_hostbanner_url: Option<String>,
        virtualserver_hostbanner_gfx_url: Option<String>,
        virtualserver_hostbanner_gfx_interval: Option<u32>,
        virtualserver_priority_speaker_dimm_modificator: Option<f32>,
        virtualserver_hostbutton_tooltip: Option<String>,
        virtualserver_hostbutton_url: Option<String>,
        virtualserver_hostbutton_gfx_url: Option<String>,
        virtualserver_name_phonetic: Option<String>,
        virtualserver_icon_id: Option<u32>,
        virtualserver_hostbanner_mode: Option<HostBannerMode>,
        virtualserver_channel_temp_delete_delay_default: Option<u32>,
    }
}

ts_response! {
    TokenUseEvent {
        client_id("clid"): u32,
        client_database_id("cldbid"): u32,
        client_uid("cluid"): String,

        token: String,
        token_custom_set("tokencustomset"): String,
        token_group_id("token1"): u32,
        token_channel_id("token2"): u32,
    }
}
