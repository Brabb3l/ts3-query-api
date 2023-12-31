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
        codec_quality("channel_codec_quality"): u8,
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
        codec_quality("channel_codec_quality"): u8,
        codec_latency_factor("channel_codec_latency_factor"): i32,
        codec_is_unencrypted("channel_codec_is_unencrypted"): bool,

        max_clients("channel_maxclients"): i32,
        max_family_clients("channel_maxfamilyclients"): i32,
        order("channel_order"): i32,

        flag_permanent("channel_flag_permanent"): bool,
        flag_semi_permanent("channel_flag_semi_permanent"): bool,
        flag_default("channel_flag_default"): bool,
        flag_maxclients_unlimited("channel_flag_maxclients_unlimited"): bool,
        flag_maxfamilyclients_unlimited("channel_flag_maxfamilyclients_unlimited"): bool,
        flag_maxfamilyclients_inherited("channel_flag_maxfamilyclients_inherited"): bool,
        flag_password("channel_flag_password"): bool,

        needed_talk_power("channel_needed_talk_power"): i32,
        forced_silence("channel_forced_silence"): bool,

        filepath("channel_filepath"): String,
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
        myteamspeak_id("client_myteamspeak_id"): Option<String>,
        myteamspeak_avatar("client_myteamspeak_avatar"): Option<String>,
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

        file_transfer_bandwidth_sent("connection_filetransfer_bandwidth_sent"): i32,
        file_transfer_bandwidth_received("connection_filetransfer_bandwidth_received"): i32,
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
