use crate::error::QueryError;
use crate::macros::ts_response;
use crate::parser::Decode;
use super::*;

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
        virtualserver_id: u32,
        virtualserver_unique_identifier: String,
        virtualserver_port: u32,

        id("client_id"): u32,
        database_id("client_database_id"): u32,
        unique_identifier("client_unique_identifier"): String,

        nickname("client_nickname"): String,
        login_name("client_login_name"): String,
        channel_id("client_channel_id"): u32,
        origin_server_id("client_origin_server_id"): u32,
    }
}

// channel list

ts_response! {
    ChannelListEntry {
        id("cid"): u32,
        parent_id("pid"): u32,

        name("channel_name"): String,
        order("channel_order"): u32,
        total_clients: u32,
        needed_subscribe_power("channel_needed_subscribe_power"): u32,
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
        needed_talk_power("channel_needed_talk_power"): u32
    }
}

ts_response! {
    ChannelListLimitsEntry {
        total_clients_family: u32,
        max_clients("channel_maxclients"): i32,
        max_family_clients("channel_maxfamilyclients"): i32
    }
}

ts_response! {
    ChannelListIconEntry {
        icon_id("channel_icon_id"): u32
    }
}

ts_response! {
    ChannelListSecondsEmptyEntry {
        seconds_empty: u32
    }
}

ts_response! {
    ChannelListBannerEntry {
        banner_gfx_url("channel_banner_gfx_url"): String,
        banner_mode("channel_banner_mode"): u32
    }
}

// channel info

ts_response! {
    ChannelInfo {
        parent_id("pid"): u32,

        name("channel_name"): String,
        unique_identifier("channel_unique_identifier"): String,

        name_phonetic("channel_name_phonetic"): Option<String>,
        topic("channel_topic"): Option<String>,
        description("channel_description"): Option<String>,
        password("channel_password"): Option<String>,
        icon_id("channel_icon_id"): u32,
        banner_gfx_url("channel_banner_gfx_url"): Option<String>,
        banner_mode("channel_banner_mode"): u32,

        codec("channel_codec"): Codec = Codec::OpusVoice,
        codec_quality("channel_codec_quality"): u8,
        codec_latency_factor("channel_codec_latency_factor"): u32,
        codec_is_unencrypted("channel_codec_is_unencrypted"): bool,

        max_clients("channel_maxclients"): i32,
        max_family_clients("channel_maxfamilyclients"): i32,
        order("channel_order"): u32,

        flag_permanent("channel_flag_permanent"): bool,
        flag_semi_permanent("channel_flag_semi_permanent"): bool,
        flag_default("channel_flag_default"): bool,
        flag_maxclients_unlimited("channel_flag_maxclients_unlimited"): bool,
        flag_maxfamilyclients_unlimited("channel_flag_maxfamilyclients_unlimited"): bool,
        flag_maxfamilyclients_inherited("channel_flag_maxfamilyclients_inherited"): bool,
        flag_password("channel_flag_password"): bool,

        needed_talk_power("channel_needed_talk_power"): u32,
        forced_silence("channel_forced_silence"): bool,

        filepath("channel_filepath"): String,
        seconds_empty: u32,
        delete_delay("channel_delete_delay"): u32,
        security_salt("channel_security_salt"): String,
    }
}

// client list

ts_response! {
    ClientListEntry {
        id("clid"): u32,
        channel_id("cid"): u32,
        database_id("client_database_id"): u32,
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
        away_message("client_away_message"): String
    }
}

ts_response! {
    ClientListVoiceEntry {
        flag_talking("client_flag_talking"): bool,
        input_muted("client_input_muted"): bool,
        output_muted("client_output_muted"): bool,
        input_hardware("client_input_hardware"): bool,
        output_hardware("client_output_hardware"): bool,
        talk_power("client_talk_power"): u32,
        is_talker("client_is_talker"): bool,
        is_priority_speaker("client_is_priority_speaker"): bool,
        is_channel_commander("client_is_channel_commander"): bool,
        is_recording("client_is_recording"): bool,
    }
}

ts_response! {
    ClientListTimesEntry {
        idle_time("client_idle_time"): u32,
        created("client_created"): u32,
        last_connected("client_lastconnected"): u32,
    }
}

