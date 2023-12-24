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
        client_id: u32,
        client_channel_id: u32,
        client_nickname: String,
        client_database_id: u32,
        client_login_name: String,
        client_unique_identifier: String,
        client_origin_server_id: u32
    }
}

// channel list

ts_response! {
    ChannelListEntry {
        channel_id("cid"): u32,
        parent_id("pid"): u32,

        channel_name: String,
        channel_order: u32,
        total_clients: u32,
        channel_needed_subscribe_power: u32,
    }
}

ts_response! {
    ChannelListTopicEntry {
        channel_topic: String
    }
}

ts_response! {
    ChannelListFlagsEntry {
        channel_flag_default: bool,
        channel_flag_password: bool,
        channel_flag_permanent: bool,
        channel_flag_semi_permanent: bool
    }
}

ts_response! {
    ChannelListVoiceEntry {
        channel_codec: Codec = Codec::OpusVoice,
        channel_codec_quality: u8,
        channel_needed_talk_power: u32
    }
}

ts_response! {
    ChannelListLimitsEntry {
        total_clients_family: u32,
        channel_maxclients: i32,
        channel_maxfamilyclients: i32
    }
}

ts_response! {
    ChannelListIconEntry {
        channel_icon_id: u32
    }
}

ts_response! {
    ChannelListSecondsEmptyEntry {
        seconds_empty: u32
    }
}

ts_response! {
    ChannelListBannerEntry {
        channel_banner_gfx_url: String,
        channel_banner_mode: u32
    }
}

// channel info

ts_response! {
    ChannelInfo {
        parent_id("pid"): u32,

        // property fields
        channel_topic: String,
        channel_description: String,
        channel_password: String,
        channel_codec: Codec = Codec::OpusVoice,
        channel_codec_quality: u8,
        channel_maxclients: i32,
        channel_maxfamilyclients: i32,
        channel_order: u32,
        channel_flag_permanent: bool,
        channel_flag_semi_permanent: bool,
        channel_flag_default: bool,
        channel_flag_maxclients_unlimited: bool,
        channel_flag_maxfamilyclients_unlimited: bool,
        channel_flag_maxfamilyclients_inherited: bool,
        channel_needed_talk_power: u32,
        channel_name_phonetic: String,
        channel_forced_silence: bool,
        channel_icon_id: u32,
        channel_codec_is_unencrypted: bool,
        channel_banner_gfx_url: String,
        channel_banner_mode: u32,
        channel_delete_delay: u32,

        // non-property fields
        channel_name: String,
        channel_flag_password: bool,
        channel_filepath: String,
        channel_unique_identifier: String,
        seconds_empty: u32,
        channel_security_salt: String,
        channel_codec_latency_factor: u32
    }
}

// client list

ts_response! {
    ClientListEntry {
        client_id("clid"): u32,
        channel_id("cid"): u32,
        client_database_id: u32,
        client_nickname: String,
        is_query("client_type"): bool
    }
}

ts_response! {
    ClientListUidEntry {
        client_unique_identifier: String
    }
}

ts_response! {
    ClientListAwayEntry {
        client_away: bool,
        client_away_message: String
    }
}

ts_response! {
    ClientListVoiceEntry {
        client_flag_talking: bool,
        client_input_muted: bool,
        client_output_muted: bool,
        client_input_hardware: bool,
        client_output_hardware: bool,
        client_talk_power: u32,
        client_is_talker: bool,
        client_is_priority_speaker: bool,
        client_is_channel_commander: bool,
        client_is_recording: bool,
    }
}

ts_response! {
    ClientListTimesEntry {
        client_idle_time: u32,
        client_created: u32,
        client_lastconnected: u32
    }
}

ts_response! {
    ClientListGroupsEntry {
        client_servergroups: Vec<u32>,
        client_channel_group_id: u32,
        client_channel_group_inherited_channel_id: u32
    }
}

ts_response! {
    ClientListInfoEntry {
        client_version: String,
        client_platform: String
    }
}

ts_response! {
    ClientListCountryEntry {
        client_country: String
    }
}

ts_response! {
    ClientListIpEntry {
        connection_client_ip: String
    }
}

ts_response! {
    ClientListIconEntry {
        client_icon_id: u32
    }
}

ts_response! {
    ClientListBadgesEntry {
        client_badges: Badges
    }
}

// client info

ts_response! {
    ClientInfo {
        client_id("clid"): u32,

        client_nickname: String,
        client_unique_identifier: String,
        client_database_id: u32,

        channel_id("cid"): u32,

        client_version: String,
        client_platform: String,
        client_base64_hash_client_uid("client_base64HashClientUID"): String,

        client_login_name: String,
        client_nickname_phonetic: Option<String>,
        client_description: Option<String>,
        client_icon_id: u32,
        client_country: Option<String>,
        client_badges: Badges,
        client_signed_badges: Vec<String>,
        client_myteamspeak_id: Option<String>,
        client_myteamspeak_avatar: Option<String>,
        client_integrations: Option<String>,

        client_idle_time: u32,
        client_away: bool,
        client_away_message: String,

        client_default_channel: String,
        client_meta_data: Option<String>,
        client_version_sign: String,
        client_security_hash: String,
        client_unread_messages: u32,

        client_channel_group_id: u32,
        client_servergroups: Vec<u32>,

        client_created: u32,
        client_lastconnected: u32,
        client_totalconnections: u32,

        is_query("client_type"): bool,

        client_input_muted: bool,
        client_output_muted: bool,
        client_outputonly_muted: bool,
        client_input_hardware: bool,
        client_output_hardware: bool,
        client_flag_avatar: Option<String>,
        client_talk_power: u32,
        client_talk_request: u32,
        client_talk_request_msg: Option<String>,
        client_is_talker: bool,
        client_is_priority_speaker: bool,
        client_is_channel_commander: bool,
        client_is_recording: bool,

        client_month_bytes_uploaded: u32,
        client_month_bytes_downloaded: u32,
        client_total_bytes_uploaded: u32,
        client_total_bytes_downloaded: u32,

        client_needed_serverquery_view_power: u32,
        client_channel_group_inherited_channel_id: u32,
        client_default_token: String,

        connection_filetransfer_bandwidth_sent: u32,
        connection_filetransfer_bandwidth_received: u32,
        connection_packets_sent_total: u32,
        connection_bytes_sent_total: u32,
        connection_packets_received_total: u32,
        connection_bytes_received_total: u32,
        connection_bandwidth_sent_last_second_total: u32,
        connection_bandwidth_sent_last_minute_total: u32,
        connection_bandwidth_received_last_second_total: u32,
        connection_bandwidth_received_last_minute_total: u32,
        connection_connected_time: u32,
        connection_client_ip: String
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
