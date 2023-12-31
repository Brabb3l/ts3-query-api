use crate::definitions::*;
use crate::error::ParseError;
use crate::parser::Decoder;

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
    pub fn from(response: &str) -> Result<Self, ParseError> {
        let mut decoder = Decoder::new(response.as_bytes());
        let name = decoder.decode_name()?;

        Ok(match name.as_str() {
            "notifytextmessage" => Event::TextMessage(decoder.decode()?),
            "notifyclientmoved" => Event::ClientMoved(decoder.decode()?),
            "notifycliententerview" => Event::ClientEnterView(decoder.decode()?),
            "notifyclientleftview" => Event::ClientLeftView(decoder.decode()?),
            "notifychannelcreated" => Event::ChannelCreated(decoder.decode()?),
            "notifychanneldeleted" => Event::ChannelDeleted(decoder.decode()?),
            "notifychanneledited" => Event::ChannelEdited(decoder.decode()?),
            "notifychannelmoved" => Event::ChannelMoved(decoder.decode()?),
            "notifychanneldescriptionchanged" => {
                Event::ChannelDescriptionChanged(decoder.decode()?)
            }
            "notifychannelpasswordchanged" => Event::ChannelPasswordChanged(decoder.decode()?),
            "notifyserveredited" => Event::ServerEdited(decoder.decode()?),
            "notifytokenused" => Event::TokenUsed(decoder.decode()?),
            _ => {
                return Err(ParseError::UnknownEvent {
                    response: response.to_string(),
                    event: name.clone(),
                })
            }
        })
    }
}
