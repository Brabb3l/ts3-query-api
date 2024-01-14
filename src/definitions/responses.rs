use crate::definitions::*;
use crate::error::ParseError;
use crate::macros::ts_response;
use crate::parser::DecodeValue;
use std::borrow::Cow;

ts_response! {
    Status {
        id("id"): i32,
        message("msg"): String,
    }
}

ts_response! {
    ChannelId {
        id("cid"): i32
    }
}

ts_response! {
    BanId {
        id("banid"): i32
    }
}

// apikey

ts_response! {
    ApiKey {
        key("apikey"): String,
        id: i32,
        server_id("sid"): i32,
        client_database_id("cldbid"): i32,
        scope: Scope,
        time_left: u64,
        created_at: u64,
        expires_at: u64,
    }
}

// ban

ts_response! {
    Ban {
        id("banid"): i32,

        ip: Option<String>,
        name: Option<String>,
        uid: Option<String>,
        my_teamspeak_id("mytsid"): Option<String>,

        invoker_name("invokername"): String,
        invoker_uid("invokeruid"): String,
        invoker_database_id("invokercldbid"): i32,

        duration: u64,
        reason: Option<String>,
        last_nickname("lastnickname"): Option<String>,

        created: u64,
        enforcements: i32,
    }
}

// version

ts_response! {
    Version {
        version: String,
        build: String,
        platform: String
    }
}

// whoami

ts_response! {
    WhoAmI {
        virtualserver_status: String,
        virtualserver_id: i32,
        virtualserver_unique_identifier: String,
        virtualserver_port: i32,

        id("client_id"): i32,
        database_id("client_database_id"): i32,
        unique_identifier("client_unique_identifier"): String,

        nickname("client_nickname"): String,
        login_name("client_login_name"): String,
        channel_id("client_channel_id"): i32,
        origin_server_id("client_origin_server_id"): i32,
    }
}

// channel list

ts_response! {
    ChannelListEntry {
        id("cid"): i32,
        parent_id("pid"): i32,

        name("channel_name"): String,
        order("channel_order"): i32,
        total_clients: i32,
        needed_subscribe_power("channel_needed_subscribe_power"): i32,
    }
}

ts_response! {
    ChannelListTopicEntry {
        topic("channel_topic"): String
    }
}

ts_response! {
    ChannelListFlagsEntry {
        flag_default("channel_flag_default"): bool,
        flag_password("channel_flag_password"): bool,
        flag_permanent("channel_flag_permanent"): bool,
        flag_semi_permanent("channel_flag_semi_permanent"): bool,
    }
}

ts_response! {
    ChannelListVoiceEntry {
        codec("channel_codec"): Codec = Codec::OpusVoice,
        codec_quality("channel_codec_quality"): i32,
        needed_talk_power("channel_needed_talk_power"): i32
    }
}

ts_response! {
    ChannelListLimitsEntry {
        total_clients_family: i32,
        max_clients("channel_maxclients"): i32,
        max_family_clients("channel_maxfamilyclients"): i32
    }
}

ts_response! {
    ChannelListIconEntry {
        icon_id("channel_icon_id"): i32
    }
}

ts_response! {
    ChannelListSecondsEmptyEntry {
        seconds_empty: i32
    }
}

ts_response! {
    ChannelListBannerEntry {
        banner_gfx_url("channel_banner_gfx_url"): String,
        banner_mode("channel_banner_mode"): i32
    }
}

// channel info

