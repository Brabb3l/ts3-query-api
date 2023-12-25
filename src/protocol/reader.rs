use std::cmp::max;
use bytes::BytesMut;
use log::{debug, error};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;
use crate::error::QueryError;
use crate::event::Event;
use crate::parser::CommandResponse;
use crate::protocol::types::{RawCommandResponse, QueryResponse};

pub(super) struct Reader {
    reader: BufReader<OwnedReadHalf>,
    response_tx: flume::Sender<RawCommandResponse>,
    event_tx: flume::Sender<Event>,

    read_buffer: [u8; 512],
    receive_buffer: BytesMut,
    last_cr_pos: usize,
    last_scan_pos: usize,
}

impl Reader {
    pub fn new(
        reader: OwnedReadHalf,
        response_tx: flume::Sender<RawCommandResponse>,
        event_tx: flume::Sender<Event>
    ) -> Self {
        Self {
            reader: BufReader::new(reader),
            response_tx,
            event_tx,
            read_buffer: [0; 512],
            receive_buffer: BytesMut::new(),
            last_cr_pos: 0,
            last_scan_pos: 0,
        }
    }

    pub async fn run(mut self) -> Result<(), QueryError> {
        loop {
            let response = match self.next().await {
                Ok(response) => response,
                Err(e) => {
                    error!("Connection closed: {:?}", e);
                    return Err(e);
                }
            };

            match response {
                QueryResponse::Response(response) => {
                    self.response_tx.send_async(response).await
                        .map_err(|_| QueryError::ConnectionClosed)?;
                },
                QueryResponse::Event(event) => {
                    self.event_tx.send_async(event).await
                        .map_err(|_| QueryError::ConnectionClosed)?;
                }
            }
        }
    }

    async fn next(&mut self) -> Result<QueryResponse, QueryError> {
        loop {
            if let Some(response) = self.try_next()? {
                return Ok(response);
            }

            self.wait_for_bytes().await?;
        }
    }

    fn try_next(&mut self) -> Result<Option<QueryResponse>, QueryError> {
        loop {
            if self.last_scan_pos >= self.receive_buffer.len() {
                break;
            }

            let pos = self.receive_buffer[self.last_scan_pos..]
                .iter()
                .position(|&b| b == b'\r');

            let pos = match pos {
                Some(pos) => pos,
                None => break
            };

            let pos = pos + self.last_scan_pos + 1;
            let start = &self.receive_buffer[self.last_cr_pos..pos];

            if start.starts_with(b"error") {
                let response = self.receive_buffer.split_to(pos);
                let (content, status) = response.split_at(self.last_cr_pos);

                self.last_cr_pos = 0;
                self.last_scan_pos = 0;

                let content = std::str::from_utf8(&content[..max(2, content.len()) - 2])
                    .map_err(QueryError::MalformedUTF8)?;
                let status = std::str::from_utf8(&status[..status.len() - 2])
                    .map_err(QueryError::MalformedUTF8)?;

                if !content.is_empty() {
                    debug!("[S->C] {}", content);
                }

                debug!("[S->C] {}", status);

                let command = CommandResponse::decode(status, true)?;

                let response = RawCommandResponse {
                    status: command,
                    content: content.to_owned()
                };

                return Ok(Some(QueryResponse::Response(response)));
            } else if start.starts_with(b"notify") {
                let status = self.receive_buffer.split_to(pos);

                self.last_cr_pos = 0;
                self.last_scan_pos = 0;

                let status = std::str::from_utf8(&status[..status.len() - 2])
                    .map_err(QueryError::MalformedUTF8)?;

                debug!("[S->C] {}", status);

                let command = CommandResponse::decode(status, true)?;
                let event = Event::from(command)?;

                return Ok(Some(QueryResponse::Event(event)));
            }

            self.last_cr_pos = pos;
            self.last_scan_pos = pos;
        }

        self.last_scan_pos = self.receive_buffer.len();

        Ok(None)
    }

    async fn wait_for_bytes(&mut self) -> Result<(), QueryError> {
        let read_bytes = self.reader.read(&mut self.read_buffer).await
            .map_err(QueryError::ReadError)?;

        self.receive_buffer.extend_from_slice(&self.read_buffer[..read_bytes]);

        Ok(())
    }

    pub(super) async fn read_welcome_message(&mut self) -> Result<(), QueryError> {
        let mut buf = Vec::new();

        self.reader.read_until(b'\r', &mut buf).await
            .map_err(QueryError::ReadError)?;

        if buf != b"TS3\n\r" {
            return Err(QueryError::NotTS3Server);
        }

        self.reader.read_until(b'\r', &mut buf).await
            .map_err(QueryError::ReadError)?;

        Ok(())
    }

}
