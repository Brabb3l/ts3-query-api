use crate::definitions::*;
use crate::error::ParseError;
use crate::parser::CommandResponse;

#[derive(Debug)]
pub enum Event {
    TextMessage(TextMessageEvent),
    ClientMoved(ClientMoveEvent),
    ClientEnterView(ClientEnterViewEvent),
    ClientLeftView(ClientLeftViewEvent),
    ChannelCreated(ChannelCreateEvent),
    ChannelDeleted(ChannelDeleteEvent),
    ChannelEdited(ChannelEditEvent),
    ChannelMoved(ChannelMoveEvent),
    ChannelDescriptionChanged(ChannelDescriptionChangeEvent),
    ChannelPasswordChanged(ChannelPasswordChangeEvent),
    ServerEdited(ServerEditEvent),
    TokenUsed(TokenUseEvent),
}

impl Event {
    pub fn from(mut response: CommandResponse) -> Result<Self, ParseError> {
        let event_name = response.name.as_ref().ok_or_else(|| ParseError::MissingName {
            response: response.to_string()
        })?;

        Ok(match event_name.as_str() {
            "notifytextmessage" => Event::TextMessage(TextMessageEvent::from(&mut response)?),
            "notifyclientmoved" => Event::ClientMoved(ClientMoveEvent::from(&mut response)?),
            "notifycliententerview" => Event::ClientEnterView(ClientEnterViewEvent::from(&mut response)?),
            "notifyclientleftview" => Event::ClientLeftView(ClientLeftViewEvent::from(&mut response)?),
            "notifychannelcreated" => Event::ChannelCreated(ChannelCreateEvent::from(&mut response)?),
            "notifychanneldeleted" => Event::ChannelDeleted(ChannelDeleteEvent::from(&mut response)?),
            "notifychanneledited" => Event::ChannelEdited(ChannelEditEvent::from(&mut response)?),
            "notifychannelmoved" => Event::ChannelMoved(ChannelMoveEvent::from(&mut response)?),
            "notifychanneldescriptionchanged" => Event::ChannelDescriptionChanged(ChannelDescriptionChangeEvent::from(&mut response)?),
            "notifychannelpasswordchanged" => Event::ChannelPasswordChanged(ChannelPasswordChangeEvent::from(&mut response)?),
            "notifyserveredited" => Event::ServerEdited(ServerEditEvent::from(&mut response)?),
            "notifytokenused" => Event::TokenUsed(TokenUseEvent::from(&mut response)?),
            _ => return Err(ParseError::UnknownEvent {
                response: response.to_string(),
                event: event_name.clone()
            })
        })
    }
}
