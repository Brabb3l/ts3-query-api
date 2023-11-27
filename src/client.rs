use std::sync::Arc;
use log::{debug, error};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::spawn;
use tokio::sync::RwLock;
use crate::error::QueryError;
use crate::event::{DefaultEventHandler, Event, EventHandler};
use crate::parser::{Command, CommandResponse};

pub struct QueryClient {
    command_tx: flume::Sender<TSCommand>,
    event_handler: Arc<RwLock<WrappedEventHandler>>,
    reader_loop_handle: tokio::task::JoinHandle<()>,
    writer_loop_handle: tokio::task::JoinHandle<()>,
    keep_alive_loop_handle: tokio::task::JoinHandle<()>,
}

impl QueryClient {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self, QueryError> {
        let stream = TcpStream::connect(addr)
            .await
            .map_err(QueryError::ConnectionFailed)?;

        let (command_tx, command_rx) = flume::unbounded::<TSCommand>();
        let (response_tx, response_rx) = flume::unbounded::<TSResponse>();
        let (reader, writer) = stream.into_split();

        let event_handler = Arc::new(RwLock::new(WrappedEventHandler::default()));
        let mut reader = BufReader::new(reader);

        Self::read_welcome_message(&mut reader).await?;

        Ok(Self {
            reader_loop_handle: spawn(Self::reader_loop(reader, response_tx, event_handler.clone())),
            writer_loop_handle: spawn(Self::writer_loop(writer, response_rx, command_rx)),
            keep_alive_loop_handle: spawn(Self::keep_alive_loop(command_tx.clone())),
            command_tx,
            event_handler,
        })
    }

    pub async fn send_command(&self, command: Command) -> Result<String, QueryError> {
        let command = command.into();
        let response = self.send_command_raw(command).await?;

        let content = std::str::from_utf8(response.content.as_slice())
            .map_err(QueryError::MalformedUTF8)?;
        let status = std::str::from_utf8(response.status.as_slice())
            .map_err(QueryError::MalformedUTF8)?;

        let mut status = CommandResponse::decode(status, true)?;

        let response_id = status.get::<i32>("id")?;

        if response_id == 0 {
            status.clear();
            Ok(content.to_owned())
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

    pub async fn set_event_handler<T: EventHandler + 'static>(&self, event_handler: T) {
        self.event_handler.write().await.handler = Arc::new(event_handler);
    }

    async fn read_welcome_message(reader: &mut BufReader<OwnedReadHalf>) -> Result<(), QueryError> {
        let mut buf = Vec::new();

        reader.read_until(b'\r', &mut buf).await
            .map_err(QueryError::ReadError)?;

        if buf != b"TS3\n\r" {
            return Err(QueryError::NotTS3Server);
        }

        reader.read_until(b'\r', &mut buf).await
            .map_err(QueryError::ReadError)?;

        Ok(())
    }

    async fn reader_loop(
        mut reader: BufReader<OwnedReadHalf>,
        response_tx: flume::Sender<TSResponse>,
        event_handler: Arc<RwLock<WrappedEventHandler>>
    ) {
        loop {
            let mut buf = Vec::new();
            let mut tmp_buf = Vec::new();

            loop {
                if let Err(e) = reader.read_until(b'\r', &mut tmp_buf).await {
                    error!("Failed to read from server: {}", e);
                    return;
                }

                tmp_buf.truncate(tmp_buf.len() - 2);

                if tmp_buf.starts_with(b"error") {
                    break;
                } else if tmp_buf.starts_with(b"notify") {
                    debug!("[S->C] {}", String::from_utf8_lossy(&tmp_buf));

                    let event = match CommandResponse::decode(std::str::from_utf8(&tmp_buf).unwrap(), true) {
                        Ok(event) => event,
                        Err(e) => {
                            if event_handler.read().await.handler.handle_error(e).await {
                                return;
                            }

                            tmp_buf.clear();
                            continue;
                        }
                    };

                    let event = match Event::from(event) {
                        Ok(event) => event,
                        Err(e) => {
                            if event_handler.read().await.handler.handle_error(e).await {
                                return;
                            }

                            tmp_buf.clear();
                            continue;
                        }
                    };

                    event_handler.read().await.handler.handle_event(event).await;
                    tmp_buf.clear();
                    break;
                } else {
                    buf.extend_from_slice(&tmp_buf);
                    tmp_buf.clear();
                }
            }

            if !buf.is_empty() {
                debug!("[S->C] {}", String::from_utf8_lossy(&buf));
            }

            debug!("[S->C] {}", String::from_utf8_lossy(&tmp_buf));

            let response = TSResponse {
                content: buf,
                status: tmp_buf
            };

            if let Err(e) = response_tx.send(response) {
                error!("Failed to send response: {}", e);
                return;
            }
        }
    }

    async fn writer_loop(
        mut writer: OwnedWriteHalf,
        response_rx: flume::Receiver<TSResponse>,
        command_rx: flume::Receiver<TSCommand>
    ) {
        while let Ok(mut command) = command_rx.recv_async().await {
            debug!("[C->S] {}", command.data);

            command.data.push_str("\n\r");

            if let Err(e) = writer.write_all(command.data.as_bytes()).await {
                error!("Failed to write to server: {}", e);
                return;
            }

            let response = match response_rx.recv_async().await {
                Ok(response) => response,
                Err(e) => {
                    error!("Failed to receive response: {}", e);
                    return;
                }
            };

            if let Err(e) = command.response_tx.send(response) {
                error!("Failed to send response: {}", e);
                return;
            }
        }
    }

    async fn keep_alive_loop(command_tx: flume::Sender<TSCommand>) {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;

            if let Err(e) = command_tx.send(TSCommand {
                data: "version".to_string(),
                response_tx: flume::unbounded::<TSResponse>().0
            }) {
                error!("Failed to send keep alive: {}", e);
                return;
            }

            debug!("Sent keep alive");
        }
    }

    async fn send_command_raw(&self, command: String) -> Result<TSResponse, QueryError> {
        let (response_tx, response_rx) = flume::unbounded::<TSResponse>();

        self.command_tx.send(TSCommand {
            data: command,
            response_tx
        }).map_err(|_| QueryError::ConnectionClosed)?;

        let response = response_rx.recv_async().await
            .map_err(|_| QueryError::ConnectionClosed)?;

        Ok(response)
    }

}

impl Drop for QueryClient {
    fn drop(&mut self) {
        self.reader_loop_handle.abort();
        self.writer_loop_handle.abort();
        self.keep_alive_loop_handle.abort();
    }
}

struct TSCommand {
    data: String,
    response_tx: flume::Sender<TSResponse>
}

struct TSResponse {
    content: Vec<u8>,
    status: Vec<u8>
}

struct WrappedEventHandler {
    handler: Arc<dyn EventHandler>
}

impl Default for WrappedEventHandler {
    fn default() -> Self {
        Self {
            handler: Arc::new(DefaultEventHandler)
        }
    }
}

