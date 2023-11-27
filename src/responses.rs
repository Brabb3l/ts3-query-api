use std::str::FromStr;
use crate::error::QueryError;
use crate::macros::ts_response;

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
        client_id: i32,
        client_channel_id: i32,
        client_nickname: String,
        client_database_id: i32,
        client_login_name: String,
        client_unique_identifier: String,
        client_origin_server_id: i32
    }
}

// channel list

ts_response! {
    ChannelListEntry {
        cid: i32,
        pid: i32,
        channel_order: i32,
        channel_name: String,
        total_clients: i32,
        channel_needed_subscribe_power: i32
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
        channel_codec: String,
        channel_codec_quality: i32,
        channel_needed_talk_power: i32
    }
}

ts_response! {
    ChannelListLimitsEntry {
        total_clients_family: i32,
        channel_maxclients: i32,
        channel_maxfamilyclients: i32
    }
}

ts_response! {
    ChannelListIconEntry {
        channel_icon_id: i32
    }
}

ts_response! {
    ChannelListSecondsEmptyEntry {
        seconds_empty: i32
    }
}

ts_response! {
    ChannelListBannerEntry {
        channel_banner_gfx_url: String,
        channel_banner_mode: i32
    }
}

#[derive(Debug)]
pub struct ChannelListDynamicEntry {
    pub base: ChannelListEntry,
    pub topic: Option<ChannelListTopicEntry>,
    pub flags: Option<ChannelListFlagsEntry>,
    pub voice: Option<ChannelListVoiceEntry>,
    pub limits: Option<ChannelListLimitsEntry>,
    pub icon: Option<ChannelListIconEntry>,
    pub seconds_empty: Option<ChannelListSecondsEmptyEntry>,
    pub banners: Option<ChannelListBannerEntry>,
}

// channel info

ts_response! {
    ChannelInfo {
        pid: i32,

        // property fields
        channel_topic: String,
        channel_description: String,
        channel_password: String,
        channel_codec: String,
        channel_codec_quality: i32,
        channel_maxclients: i32,
        channel_maxfamilyclients: i32,
        channel_order: i32,
        channel_flag_permanent: bool,
        channel_flag_semi_permanent: bool,
        channel_flag_default: bool,
        channel_flag_maxclients_unlimited: bool,
        channel_flag_maxfamilyclients_unlimited: bool,
        channel_flag_maxfamilyclients_inherited: bool,
        channel_needed_talk_power: i32,
        channel_name_phonetic: String,
        channel_forced_silence: bool,
        channel_icon_id: i32,
        channel_codec_is_unencrypted: bool,
        channel_banner_gfx_url: String,
        channel_banner_mode: i32,
        channel_delete_delay: i32,

        // non-property fields
        channel_name: String,
        channel_flag_password: bool,
        channel_filepath: String,
        channel_unique_identifier: String,
        seconds_empty: i32,
        channel_security_salt: String,
        channel_codec_latency_factor: i32
    }
}

// client list

ts_response! {
    ClientListEntry {
        clid: i32,
        cid: i32,
        client_database_id: i32,
        client_nickname: String,
        client_type: bool
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
        client_talk_power: i32,
        client_is_talker: bool,
        client_is_priority_speaker: bool,
        client_is_recording: bool,
        client_is_channel_commander: bool
    }
}

ts_response! {
    ClientListTimesEntry {
        client_idle_time: i32,
        client_created: i32,
        client_lastconnected: i32
    }
}

ts_response! {
    ClientListGroupsEntry {
        client_servergroups: Vec<i32>,
        client_channel_group_id: i32,
        client_channel_group_inherited_channel_id: i32
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
        client_icon_id: i32
    }
}

ts_response! {
    ClientListBadgesEntry {
        client_badges: Badges
    }
}

#[derive(Debug)]
pub struct Badges {
    pub overwolf: bool,
    pub badges: Vec<String>
}

impl FromStr for Badges {
    type Err = QueryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut overwolf = false;
        let mut badges = Vec::new();

        if s.is_empty() {
            return Ok(Self {
                overwolf,
                badges
            });
        }

        for part in s.split(':') {
            let mut split = part.split('=');

            let key = split.next().ok_or(QueryError::MissingKey {
                response: s.to_owned(),
                key: part.to_owned()
            })?;

            let value = split.next().ok_or(QueryError::MissingKey {
                response: s.to_owned(),
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
                        response: s.to_owned(),
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

#[derive(Debug)]
pub struct ClientListDynamicEntry {
    pub base: ClientListEntry,
    pub uid: Option<ClientListUidEntry>,
    pub away: Option<ClientListAwayEntry>,
    pub voice: Option<ClientListVoiceEntry>,
    pub times: Option<ClientListTimesEntry>,
    pub groups: Option<ClientListGroupsEntry>,
    pub info: Option<ClientListInfoEntry>,
    pub country: Option<ClientListCountryEntry>,
    pub ip: Option<ClientListIpEntry>,
    pub icon: Option<ClientListIconEntry>,
    pub badges: Option<ClientListBadgesEntry>,
}

// client info

ts_response! {
    ClientInfo {
        cid: i32,
        client_idle_time: i32,
        client_unique_identifier: String,
        client_nickname: String,
        client_version: String,
        client_platform: String,
        client_input_muted: bool,
        client_output_muted: bool,
        client_outputonly_muted: bool,
        client_input_hardware: bool,
        client_output_hardware: bool,
        client_default_channel: String,
        client_meta_data: String,
        client_is_recording: bool,
        client_version_sign: String,
        client_security_hash: String,
        client_login_name: String,
        client_database_id: i32,
        client_channel_group_id: i32,
        client_servergroups: Vec<i32>,
        client_created: i32,
        client_lastconnected: i32,
        client_totalconnections: i32,
        client_away: bool,
        client_away_message: String,
        client_type: bool,
        client_flag_avatar: String,
        client_talk_power: i32,
        client_talk_request: i32,
        client_talk_request_msg: String,
        client_description: String,
        client_is_talker: bool,
        client_month_bytes_uploaded: i32,
        client_month_bytes_downloaded: i32,
        client_total_bytes_uploaded: i32,
        client_total_bytes_downloaded: i32,
        client_is_priority_speaker: bool,
        client_nickname_phonetic: String,
        client_needed_serverquery_view_power: i32,
        client_default_token: String,
        client_icon_id: i32,
        client_is_channel_commander: bool,
        client_country: String,
        client_channel_group_inherited_channel_id: i32,
        client_badges: Badges,
        client_myteamspeak_id: String,
        client_integrations: String,
        client_myteamspeak_avatar: String,
        client_signed_badges: Vec<String>,
        client_base64HashClientUID: String,
        connection_filetransfer_bandwidth_sent: i32,
        connection_filetransfer_bandwidth_received: i32,
        connection_packets_sent_total: i32,
        connection_bytes_sent_total: i32,
        connection_packets_received_total: i32,
        connection_bytes_received_total: i32,
        connection_bandwidth_sent_last_second_total: i32,
        connection_bandwidth_sent_last_minute_total: i32,
        connection_bandwidth_received_last_second_total: i32,
        connection_bandwidth_received_last_minute_total: i32,
        connection_connected_time: i32,
        connection_client_ip: String
    }
}