use bytes::Bytes;
use std::cmp::max;

pub(super) struct RawCommandRequest {
    pub data: String,
    pub response_tx: flume::Sender<RawCommandResponse>,
}

pub struct RawCommandResponse {
    pub response: Bytes,
    pub mid_index: usize,
}

impl RawCommandResponse {
    pub fn status(&self) -> &[u8] {
        let (_, status) = self.response.split_at(self.mid_index);

        &status[..max(2, status.len()) - 2]
    }

    pub fn content(&self) -> &[u8] {
        let (content, _) = self.response.split_at(self.mid_index);

        &content[..max(2, content.len()) - 2]
    }
}