ts_response! {
    ChannelInfo {
        parent_id("pid"): i32,

        name("channel_name"): String,
        unique_identifier("channel_unique_identifier"): String,

        name_phonetic("channel_name_phonetic"): Option<String>,
        topic("channel_topic"): Option<String>,
        description("channel_description"): Option<String>,
        password("channel_password"): Option<String>,
        icon_id("channel_icon_id"): i32,
        banner_gfx_url("channel_banner_gfx_url"): Option<String>,
        banner_mode("channel_banner_mode"): i32,

        codec("channel_codec"): Codec = Codec::OpusVoice,
        codec_quality("channel_codec_quality"): i32,
        codec_latency_factor("channel_codec_latency_factor"): i32,
        codec_is_unencrypted("channel_codec_is_unencrypted"): bool,

        max_clients("channel_maxclients"): i32,
        max_family_clients("channel_maxfamilyclients"): i32,
        order("channel_order"): i32,

        flag_permanent("channel_flag_permanent"): bool,
        flag_semi_permanent("channel_flag_semi_permanent"): bool,
        flag_default("channel_flag_default"): bool,
        flag_max_clients_unlimited("channel_flag_maxclients_unlimited"): bool,
        flag_max_family_clients_unlimited("channel_flag_maxfamilyclients_unlimited"): bool,
        flag_max_family_clients_inherited("channel_flag_maxfamilyclients_inherited"): bool,
        flag_password("channel_flag_password"): bool,

        needed_talk_power("channel_needed_talk_power"): i32,
        forced_silence("channel_forced_silence"): bool,

        filepath("channel_filepath"): Option<String>,
        seconds_empty: i32,
        delete_delay("channel_delete_delay"): i32,
        security_salt("channel_security_salt"): Option<String>,
    }
}

// channel perm list

ts_response! {
    ChannelPermission {
        channel_id("cid"): i32,
        perm: Inline<Permission>,
        perm_negated("permnegated"): bool,
        perm_skip("permskip"): bool,
    }
}

// client list

ts_response! {
    ClientListEntry {
        id("clid"): i32,
        channel_id("cid"): i32,
        database_id("client_database_id"): i32,
        nickname("client_nickname"): String,
        is_query("client_type"): bool
    }
}

ts_response! {
    ClientListUidEntry {
        unique_identifier("client_unique_identifier"): String
    }
}

ts_response! {
    ClientListAwayEntry {
        away("client_away"): bool,
        away_message("client_away_message"): Option<String>
    }
}

ts_response! {
    ClientListVoiceEntry {
        flag_talking("client_flag_talking"): bool,
        input_muted("client_input_muted"): bool,
        output_muted("client_output_muted"): bool,
        input_hardware("client_input_hardware"): bool,
        output_hardware("client_output_hardware"): bool,
        talk_power("client_talk_power"): i32,
        is_talker("client_is_talker"): bool,
        is_priority_speaker("client_is_priority_speaker"): bool,
        is_channel_commander("client_is_channel_commander"): bool,
        is_recording("client_is_recording"): bool,
    }
}

ts_response! {
    ClientListTimesEntry {
        idle_time("client_idle_time"): i32,
        created("client_created"): i32,
        last_connected("client_lastconnected"): i32,
    }
}

ts_response! {
    ClientListGroupsEntry {
        server_groups("client_servergroups"): Vec<i32>,
        channel_group_id("client_channel_group_id"): i32,
        channel_group_inherited_channel_id("client_channel_group_inherited_channel_id"): i32
    }
}

ts_response! {
    ClientListInfoEntry {
        version("client_version"): String,
        platform("client_platform"): String,
    }
}

ts_response! {
    ClientListCountryEntry {
        country("client_country"): Option<String>
    }
}

ts_response! {
    ClientListIpEntry {
        client_ip("connection_client_ip"): String
    }
}

ts_response! {
    ClientListIconEntry {
        icon_id("client_icon_id"): i32
    }
}

ts_response! {
    ClientListBadgesEntry {
        badges("client_badges"): Badges = Badges::default(),
    }
}

// client info

