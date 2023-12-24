use crate::parser::Encode;
use super::*;

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

