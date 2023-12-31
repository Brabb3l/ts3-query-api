use super::*;
use crate::macros::ts_response;

ts_response! {
    TextMessageEvent {
        invoker_id("invokerid"): i32,
        invoker_name("invokername"): String,
        invoker_uid("invokeruid"): Option<String>,

        target_mode("targetmode"): i32,
        message("msg"): String,
    }
}

ts_response! {
    ClientMoveEvent {
        invoker_id("invokerid"): Option<i32>,
        invoker_name("invokername"): Option<String>,
        invoker_uid("invokeruid"): Option<String>,

        client_ids("clid"): Vec<i32>,
        reason_id("reasonid"): ReasonId = ReasonId::Move,
        reason_msg("reasonmsg"): Option<String>,

        channel_to_id("ctid"): i32,
        channel_from_id("cfid"): Option<i32>,
    }
}

ts_response! {
    ClientEnterViewEvent {
        client_id("clid"): i32,

        reason_id("reasonid"): ReasonId = ReasonId::JoinChannel,
        reason_msg("reasonmsg"): Option<String>,

        channel_from_id("cfid"): i32,
        channel_to_id("ctid"): i32,

        nickname("client_nickname"): String,
        unique_identifier("client_unique_identifier"): String,
        database_id("client_database_id"): i32,

        nickname_phonetic("client_nickname_phonetic"): Option<String>,
        description("client_description"): Option<String>,
        icon_id("client_icon_id"): i32,
        country("client_country"): Option<String>,
        badges("client_badges"): Badges = Badges::default(),
        signed_badges("client_signed_badges"): Vec<String>,
        myteamspeak_id("client_myteamspeak_id"): Option<String>,
        myteamspeak_avatar("client_myteamspeak_avatar"): Option<String>,
        integrations("client_integrations"): Option<String>,
        flag_avatar("client_flag_avatar"): Option<String>,

        away("client_away"): bool = false,
        away_message("client_away_message"): Option<String>,

        meta_data("client_meta_data"): Option<String>,
        unread_messages("client_unread_messages"): i32 = 0,

        channel_group_id("client_channel_group_id"): Option<i32>,
        server_groups("client_servergroups"): Vec<i32>,

        is_query("client_type"): bool,

        input_muted("client_input_muted"): bool = false,
        output_muted("client_output_muted"): bool = false,
        outputonly_muted("client_outputonly_muted"): bool = false,
        input_hardware("client_input_hardware"): bool = true,
        output_hardware("client_output_hardware"): bool = true,
        talk_power("client_talk_power"): i32 = 0,
        talk_request("client_talk_request"): bool = false,
        talk_request_msg("client_talk_request_msg"): Option<String>,
        is_talker("client_is_talker"): bool = false,
        is_priority_speaker("client_is_priority_speaker"): bool = false,
        is_channel_commander("client_is_channel_commander"): bool = false,
        is_recording("client_is_recording"): bool = false,

        needed_serverquery_view_power("client_needed_serverquery_view_power"): i32,
        channel_group_inherited_channel_id("client_channel_group_inherited_channel_id"): i32,

        // ts5 only
        user_tag("client_user_tag"): Option<String>,
    }
}

ts_response! {
    ClientLeftViewEvent {
        invoker_id("invokerid"): Option<i32>,
        invoker_name("invokername"): Option<String>,
        invoker_uid("invokeruid"): Option<String>,

        client_id("clid"): i32,
        reason_id("reasonid"): ReasonId = ReasonId::Leave,
        reason_msg("reasonmsg"): Option<String>,

        channel_from_id("cfid"): Option<i32>,
        channel_to_id("ctid"): Option<i32>,
        ban_time("bantime"): Option<i32>,
    }
}

ts_response! {
    ChannelCreateEvent {
        invoker_id("invokerid"): i32,
        invoker_name("invokername"): String,
        invoker_uid("invokeruid"): Option<String>,

        id("cid"): i32,
        parent_id("cpid"): i32,

        name("channel_name"): String,

        name_phonetic("channel_name_phonetic"): Option<String>,
        topic("channel_topic"): Option<String>,
        icon_id("channel_icon_id"): Option<i32>,

        codec("channel_codec"): Codec = Codec::OpusVoice,
        codec_quality("channel_codec_quality"): u8 = 6,
        codec_latency_factor("channel_codec_latency_factor"): i32 = 1,
        codec_is_unencrypted("channel_codec_is_unencrypted"): bool = false,

        max_clients("channel_maxclients"): i32 = -1,
        max_family_clients("channel_maxfamilyclients"): i32 = -1,
        order("channel_order"): i32,

        flag_permanent("channel_flag_permanent"): bool = false,
        flag_semi_permanent("channel_flag_semi_permanent"): bool = false,
        flag_default("channel_flag_default"): bool = false,
        flag_max_clients_unlimited("channel_flag_maxclients_unlimited"): bool = true,
        flag_max_family_clients_unlimited("channel_flag_maxfamilyclients_unlimited"): bool = false,
        flag_max_family_clients_inherited("channel_flag_maxfamilyclients_inherited"): bool = true,
        flag_password("channel_flag_password"): bool = false,

        needed_talk_power("channel_needed_talk_power"): i32 = 0,

        delete_delay("channel_delete_delay"): i32 = 0,
    }
}

