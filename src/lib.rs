/// # TeamSpeak 3 Query Library
///
/// ## Example
/// ```no_run
/// use ts3_query_api::QueryClient;
/// use ts3_query_api::error::QueryError;
/// use ts3_query_api::definitions::EventType;
///
/// #[tokio::main]
/// async fn main() -> Result<(), QueryError> {
///     let client = QueryClient::connect(("localhost", 10011)).await?;
///
///     // Login and select virtual server
///     client.login("username", "password").await?;
///     client.use_sid(1).await?;
///     client.server_notify_register(EventType::Channel).await?;
///
///     // Wait for events
///     while let Ok(event) = client.wait_for_event().await {
///         // ...
///     }
///
///     Ok(())
/// }
/// ```
pub mod requests;

pub mod definitions;
pub mod event;
pub mod parser;

pub mod error;

mod macros;
mod protocol;

pub use protocol::*;
