use tokio::net::TcpStream;
use tokio::time::sleep;
use crate::error::QueryError;
use crate::event::Event;
use super::reader::Reader;
use super::types::{RawCommandRequest, RawCommandResponse};
use super::writer::Writer;

pub(super) struct Connection {
    reader: Reader,
    writer: Writer,
    command_tx: flume::Sender<RawCommandRequest>,
}

impl Connection {
    pub fn new(
        stream: TcpStream,
        event_tx: flume::Sender<Event>,
        command_rx: flume::Receiver<RawCommandRequest>,
        command_tx: flume::Sender<RawCommandRequest>,
    ) -> Self {
        let (response_tx, response_rx) = flume::unbounded::<RawCommandResponse>();
        let (reader, writer) = stream.into_split();

        Self {
            reader: Reader::new(reader, response_tx, event_tx),
            writer: Writer::new(writer, response_rx, command_rx),
            command_tx,
        }
    }

    pub async fn read_welcome_message(&mut self) -> Result<(), QueryError> {
        self.reader.read_welcome_message().await
    }

    pub async fn run(self) -> Result<(), QueryError> {
        let reader_handle = tokio::spawn(self.reader.run());
        let writer_handle = tokio::spawn(self.writer.run());
        let keep_alive_handle = tokio::spawn(Self::keep_alive_loop(self.command_tx));

        let (r1, r2, r3) = tokio::try_join!(reader_handle, writer_handle, keep_alive_handle)
            .expect("Failed to join threads");

        r1?;
        r2?;
        r3?;

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

            let mut status = response_rx.recv_async().await
                .map_err(|_| QueryError::ConnectionClosed)?.status;

            let response_id = status.get::<i32>("id")?;

            if response_id != 0 {
                return Err(QueryError::QueryError {
                    id: response_id,
                    message: status.get("msg")?,
                    response: status
                });
            }
        }
    }
}