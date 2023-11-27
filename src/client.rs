use log::{debug, error};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::spawn;
use crate::error::QueryError;
use crate::parser::{Command, CommandResponse};

struct TSCommand {
    data: String,
    response_tx: flume::Sender<TSResponse>
}

struct TSResponse {
    content: Vec<u8>,
    status: Vec<u8>
}

pub struct QueryClient {
    command_tx: flume::Sender<TSCommand>,
}

impl QueryClient {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self, QueryError> {
        let stream = TcpStream::connect(addr)
            .await
            .map_err(QueryError::ConnectionFailed)?;

        let (command_tx, command_rx) = flume::unbounded::<TSCommand>();
        let (response_tx, response_rx) = flume::unbounded::<TSResponse>();
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);

        // read the welcome message

        let mut buf = Vec::new();

        reader.read_until(b'\r', &mut buf).await
            .map_err(QueryError::ReadError)?;

        if buf != b"TS3\n\r" {
            return Err(QueryError::NotTS3Server);
        }

        reader.read_until(b'\r', &mut buf).await
            .map_err(QueryError::ReadError)?;

        // setup reader thread

        spawn(async move {
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
                    } else {
                        buf.extend_from_slice(&tmp_buf);
                        tmp_buf.clear();
                    }
                }

                debug!("[S->C] {}", String::from_utf8_lossy(&buf));
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
        });

        // setup writer thread

        spawn({
            let response_rx = response_rx.clone();

            async move {
                while let Ok(command) = command_rx.recv_async().await {
                    debug!("[C->S] {}", command.data);

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
        });

        Ok(Self {
            command_tx,
        })
    }

    async fn send_command_raw(&self, mut command: String) -> Result<TSResponse, QueryError> {
        let (response_tx, response_rx) = flume::unbounded::<TSResponse>();

        command.push_str("\n\r");

        self.command_tx.send(TSCommand {
            data: command,
            response_tx
        }).map_err(|_| QueryError::ConnectionClosed)?;

        let response = response_rx.recv_async().await
            .map_err(|_| QueryError::ConnectionClosed)?;

        Ok(response)
    }

    pub async fn send_command(&self, command: Command) -> Result<String, QueryError> {
        let command = command.into();
        let response = self.send_command_raw(command).await?;

        let content = std::str::from_utf8(response.content.as_slice())
            .map_err(QueryError::MalformedUTF8)?;
        let status = std::str::from_utf8(response.status.as_slice())
            .map_err(QueryError::MalformedUTF8)?;

        let mut status = CommandResponse::decode(status, true)?;

        let response_id = status.get_i32("id")?;

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
}







