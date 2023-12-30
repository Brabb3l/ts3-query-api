use tokio::net::TcpStream;
use tokio::time::sleep;
use crate::definitions::Status;
use crate::error::QueryError;
use crate::event::Event;
use crate::parser::Decoder;
use super::reader::Reader;
use super::types::{RawCommandRequest, RawCommandResponse};
use super::writer::Writer;

pub(super) struct Connection {
    reader: Reader,
    writer: Writer,
    command_tx: flume::Sender<RawCommandRequest>,
    shutdown_rx: flume::Receiver<()>,
}

impl Connection {
    pub fn new(
        stream: TcpStream,
        event_tx: flume::Sender<Event>,
        command_rx: flume::Receiver<RawCommandRequest>,
        command_tx: flume::Sender<RawCommandRequest>,
        shutdown_rx: flume::Receiver<()>,
    ) -> Self {
        let (response_tx, response_rx) = flume::unbounded::<RawCommandResponse>();
        let (reader, writer) = stream.into_split();

        Self {
            reader: Reader::new(reader, response_tx, event_tx),
            writer: Writer::new(writer, response_rx, command_rx),
            command_tx,
            shutdown_rx,
        }
    }

    pub async fn read_welcome_message(&mut self) -> Result<(), QueryError> {
        self.reader.read_welcome_message().await
    }

    pub async fn run(self) -> Result<(), QueryError> {
        tokio::select! {
            r = self.reader.run() => r?,
            r = self.writer.run() => r?,
            r = Self::keep_alive_loop(self.command_tx) => r?,
            _ = self.shutdown_rx.recv_async() => {},
        }

        Ok(())
    }

    async fn keep_alive_loop(command_tx: flume::Sender<RawCommandRequest>) -> Result<(), QueryError> {
        loop {
            sleep(std::time::Duration::from_secs(60)).await;

            let (response_tx, response_rx) = flume::unbounded::<RawCommandResponse>();

            command_tx.send(RawCommandRequest {
                data: "version\n\r".to_string(),
                response_tx
            }).map_err(|_| QueryError::ConnectionClosed)?;

            let status = response_rx.recv_async().await
                .map_err(|_| QueryError::ConnectionClosed)?;

            let status = Decoder::new(status.status())
                .decode_with_name::<Status>()
                .map_err(QueryError::ParseError)?;

            if status.id != 0 {
                return Err(QueryError::QueryError {
                    id: status.id,
                    message: status.message
                });
            }
        }
    }
}