ts_response! {
    ClientInfo {
        nickname("client_nickname"): String,
        unique_identifier("client_unique_identifier"): String,
        database_id("client_database_id"): i32,

        channel_id("cid"): i32,

        version("client_version"): String,
        platform("client_platform"): String,
        base64_hash_client_uid("client_base64HashClientUID"): Option<String>,

        login_name("client_login_name"): Option<String>,
        nickname_phonetic("client_nickname_phonetic"): Option<String>,
        description("client_description"): Option<String>,
        icon_id("client_icon_id"): i32,
        country("client_country"): Option<String>,
        badges("client_badges"): Badges = Badges::default(),
        signed_badges("client_signed_badges"): Vec<String>,
        my_teamspeak_id("client_myteamspeak_id"): Option<String>,
        my_teamspeak_avatar("client_myteamspeak_avatar"): Option<String>,
        integrations("client_integrations"): Option<String>,
        flag_avatar("client_flag_avatar"): Option<String>,

        idle_time("client_idle_time"): i32,
        away("client_away"): bool,
        away_message("client_away_message"): Option<String>,

        default_channel("client_default_channel"): Option<String>,
        meta_data("client_meta_data"): Option<String>,
        version_sign("client_version_sign"): Option<String>,
        security_hash("client_security_hash"): Option<String>,
        unread_messages("client_unread_messages"): i32 = 0,

        channel_group_id("client_channel_group_id"): i32,
        server_groups("client_servergroups"): Vec<i32>,

        created("client_created"): i32,
        last_connected("client_lastconnected"): i32,
        total_connections("client_totalconnections"): i32,

        is_query("client_type"): bool,

        input_muted("client_input_muted"): bool,
        output_muted("client_output_muted"): bool,
        outputonly_muted("client_outputonly_muted"): bool,
        input_hardware("client_input_hardware"): bool,
        output_hardware("client_output_hardware"): bool,
        talk_power("client_talk_power"): i32,
        talk_request("client_talk_request"): bool,
        talk_request_msg("client_talk_request_msg"): Option<String>,
        is_talker("client_is_talker"): bool,
        is_priority_speaker("client_is_priority_speaker"): bool,
        is_channel_commander("client_is_channel_commander"): bool,
        is_recording("client_is_recording"): bool,

        month_bytes_uploaded("client_month_bytes_uploaded"): i32,
        month_bytes_downloaded("client_month_bytes_downloaded"): i32,
        total_bytes_uploaded("client_total_bytes_uploaded"): i32,
        total_bytes_downloaded("client_total_bytes_downloaded"): i32,

        needed_serverquery_view_power("client_needed_serverquery_view_power"): i32,
        channel_group_inherited_channel_id("client_channel_group_inherited_channel_id"): i32,
        default_token("client_default_token"): Option<String>,

        file_transfer_bandwidth_sent("connection_filetransfer_bandwidth_sent"): Option<i32>,
        file_transfer_bandwidth_received("connection_filetransfer_bandwidth_received"): Option<i32>,
        packets_sent_total("connection_packets_sent_total"): i32,
        bytes_sent_total("connection_bytes_sent_total"): i32,
        packets_received_total("connection_packets_received_total"): i32,
        bytes_received_total("connection_bytes_received_total"): i32,
        bandwidth_sent_last_second_total("connection_bandwidth_sent_last_second_total"): i32,
        bandwidth_sent_last_minute_total("connection_bandwidth_sent_last_minute_total"): i32,
        bandwidth_received_last_second_total("connection_bandwidth_received_last_second_total"): i32,
        bandwidth_received_last_minute_total("connection_bandwidth_received_last_minute_total"): i32,
        connected_time("connection_connected_time"): i32,
        client_ip("connection_client_ip"): String,
    }
}

// permissionlist

ts_response! {
    PermissionListEntry {
        id("permid"): i32,
        name("permname"): String,
        description("permdesc"): Option<String>,
    }
}

// serverinfo

