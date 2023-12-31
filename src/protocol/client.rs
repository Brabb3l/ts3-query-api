use crate::definitions::Status;
use crate::error::{ParseError, QueryError};
use crate::event::Event;
use crate::parser::{Command, Decode, DecodeCustomInto, DecodeInto, Decoder};
use crate::protocol::connection::Connection;
use crate::protocol::types::{RawCommandRequest, RawCommandResponse};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::spawn;

pub struct QueryClient {
    command_tx: flume::Sender<RawCommandRequest>,
    event_rx: flume::Receiver<Event>,
    shutdown_tx: flume::Sender<()>,
}

impl QueryClient {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self, QueryError> {
        let stream = TcpStream::connect(addr)
            .await
            .map_err(QueryError::ConnectionFailed)?;

        let (command_tx, command_rx) = flume::unbounded::<RawCommandRequest>();
        let (event_tx, event_rx) = flume::unbounded::<Event>();
        let (shutdown_tx, shutdown_rx) = flume::unbounded::<()>();

        let mut connection = Connection::new(
            stream,
            event_tx,
            command_rx,
            command_tx.clone(),
            shutdown_rx,
        );

        connection.read_welcome_message().await?;

        spawn(connection.run());

        Ok(Self {
            command_tx,
            event_rx,
            shutdown_tx,
        })
    }

    pub async fn send_command_no_response(&self, command: Command) -> Result<(), QueryError> {
        let command = command.into();
        self.send_command_internal(command).await?;
        Ok(())
    }

    pub async fn send_command<T: Decode>(&self, command: Command) -> Result<T, QueryError> {
        let response = self.send_command_raw(command).await?;
        let mut decoder = Decoder::new(response.content());

        decoder.decode().map_err(QueryError::ParseError)
    }

    pub async fn send_command_into<I: DecodeInto>(
        &self,
        command: Command,
        dst: I,
    ) -> Result<I, QueryError> {
        let response = self.send_command_raw(command).await?;
        let mut decoder = Decoder::new(response.content());

        dst.decode_into(&mut decoder)
            .map_err(QueryError::ParseError)
    }

    pub async fn send_command_custom_into<F, T, I: DecodeCustomInto<T>>(
        &self,
        command: Command,
        dst: I,
        gen: F,
    ) -> Result<I, QueryError>
    where
        F: Fn(&mut Decoder) -> Result<T, ParseError>,
    {
        let response = self.send_command_raw(command).await?;
        let mut decoder = Decoder::new(response.content());

        dst.decode_into(&mut decoder, gen)
            .map_err(QueryError::ParseError)
    }

    pub async fn wait_for_event(&self) -> Result<Event, QueryError> {
        self.event_rx
            .recv_async()
            .await
            .map_err(|_| QueryError::ConnectionClosed)
    }

    async fn send_command_internal(
        &self,
        mut command: String,
    ) -> Result<RawCommandResponse, QueryError> {
        let (response_tx, response_rx) = flume::unbounded::<RawCommandResponse>();

        command.push_str("\n\r");

        self.command_tx
            .send_async(RawCommandRequest {
                data: command,
                response_tx,
            })
            .await
            .map_err(|_| QueryError::ConnectionClosed)?;

        let response = response_rx
            .recv_async()
            .await
            .map_err(|_| QueryError::ConnectionClosed)?;

        Ok(response)
    }

    pub async fn send_command_raw(
        &self,
        command: Command,
    ) -> Result<RawCommandResponse, QueryError> {
        let command = command.into();
        let response = self.send_command_internal(command).await?;

        let status = Decoder::new(response.status())
            .decode_with_name::<Status>()
            .map_err(QueryError::ParseError)?;

        if status.id == 0 {
            Ok(response)
        } else {
            Err(QueryError::QueryError {
                id: status.id,
                message: status.message,
            })
        }
    }
}

impl Drop for QueryClient {
    fn drop(&mut self) {
        let _ = self.shutdown_tx.send(());
    }
}
