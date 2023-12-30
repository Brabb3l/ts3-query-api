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

        client_ids("clid"): Vec<u32>,
        reason_id("reasonid"): ReasonId,
        reason_msg("reasonmsg"): Option<String>,

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

        nickname("client_nickname"): String,
        unique_identifier("client_unique_identifier"): String,
        database_id("client_database_id"): u32,

        nickname_phonetic("client_nickname_phonetic"): Option<String>,
        description("client_description"): Option<String>,
        icon_id("client_icon_id"): u32,
        country("client_country"): Option<String>,
        badges("client_badges"): Badges,
        signed_badges("client_signed_badges"): Vec<String>,
        myteamspeak_id("client_myteamspeak_id"): String,
        myteamspeak_avatar("client_myteamspeak_avatar"): Option<String>,
        integrations("client_integrations"): Option<String>,
        flag_avatar("client_flag_avatar"): Option<String>,

        away("client_away"): bool,
        away_message("client_away_message"): Option<String>,

        meta_data("client_meta_data"): Option<String>,
        unread_messages("client_unread_messages"): u32,

        channel_group_id("client_channel_group_id"): u32,
        server_groups("client_servergroups"): Vec<u32>,

        is_query("client_type"): bool,

        input_muted("client_input_muted"): bool,
        output_muted("client_output_muted"): bool,
        outputonly_muted("client_outputonly_muted"): bool,
        input_hardware("client_input_hardware"): bool,
        output_hardware("client_output_hardware"): bool,
        talk_power("client_talk_power"): u32,
        talk_request("client_talk_request"): bool,
        talk_request_msg("client_talk_request_msg"): Option<String>,
        is_talker("client_is_talker"): bool,
        is_priority_speaker("client_is_priority_speaker"): bool,
        is_channel_commander("client_is_channel_commander"): bool,
        is_recording("client_is_recording"): bool,

        needed_serverquery_view_power("client_needed_serverquery_view_power"): u32,
        channel_group_inherited_channel_id("client_channel_group_inherited_channel_id"): u32,
    }
}

ts_response! {
    ClientLeftViewEvent {
        invoker_id("invokerid"): Option<u32>,
        invoker_name("invokername"): Option<String>,
        invoker_uid("invokeruid"): Option<String>,

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

        id("cid"): u32,
        parent_id("cpid"): u32,

        name("channel_name"): String,

        name_phonetic("channel_name_phonetic"): Option<String>,
        topic("channel_topic"): Option<String>,
        icon_id("channel_icon_id"): Option<u32>,

        codec("channel_codec"): Codec = Codec::OpusVoice,
        codec_quality("channel_codec_quality"): u8 = 6,
        codec_latency_factor("channel_codec_latency_factor"): u32 = 1,
        codec_is_unencrypted("channel_codec_is_unencrypted"): bool,

        max_clients("channel_maxclients"): i32 = -1,
        max_family_clients("channel_maxfamilyclients"): i32 = -1,
        order("channel_order"): u32,

        flag_permanent("channel_flag_permanent"): bool = false,
        flag_semi_permanent("channel_flag_semi_permanent"): bool = false,
        flag_default("channel_flag_default"): bool = false,
        flag_max_clients_unlimited("channel_flag_maxclients_unlimited"): bool = true,
        flag_max_family_clients_unlimited("channel_flag_maxfamilyclients_unlimited"): bool,
        flag_max_family_clients_inherited("channel_flag_maxfamilyclients_inherited"): bool,
        flag_password("channel_flag_password"): bool = false,

        needed_talk_power("channel_needed_talk_power"): u32 = 0,

        delete_delay("channel_delete_delay"): u32 = 0,
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

        id("cid"): u32,
        reason_id("reasonid"): ReasonId,

        name("channel_name"): Option<String>,

        name_phonetic("channel_name_phonetic"): Option<String>,
        topic("channel_topic"): Option<String>,
        icon_id("channel_icon_id"): Option<u32>,

        codec("channel_codec"): Option<Codec>,
        codec_quality("channel_codec_quality"): Option<u8>,
        codec_latency_factor("channel_codec_latency_factor"): Option<u32>,
        codec_is_unencrypted("channel_codec_is_unencrypted"): Option<bool>,

        max_clients("channel_maxclients"): Option<u32>,
        max_family_clients("channel_maxfamilyclients"): Option<u32>,
        order("channel_order"): Option<u32>,

        flag_permanent("channel_flag_permanent"): Option<bool>,
        flag_semi_permanent("channel_flag_semi_permanent"): Option<bool>,
        flag_default("channel_flag_default"): Option<bool>,
        flag_max_clients_unlimited("channel_flag_maxclients_unlimited"): Option<bool>,
        flag_max_family_clients_unlimited("channel_flag_maxfamilyclients_unlimited"): Option<bool>,
        flag_max_family_clients_inherited("channel_flag_maxfamilyclients_inherited"): Option<bool>,
        flag_password("channel_flag_password"): Option<bool>,

        needed_talk_power("channel_needed_talk_power"): Option<u32>,

        delete_delay("channel_delete_delay"): Option<u32>,
    }
}

ts_response! {
    ChannelMoveEvent {
        invoker_id("invokerid"): u32,
        invoker_name("invokername"): String,
        invoker_uid("invokeruid"): Option<String>,

        id("cid"): u32,
        reason_id("reasonid"): ReasonId,

        parent_id("cpid"): u32,
        order: u32,
    }
}

ts_response! {
    ChannelDescriptionChangeEvent {
        id("cid"): u32,
    }
}

ts_response! {
    ChannelPasswordChangeEvent {
        id("cid"): u32,
    }
}

ts_response! {
    ServerEditEvent {
        invoker_id("invokerid"): u32,
        invoker_name("invokername"): String,
        invoker_uid("invokeruid"): Option<String>,

        reason_id("reasonid"): ReasonId,

        name("virtualserver_name"): Option<String>,
        nickname("virtualserver_nickname"): Option<String>,
        codec_encryption_mode("virtualserver_codec_encryption_mode"): Option<CodecEncryptionMode>,
        default_server_group("virtualserver_default_server_group"): Option<u32>,
        default_channel_group("virtualserver_default_channel_group"): Option<u32>,
        hostbanner_url("virtualserver_hostbanner_url"): Option<String>,
        hostbanner_gfx_url("virtualserver_hostbanner_gfx_url"): Option<String>,
        hostbanner_gfx_interval("virtualserver_hostbanner_gfx_interval"): Option<u32>,
        priority_speaker_dimm_modificator("virtualserver_priority_speaker_dimm_modificator"): Option<f32>,
        hostbutton_tooltip("virtualserver_hostbutton_tooltip"): Option<String>,
        hostbutton_url("virtualserver_hostbutton_url"): Option<String>,
        hostbutton_gfx_url("virtualserver_hostbutton_gfx_url"): Option<String>,
        name_phonetic("virtualserver_name_phonetic"): Option<String>,
        icon_id("virtualserver_icon_id"): Option<u32>,
        hostbanner_mode("virtualserver_hostbanner_mode"): Option<HostBannerMode>,
        channel_temp_delete_delay_default("virtualserver_channel_temp_delete_delay_default"): Option<u32>,
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
