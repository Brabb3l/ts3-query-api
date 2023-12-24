use crate::error::QueryError;
use crate::macros::ts_response;
use crate::parser::{CommandResponse, Encode};
use crate::QueryClient;
use async_trait::async_trait;
use log::error;

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle_event(&self, client: QueryClient, event: Event);

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
    TextMessage(Box<TextMessageEvent>),
    ClientMoved(Box<ClientMoveEvent>),
    ClientEnterView(Box<ClientEnterViewEvent>),
    ClientLeftView(Box<ClientLeftViewEvent>),
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
        let event_name = response
            .name
            .as_ref()
            .ok_or_else(|| QueryError::MissingName {
                response: response.to_string(),
            })?;

        Ok(match event_name.as_str() {
            "notifytextmessage" => {
                Event::TextMessage(Box::new(TextMessageEvent::from(&mut response)?))
            }
            "notifyclientmoved" => {
                Event::ClientMoved(Box::new(ClientMoveEvent::from(&mut response)?))
            }
            "notifycliententerview" => {
                Event::ClientEnterView(Box::new(ClientEnterViewEvent::from(&mut response)?))
            }
            "notifyclientleftview" => {
                Event::ClientLeftView(Box::new(ClientLeftViewEvent::from(&mut response)?))
            }
            // "notifychannelcreated" => {
            //     Event::ChannelCreated(Box::new(ChannelCreateEvent::from(&mut response)?))
            // }
            // "notifychanneldeleted" => {
            //     Event::ChannelDeleted(Box::new(ChannelDeleteEvent::from(&mut response)?))
            // }
            // "notifychanneledited" => {
            //     Event::ChannelEdited(Box::new(ChannelEditEvent::from(&mut response)?))
            // }
            // "notifychannelmoved" => {
            //     Event::ChannelMoved(Box::new(ChannelMoveEvent::from(&mut response)?))
            // }
            // "notifychanneldescriptionchanged" => Event::ChannelDescriptionChanged(Box::new(
            //     ChannelDescriptionChangeEvent::from(&mut response)?,
            // )),
            // "notifychannelpasswordchanged" => Event::ChannelPasswordChanged(Box::new(
            //     ChannelPasswordChangeEvent::from(&mut response)?,
            // )),
            // "notifyserveredited" => {
            //     Event::ServerEdited(Box::new(ServerEditEvent::from(&mut response)?))
            // }
            // "notifytokenused" => Event::TokenUsed(Box::new(TokenUseEvent::from(&mut response)?)),
            _ => {
                println!("Unknown event: {}, {}", event_name, response.to_string());
                return Err(QueryError::UnknownEvent {
                    response: response.to_string(),
                    event: event_name.clone(),
                });
            }
        })
    }
}

#[derive(Default)]
pub struct DefaultEventHandler;

#[async_trait]
impl EventHandler for DefaultEventHandler {
    async fn handle_event(&self, _client: QueryClient, _event: Event) {}
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

ts_response! {
    ClientMoveEvent {
        client_id("clid"): i32,
        channel_from_id("cfid"): i32,
        channel_to_id("ctid"): i32,
        reason_id("reasonid"): i32,
    }
}

ts_response! {
    ClientLeftViewEvent {
        channel_to_id("ctid"): i32,
        invoker_name("invokername"): String,
        invoker_id("invokerid"): i32,
        channel_from_id("cfid"): i32,
        reason_msg("reasonmsg"): String,
        invoker_uid("invokeruid"): String,
        reason_id("reasonid"): i32,
        client_id("clid"): i32,
    }
}

// TODO: I'm unsure about some of these types,
//       perhaps some of them should be enums?
ts_response! {
    ClientEnterViewEvent {
        client_unread_messages("client_unread_messages"): i32,
        client_output_muted("client_output_muted"): i32,
        client_talk_request("client_talk_request"): i32,
        client_description("client_description"): String,
        client_icon_id("client_icon_id"): i32,
        client_is_talker("client_is_talker"): bool,
        client_badges("client_badges"): String,
        client_id("clid"): i32,
        client_myteamspeak_id("client_myteamspeak_id"): String,
        client_flag_avatar("client_flag_avatar"): String,
        client_talk_power("client_talk_power"): i32,
        client_input_muted("client_input_muted"): bool,
        client_output_hardware("client_output_hardware"): i32,
        client_type("client_type"): i32,
        client_input_hardware("client_input_hardware"): i32,
        client_servergroups("client_servergroups"): String,
        client_needed_serverquery_view_power("client_needed_serverquery_view_power"): i32,
        client_myteamspeak_avatar("client_myteamspeak_avatar"): String,
        client_signed_badges("client_signed_badges"): String,
        client_outputonly_muted("client_outputonly_muted"): bool,
        channel_from("cfid"): i32,
        client_meta_data("client_meta_data"): String,
        reasonid("reasonid"): i32,
        client_nickname("client_nickname"): String,
        client_channel_group_inherited_channel_id("client_channel_group_inherited_channel_id"): i32,
        client_user_tag("client_user_tag"): String,
        client_unique_identifier("client_unique_identifier"): String,
        client_is_channel_commander("client_is_channel_commander"): bool,
        client_country("client_country"): String,
        channel_to_id("ctid"): i32,
        client_is_priority_speaker("client_is_priority_speaker"): bool,
        client_integrations("client_integrations"): String,
        client_away("client_away"): bool,
        client_talk_request_msg("client_talk_request_msg"): String,
        client_nickname_phonetic("client_nickname_phonetic"): String,
        client_is_recording("client_is_recording"): bool,
        client_database_id("client_database_id"): i32,
        client_channel_group_id("client_channel_group_id"): i32,
        client_away_message("client_away_message"): String,
    }
}