ts_response! {
    ClientListGroupsEntry {
        server_groups("client_servergroups"): Vec<u32>,
        channel_group_id("client_channel_group_id"): u32,
        channel_group_inherited_channel_id("client_channel_group_inherited_channel_id"): u32
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
        country("client_country"): String
    }
}

ts_response! {
    ClientListIpEntry {
        client_ip("connection_client_ip"): String
    }
}

ts_response! {
    ClientListIconEntry {
        icon_id("client_icon_id"): u32
    }
}

ts_response! {
    ClientListBadgesEntry {
        badges("client_badges"): Badges
    }
}

// client info

ts_response! {
    ClientInfo {
        id("clid"): u32,

        nickname("client_nickname"): String,
        unique_identifier("client_unique_identifier"): String,
        database_id("client_database_id"): u32,

        channel_id("cid"): u32,

        version("client_version"): String,
        platform("client_platform"): String,
        base64_hash_client_uid("client_base64HashClientUID"): String,

        login_name("client_login_name"): String,
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

        idle_time("client_idle_time"): u32,
        away("client_away"): bool,
        away_message("client_away_message"): Option<String>,

        default_channel("client_default_channel"): String,
        meta_data("client_meta_data"): Option<String>,
        version_sign("client_version_sign"): String,
        security_hash("client_security_hash"): String,
        unread_messages("client_unread_messages"): u32,

        channel_group_id("client_channel_group_id"): u32,
        server_groups("client_servergroups"): Vec<u32>,

        created("client_created"): u32,
        last_connected("client_lastconnected"): u32,
        total_connections("client_totalconnections"): u32,

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

        month_bytes_uploaded("client_month_bytes_uploaded"): u32,
        month_bytes_downloaded("client_month_bytes_downloaded"): u32,
        total_bytes_uploaded("client_total_bytes_uploaded"): u32,
        total_bytes_downloaded("client_total_bytes_downloaded"): u32,

        needed_serverquery_view_power("client_needed_serverquery_view_power"): u32,
        channel_group_inherited_channel_id("client_channel_group_inherited_channel_id"): u32,
        default_token("client_default_token"): String,

        file_transfer_bandwidth_sent("connection_filetransfer_bandwidth_sent"): u32,
        file_transfer_bandwidth_received("connection_filetransfer_bandwidth_received"): u32,
        packets_sent_total("connection_packets_sent_total"): u32,
        bytes_sent_total("connection_bytes_sent_total"): u32,
        packets_received_total("connection_packets_received_total"): u32,
        bytes_received_total("connection_bytes_received_total"): u32,
        bandwidth_sent_last_second_total("connection_bandwidth_sent_last_second_total"): u32,
        bandwidth_sent_last_minute_total("connection_bandwidth_sent_last_minute_total"): u32,
        bandwidth_received_last_second_total("connection_bandwidth_received_last_second_total"): u32,
        bandwidth_received_last_minute_total("connection_bandwidth_received_last_minute_total"): u32,
        connected_time("connection_connected_time"): u32,
        client_ip("connection_client_ip"): String,
    }
}

// custom decoding

#[derive(Debug, Default)]
pub struct Badges {
    pub overwolf: bool,
    pub badges: Vec<String>
}

impl Decode for Badges {
    fn decode(_key: &str, value: String) -> Result<Self, QueryError> {
        let mut overwolf = false;
        let mut badges = Vec::new();

        if value.is_empty() {
            return Ok(Self {
                overwolf,
                badges
            });
        }

        for part in value.split(':') {
            let mut split = part.split('=');

            let key = split.next().ok_or(QueryError::MissingKey {
                response: value.to_owned(),
                key: part.to_owned()
            })?;

            let value = split.next().ok_or(QueryError::MissingKey {
                response: value.to_owned(),
                key: part.to_owned()
            })?;

            match key {
                "Overwolf" => {
                    overwolf = value == "1";
                },
                "badges" => {
                    badges = value.split(',').map(|v| v.to_owned()).collect();
                },
                _ => {
                    return Err(QueryError::UnknownKey {
                        response: value.to_owned(),
                        key: key.to_owned()
                    });
                }
            }
        }

        Ok(Self {
            overwolf,
            badges
        })
    }
}