ts_response! {
    ServerInfo {
        id("virtualserver_id"): i32,
        unique_identifier("virtualserver_unique_identifier"): String,
        port("virtualserver_port"): i32,

        ip("virtualserver_ip"): String,
        version("virtualserver_version"): String,
        platform("virtualserver_platform"): String,

        min_client_version("virtualserver_min_client_version"): String,
        min_android_version("virtualserver_min_android_version"): String,
        min_ios_version("virtualserver_min_ios_version"): String,

        name("virtualserver_name"): String,
        name_phonetic("virtualserver_name_phonetic"): Option<String>,
        nickname("virtualserver_nickname"): Option<String>,
        welcome_message("virtualserver_welcomemessage"): Option<String>,
        icon_id("virtualserver_icon_id"): i32,
        machine_id("virtualserver_machine_id"): Option<String>,

        status("virtualserver_status"): ServerStatus,
        auto_start("virtualserver_autostart"): bool,
        weblist_enabled("virtualserver_weblist_enabled"): bool,
        ask_for_privilege_key("virtualserver_ask_for_privilegekey"): bool,
        channel_temp_delete_delay_default("virtualserver_channel_temp_delete_delay_default"): i32,
        antiflood_points_needed_plugin_block("virtualserver_antiflood_points_needed_plugin_block"): i32,
        capability_extensions("virtualserver_capability_extensions"): Option<String>,

        password("virtualserver_password"): Option<String>,
        flag_password("virtualserver_flag_password"): bool,

        max_clients("virtualserver_maxclients"): i32,
        clients_online("virtualserver_clientsonline"): i32,
        channels_online("virtualserver_channelsonline"): i32,
        reserved_slots("virtualserver_reserved_slots"): i32,

        created("virtualserver_created"): u64,
        uptime("virtualserver_uptime"): u64,

        needed_identity_security_level("virtualserver_needed_identity_security_level"): i32,
        codec_encryption_mode("virtualserver_codec_encryption_mode"): CodecEncryptionMode,

        filebase("virtualserver_filebase"): Option<String>,
        file_storage_class("virtualserver_file_storage_class"): Option<String>,

        default_server_group("virtualserver_default_server_group"): i32,
        default_channel_group("virtualserver_default_channel_group"): i32,
        default_channel_admin_group("virtualserver_default_channel_admin_group"): i32,

        host_message("virtualserver_hostmessage"): Option<String>,
        host_message_mode("virtualserver_hostmessage_mode"): HostMessageMode,

        host_banner_mode("virtualserver_hostbanner_mode"): HostBannerMode,
        host_banner_url("virtualserver_hostbanner_url"): Option<String>,
        host_banner_gfx_url("virtualserver_hostbanner_gfx_url"): Option<String>,
        host_banner_gfx_interval("virtualserver_hostbanner_gfx_interval"): i32,

        host_button_tooltip("virtualserver_hostbutton_tooltip"): Option<String>,
        host_button_url("virtualserver_hostbutton_url"): Option<String>,
        host_button_gfx_url("virtualserver_hostbutton_gfx_url"): Option<String>,

        complain_autoban_count("virtualserver_complain_autoban_count"): i32,
        complain_autoban_time("virtualserver_complain_autoban_time"): i32,
        complain_remove_time("virtualserver_complain_remove_time"): i32,

        min_clients_in_channel_before_forced_silence("virtualserver_min_clients_in_channel_before_forced_silence"): i32,
        priority_speaker_dimm_modificator("virtualserver_priority_speaker_dimm_modificator"): f32,

        antiflood_points_tick_reduce("virtualserver_antiflood_points_tick_reduce"): i32,
        antiflood_points_needed_command_block("virtualserver_antiflood_points_needed_command_block"): i32,
        antiflood_points_needed_ip_block("virtualserver_antiflood_points_needed_ip_block"): i32,

        log_client("virtualserver_log_client"): bool,
        log_query("virtualserver_log_query"): bool,
        log_channel("virtualserver_log_channel"): bool,
        log_permissions("virtualserver_log_permissions"): bool,
        log_server("virtualserver_log_server"): bool,
        log_file_transfer("virtualserver_log_filetransfer"): bool,

        client_connections("virtualserver_client_connections"): i32,
        query_client_connections("virtualserver_query_client_connections"): i32,

        query_clients_online("virtualserver_queryclientsonline"): i32,

        ping("virtualserver_ping"): i32 = 0,
        total_ping("virtualserver_total_ping"): f32,

        download_quota("virtualserver_download_quota"): i64,
        upload_quota("virtualserver_upload_quota"): i64,

        max_download_total_bandwidth("virtualserver_max_download_total_bandwidth"): i64,
        max_upload_total_bandwidth("virtualserver_max_upload_total_bandwidth"): i64,

        month_bytes_downloaded("virtualserver_month_bytes_downloaded"): i64,
        month_bytes_uploaded("virtualserver_month_bytes_uploaded"): i64,
        total_bytes_downloaded("virtualserver_total_bytes_downloaded"): i64,
        total_bytes_uploaded("virtualserver_total_bytes_uploaded"): i64,

        total_packet_loss_speech("virtualserver_total_packetloss_speech"): f32,
        total_packet_loss_keepalive("virtualserver_total_packetloss_keepalive"): f32,
        total_packet_loss_control("virtualserver_total_packetloss_control"): f32,
        total_packet_loss_total("virtualserver_total_packetloss_total"): f32,

        file_transfer_bandwidth_sent("connection_filetransfer_bandwidth_sent"): i64 = 0,
        file_transfer_bandwidth_received("connection_filetransfer_bandwidth_received"): i64 = 0,
        file_transfer_bytes_sent_total("connection_filetransfer_bytes_sent_total"): i64 = 0,
        file_transfer_bytes_received_total("connection_filetransfer_bytes_received_total"): i64 = 0,

        packets_sent_speech("connection_packets_sent_speech"): i64,
        packets_received_speech("connection_packets_received_speech"): i64,
        packets_sent_keepalive("connection_packets_sent_keepalive"): i64,
        packets_received_keepalive("connection_packets_received_keepalive"): i64,
        packets_sent_control("connection_packets_sent_control"): i64,
        packets_received_control("connection_packets_received_control"): i64,
        packets_sent_total("connection_packets_sent_total"): i64,
        packets_received_total("connection_packets_received_total"): i64,

        bytes_sent_speech("connection_bytes_sent_speech"): i64,
        bytes_received_speech("connection_bytes_received_speech"): i64,
        bytes_sent_keepalive("connection_bytes_sent_keepalive"): i64,
        bytes_received_keepalive("connection_bytes_received_keepalive"): i64,
        bytes_sent_control("connection_bytes_sent_control"): i64,
        bytes_received_control("connection_bytes_received_control"): i64,
        bytes_sent_total("connection_bytes_sent_total"): i64,
        bytes_received_total("connection_bytes_received_total"): i64,

        bandwidth_sent_last_second_total("connection_bandwidth_sent_last_second_total"): i64,
        bandwidth_sent_last_minute_total("connection_bandwidth_sent_last_minute_total"): i64,

        bandwidth_received_last_second_total("connection_bandwidth_received_last_second_total"): i64,
        bandwidth_received_last_minute_total("connection_bandwidth_received_last_minute_total"): i64,
    }
}

// custom decoding

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Badges {
    pub overwolf: bool,
    pub badges: Vec<String>,
}

impl DecodeValue for Badges {
    fn decode(_key: &str, value: String) -> Result<Self, ParseError> {
        let mut overwolf = false;
        let mut badges = Vec::new();

        for part in value.split(':') {
            let mut split = part.split('=');

            let key = split
                .next()
                .ok_or_else(|| ParseError::MissingKey(part.to_owned()))?;
            let value = split
                .next()
                .ok_or_else(|| ParseError::MissingKey(part.to_owned()))?;

            match key {
                "Overwolf" => {
                    overwolf = value == "1";
                }
                "badges" => {
                    badges = value.split(',').map(|v| v.to_owned()).collect();
                }
                _ => {
                    return Err(ParseError::Other(Cow::from(format!(
                        "unknown key: {}",
                        key
                    ))));
                }
            }
        }

        Ok(Self { overwolf, badges })
    }
}
