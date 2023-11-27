use async_trait::async_trait;
use log::error;
use crate::error::QueryError;
use crate::macros::ts_response;
use crate::parser::{CommandResponse, Encode};

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle_event(&self, event: Event);

    /// Returns true if the error should cause the connection to be closed, false otherwise
    async fn handle_error(&self, error: QueryError) -> bool {
        error!("Unhandled error: {:?}", error);
        true
    }
}

pub enum EventType {
    Server,
    Channel,
    TextServer,
    TextChannel,
    TextPrivate,
}

impl Encode for EventType {
    fn encode(&self, buf: &mut String) -> std::fmt::Result {
        match self {
            EventType::Server => buf.push_str("server"),
            EventType::Channel => buf.push_str("channel"),
            EventType::TextServer => buf.push_str("textserver"),
            EventType::TextChannel => buf.push_str("textchannel"),
            EventType::TextPrivate => buf.push_str("textprivate"),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Event {
    TextMessage(TextMessageEvent),
    // ClientMoved(ClientMoveEvent),
    // ClientEnterView(ClientEnterViewEvent),
    // ClientLeftView(ClientLeftViewEvent),
    // ChannelCreated(ChannelCreateEvent),
    // ChannelDeleted(ChannelDeleteEvent),
    // ChannelEdited(ChannelEditEvent),
    // ChannelMoved(ChannelMoveEvent),
    // ChannelDescriptionChanged(ChannelDescriptionChangeEvent),
    // ChannelPasswordChanged(ChannelPasswordChangeEvent),
    // ServerEdited(ServerEditEvent),
    // TokenUsed(TokenUseEvent),
}

impl Event {
    pub fn from(mut response: CommandResponse) -> Result<Self, QueryError> {
        let event_name = response.name.as_ref().ok_or_else(|| QueryError::MissingName {
            response: response.to_string()
        })?;

        Ok(match event_name.as_str() {
            "notifytextmessage" => Event::TextMessage(TextMessageEvent::from(&mut response)?),
            // "notifyclientmoved" => Event::ClientMoved(ClientMoveEvent::from(&mut response)?),
            // "notifycliententerview" => Event::ClientEnterView(ClientEnterViewEvent::from(&mut response)?),
            // "notifyclientleftview" => Event::ClientLeftView(ClientLeftViewEvent::from(&mut response)?),
            // "notifychannelcreated" => Event::ChannelCreated(ChannelCreateEvent::from(&mut response)?),
            // "notifychanneldeleted" => Event::ChannelDeleted(ChannelDeleteEvent::from(&mut response)?),
            // "notifychanneledited" => Event::ChannelEdited(ChannelEditEvent::from(&mut response)?),
            // "notifychannelmoved" => Event::ChannelMoved(ChannelMoveEvent::from(&mut response)?),
            // "notifychanneldescriptionchanged" => Event::ChannelDescriptionChanged(ChannelDescriptionChangeEvent::from(&mut response)?),
            // "notifychannelpasswordchanged" => Event::ChannelPasswordChanged(ChannelPasswordChangeEvent::from(&mut response)?),
            // "notifyserveredited" => Event::ServerEdited(ServerEditEvent::from(&mut response)?),
            // "notifytokenused" => Event::TokenUsed(TokenUseEvent::from(&mut response)?),
            _ => return Err(QueryError::UnknownEvent {
                response: response.to_string(),
                event: event_name.clone()
            })
        })
    }
}

#[derive(Default)]
pub struct DefaultEventHandler;

#[async_trait]
impl EventHandler for DefaultEventHandler {
    async fn handle_event(&self, _event: Event) {}
}

ts_response! {
    TextMessageEvent {
        invoker_id("invokerid"): i32,
        invoker_name("invokername"): String,
        invoker_uid("invokeruid"): String,
        target_mode("targetmode"): i32,
        message("msg"): String,
    }
}
