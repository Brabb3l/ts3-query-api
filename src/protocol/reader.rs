use crate::error::QueryError;
use crate::event::Event;
use crate::protocol::types::RawCommandResponse;
use bytes::BytesMut;
use log::{debug, error, log_enabled};
use std::cmp::max;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;

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
        event_tx: flume::Sender<Event>,
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
            if let Err(e) = self.try_next().await {
                error!("Connection closed: {:?}", e);
                return Err(e);
            }

            if let Err(e) = self.wait_for_bytes().await {
                error!("Connection closed: {:?}", e);
                return Err(e);
            }
        }
    }

    async fn try_next(&mut self) -> Result<(), QueryError> {
        loop {
            if self.last_scan_pos >= self.receive_buffer.len() {
                break;
            }

            let pos = self.receive_buffer[self.last_scan_pos..]
                .iter()
                .position(|&b| b == b'\r');

            let pos = match pos {
                Some(pos) => pos,
                None => break,
            };

            let pos = pos + self.last_scan_pos + 1;
            let start = &self.receive_buffer[self.last_cr_pos..pos];

            if start.starts_with(b"error") {
                let response = self.receive_buffer.split_to(pos).freeze();
                let mid_index = self.last_cr_pos;

                self.last_cr_pos = 0;
                self.last_scan_pos = 0;

                if log_enabled!(log::Level::Debug) {
                    let (content, status) = response.split_at(mid_index);

                    let content = &content[..max(2, content.len()) - 2];
                    let status = &status[..status.len() - 2];

                    if !content.is_empty() {
                        debug!(
                            "[S->C] {}",
                            std::str::from_utf8(content).unwrap_or("~Malformed UTF-8~")
                        );
                    }

                    debug!(
                        "[S->C] {}",
                        std::str::from_utf8(status).unwrap_or("~Malformed UTF-8~")
                    );
                }

                let response = RawCommandResponse {
                    response,
                    mid_index,
                };

                self.response_tx
                    .send_async(response)
                    .await
                    .map_err(|_| QueryError::ConnectionClosed)?;
            } else if start.starts_with(b"notify") {
                let scrambled_data = self.receive_buffer.split_off(self.last_cr_pos);
                let (status, remaining) = scrambled_data.split_at(pos - self.last_cr_pos);

                self.receive_buffer.extend_from_slice(remaining);

                let status = std::str::from_utf8(&status[..status.len() - 2])
                    .map_err(QueryError::MalformedUTF8)?;

                if log_enabled!(log::Level::Debug) {
                    debug!("[S->C] {}", status);
                }

                let event = Event::from(status)?;

                self.event_tx
                    .send_async(event)
                    .await
                    .map_err(|_| QueryError::ConnectionClosed)?;
            } else {
                self.last_cr_pos = pos;
                self.last_scan_pos = pos;
            }
        }

        self.last_scan_pos = self.receive_buffer.len();

        Ok(())
    }

    async fn wait_for_bytes(&mut self) -> Result<(), QueryError> {
        let read_bytes = self
            .reader
            .read(&mut self.read_buffer)
            .await
            .map_err(QueryError::ReadError)?;

        self.receive_buffer
            .extend_from_slice(&self.read_buffer[..read_bytes]);

        Ok(())
    }

    pub(super) async fn read_welcome_message(&mut self) -> Result<(), QueryError> {
        let mut buf = Vec::new();

        self.reader
            .read_until(b'\r', &mut buf)
            .await
            .map_err(QueryError::ReadError)?;

        if buf != b"TS3\n\r" {
            return Err(QueryError::NotTS3Server);
        }

        self.reader
            .read_until(b'\r', &mut buf)
            .await
            .map_err(QueryError::ReadError)?;

        Ok(())
    }
}