ts_response! {
    ChannelDeleteEvent {
        invoker_id("invokerid"): i32,
        invoker_name("invokername"): String,
        invoker_uid("invokeruid"): Option<String>,

        channel_id("cid"): i32,
    }
}

ts_response! {
    ChannelEditEvent {
        invoker_id("invokerid"): i32,
        invoker_name("invokername"): String,
        invoker_uid("invokeruid"): Option<String>,

        id("cid"): i32,
        reason_id("reasonid"): ReasonId = ReasonId::Edit,

        name("channel_name"): Option<String>,

        name_phonetic("channel_name_phonetic"): Option<String>,
        topic("channel_topic"): Option<String>,
        icon_id("channel_icon_id"): Option<i32>,

        codec("channel_codec"): Option<Codec>,
        codec_quality("channel_codec_quality"): Option<u8>,
        codec_latency_factor("channel_codec_latency_factor"): Option<i32>,
        codec_is_unencrypted("channel_codec_is_unencrypted"): Option<bool>,

        max_clients("channel_maxclients"): Option<i32>,
        max_family_clients("channel_maxfamilyclients"): Option<i32>,
        order("channel_order"): Option<i32>,

        flag_permanent("channel_flag_permanent"): Option<bool>,
        flag_semi_permanent("channel_flag_semi_permanent"): Option<bool>,
        flag_default("channel_flag_default"): Option<bool>,
        flag_max_clients_unlimited("channel_flag_maxclients_unlimited"): Option<bool>,
        flag_max_family_clients_unlimited("channel_flag_maxfamilyclients_unlimited"): Option<bool>,
        flag_max_family_clients_inherited("channel_flag_maxfamilyclients_inherited"): Option<bool>,
        flag_password("channel_flag_password"): Option<bool>,

        needed_talk_power("channel_needed_talk_power"): Option<i32>,

        delete_delay("channel_delete_delay"): Option<i32>,
    }
}

ts_response! {
    ChannelMoveEvent {
        invoker_id("invokerid"): i32,
        invoker_name("invokername"): String,
        invoker_uid("invokeruid"): Option<String>,

        id("cid"): i32,
        reason_id("reasonid"): ReasonId = ReasonId::Move,

        parent_id("cpid"): i32,
        order: i32,
    }
}

ts_response! {
    ChannelDescriptionChangeEvent {
        id("cid"): i32,
    }
}

ts_response! {
    ChannelPasswordChangeEvent {
        id("cid"): i32,
    }
}

ts_response! {
    ServerEditEvent {
        invoker_id("invokerid"): i32,
        invoker_name("invokername"): String,
        invoker_uid("invokeruid"): Option<String>,

        reason_id("reasonid"): ReasonId = ReasonId::Edit,

        name("virtualserver_name"): Option<String>,
        nickname("virtualserver_nickname"): Option<String>,
        codec_encryption_mode("virtualserver_codec_encryption_mode"): Option<CodecEncryptionMode>,
        default_server_group("virtualserver_default_server_group"): Option<i32>,
        default_channel_group("virtualserver_default_channel_group"): Option<i32>,
        hostbanner_url("virtualserver_hostbanner_url"): Option<String>,
        hostbanner_gfx_url("virtualserver_hostbanner_gfx_url"): Option<String>,
        hostbanner_gfx_interval("virtualserver_hostbanner_gfx_interval"): Option<i32>,
        priority_speaker_dimm_modificator("virtualserver_priority_speaker_dimm_modificator"): Option<f32>,
        hostbutton_tooltip("virtualserver_hostbutton_tooltip"): Option<String>,
        hostbutton_url("virtualserver_hostbutton_url"): Option<String>,
        hostbutton_gfx_url("virtualserver_hostbutton_gfx_url"): Option<String>,
        name_phonetic("virtualserver_name_phonetic"): Option<String>,
        icon_id("virtualserver_icon_id"): Option<i32>,
        hostbanner_mode("virtualserver_hostbanner_mode"): Option<HostBannerMode>,
        channel_temp_delete_delay_default("virtualserver_channel_temp_delete_delay_default"): Option<i32>,
    }
}

ts_response! {
    TokenUseEvent {
        client_id("clid"): i32,
        client_database_id("cldbid"): i32,
        client_uid("cluid"): String,

        token: String,
        token_custom_set("tokencustomset"): String,
        token_group_id("token1"): i32,
        token_channel_id("token2"): i32,
    }
}
