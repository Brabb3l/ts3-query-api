/// # TeamSpeak 3 Query Library
///
/// ## Example
/// ```no_run
/// use ts3_query_api::QueryClient;
/// use ts3_query_api::error::QueryError;
///
/// #[tokio::main]
/// async fn main() -> Result<(), QueryError> {
///     let client = QueryClient::connect(("localhost", 10011)).await?;
///
///     // Login and select virtual server
///     client.login("username", "password").await?;
///     client.use_sid(1).await?;
///
///     // ...
///
///     Ok(())
/// }
/// ```
///
/// ```no_run
/// use ts3_query_api::QueryClient;
/// use ts3_query_api::error::QueryError;
/// use ts3_query_api::event::{Event, EventHandler, EventType};
///
/// #[derive(Default)]
/// struct MyHandler;
///
/// #[async_trait::async_trait]
/// impl EventHandler for MyHandler {
///     async fn handle_event(&self, event: Event) {
///         match event {
///             Event::TextMessage(event) => {
///                 println!("Received text message from {}: {}", event.invoker_id, event.message);
///             }
///             _ => {}
///         }
///     }
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), QueryError> {
///     let client = QueryClient::connect(("localhost", 10011)).await?;
///
///     // Login and select virtual server
///     client.login("username", "password").await?;
///     client.use_sid(1).await?;
///
///     // Register for server events
///     client.server_notify_register(EventType::TextChannel, Some(0)).await?;
///
///     // Set event handler
///     client.set_event_handler(MyHandler::default()).await;
///
///     // ...
///
///     Ok(())
/// }
/// ```
///

mod client;

pub mod request;

pub mod parser;
pub mod responses;
pub mod event;

pub mod error;
pub mod properties;

mod macros;

pub use client::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test]
    #[ignore]
    async fn test_connection() {
        init();

        let client = match QueryClient::connect(("localhost", 10011)).await {
            Ok(client) => client,
            Err(e) => panic!("Failed to connect to server\n{:?}", e),
        };

        if let Err(e) = client.version().await {
            panic!("Failed to get server version\n{:?}", e);
        }
    }
}
