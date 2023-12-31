use super::*;
use crate::definitions::builder::{ChannelListFlags, ClientListFlags};
use crate::error::ParseError;
use crate::parser::{Decode, Decoder, Encode};

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

impl ClientListDynamicEntry {
    pub fn decode(decoder: &mut Decoder, flags: &ClientListFlags) -> Result<Self, ParseError> {
        let base = ClientListEntry::decode(decoder)?;

        let uid = if flags.uid {
            Some(decoder.decode()?)
        } else {
            None
        };
        let away = if flags.away {
            Some(decoder.decode()?)
        } else {
            None
        };
        let voice = if flags.voice {
            Some(decoder.decode()?)
        } else {
            None
        };
        let times = if flags.times {
            Some(decoder.decode()?)
        } else {
            None
        };
        let groups = if flags.groups {
            Some(decoder.decode()?)
        } else {
            None
        };
        let info = if flags.info {
            Some(decoder.decode()?)
        } else {
            None
        };
        let country = if flags.country {
            Some(decoder.decode()?)
        } else {
            None
        };
        let ip = if flags.ip {
            Some(decoder.decode()?)
        } else {
            None
        };
        let icon = if flags.icon {
            Some(decoder.decode()?)
        } else {
            None
        };
        let badges = if flags.badges {
            Some(decoder.decode()?)
        } else {
            None
        };

        Ok(Self {
            base,
            uid,
            away,
            voice,
            times,
            groups,
            info,
            country,
            ip,
            icon,
            badges,
        })
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

impl ChannelListDynamicEntry {
    pub fn decode(decoder: &mut Decoder, flags: &ChannelListFlags) -> Result<Self, ParseError> {
        let base = ChannelListEntry::decode(decoder)?;

        let topic = if flags.topic {
            Some(decoder.decode()?)
        } else {
            None
        };
        let flags2 = if flags.flags {
            Some(decoder.decode()?)
        } else {
            None
        };
        let voice = if flags.voice {
            Some(decoder.decode()?)
        } else {
            None
        };
        let limits = if flags.limits {
            Some(decoder.decode()?)
        } else {
            None
        };
        let icon = if flags.icon {
            Some(decoder.decode()?)
        } else {
            None
        };
        let seconds_empty = if flags.seconds_empty {
            Some(decoder.decode()?)
        } else {
            None
        };
        let banners = if flags.banners {
            Some(decoder.decode()?)
        } else {
            None
        };

        Ok(Self {
            base,
            topic,
            flags: flags2,
            voice,
            limits,
            icon,
            seconds_empty,
            banners,
        })
    }
}

#[derive(Debug)]
pub enum EventType {
    Server,
    Channel,
    TextServer,
    TextChannel,
    TextPrivate,
    TokenUsed,
}

impl Encode for EventType {
    fn encode(&self, buf: &mut String) -> std::fmt::Result {
        match self {
            EventType::Server => buf.push_str("server"),
            EventType::Channel => buf.push_str("channel"),
            EventType::TextServer => buf.push_str("textserver"),
            EventType::TextChannel => buf.push_str("textchannel"),
            EventType::TextPrivate => buf.push_str("textprivate"),
            EventType::TokenUsed => buf.push_str("tokenused"),
        }

        Ok(())
    }
}
