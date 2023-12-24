use crate::event::Event;
use crate::parser::CommandResponse;

pub(super) struct RawCommandRequest {
    pub data: String,
    pub response_tx: flume::Sender<RawCommandResponse>
}

pub(super) struct RawCommandResponse {
    pub status: CommandResponse,
    pub content: String,
}

pub(super) enum QueryResponse {
    Response(RawCommandResponse),
    Event(Event)
}

