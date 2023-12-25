use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::spawn;
use crate::error::QueryError;
use crate::event::Event;
use crate::parser::{Command, CommandResponse};
use crate::protocol::connection::Connection;
use crate::protocol::types::{RawCommandRequest, RawCommandResponse};

pub struct QueryClient {
    command_tx: flume::Sender<RawCommandRequest>,
    event_rx: flume::Receiver<Event>,
    shutdown_tx: flume::Sender<()>
}

impl QueryClient {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self, QueryError> {
        let stream = TcpStream::connect(addr)
            .await
            .map_err(QueryError::ConnectionFailed)?;

        let (command_tx, command_rx) = flume::unbounded::<RawCommandRequest>();
        let (event_tx, event_rx) = flume::unbounded::<Event>();
        let (shutdown_tx, shutdown_rx) = flume::unbounded::<()>();

        let mut connection = Connection::new(stream, event_tx, command_rx, command_tx.clone(), shutdown_rx);

        connection.read_welcome_message().await?;

        spawn(connection.run());

        Ok(Self {
            command_tx,
            event_rx,
            shutdown_tx,
        })
    }

    pub async fn send_command(&self, command: Command) -> Result<String, QueryError> {
        let command = command.into();
        let response = self.send_command_raw(command).await?;

        let mut status = response.status;
        let response_id = status.get::<i32>("id")?;

        if response_id == 0 {
            Ok(response.content)
        } else {
            Err(QueryError::QueryError {
                id: response_id,
                message: status.get("msg")?,
                response: status
            })
        }
    }

    pub async fn send_command_decode(&self, command: Command) -> Result<CommandResponse, QueryError> {
        self.send_command(command).await
            .map(|v| CommandResponse::decode(&v, false))?
    }

    pub async fn send_command_multi_decode(&self, command: Command) -> Result<Vec<CommandResponse>, QueryError> {
        self.send_command(command).await
            .map(|v| CommandResponse::decode_multi(&v))?
    }

    pub async fn wait_for_event(&self) -> Result<Event, QueryError> {
        self.event_rx.recv_async().await
            .map_err(|_| QueryError::ConnectionClosed)
    }

    async fn send_command_raw(&self, mut command: String) -> Result<RawCommandResponse, QueryError> {
        let (response_tx, response_rx) = flume::unbounded::<RawCommandResponse>();

        command.push_str("\n\r");

        self.command_tx.send_async(RawCommandRequest {
            data: command,
            response_tx
        }).await.map_err(|_| QueryError::ConnectionClosed)?;

        let response = response_rx.recv_async().await
            .map_err(|_| QueryError::ConnectionClosed)?;

        Ok(response)
    }

}

impl Drop for QueryClient {
    fn drop(&mut self) {
        let _ = self.shutdown_tx.send(());
    }
}