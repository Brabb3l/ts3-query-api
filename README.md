# Teamspeak 3 Server Query API for Rust

## Example

```rust
use ts3_query_api::QueryClient;
use ts3_query_api::error::QueryError;

#[tokio::main]
async fn main() -> Result<(), QueryError> {
    let client = QueryClient::connect(("localhost", 10011)).await?;

    // Login and select virtual server
    client.login("username", "password").await?;
    client.use_sid(1).await?;

    // ...

    Ok(())
}
```

## Example with event handler

```rust
use ts3_query_api::QueryClient;
use ts3_query_api::error::QueryError;
use ts3_query_api::event::{Event, EventHandler, EventType};

#[derive(Default)]
struct MyHandler;

#[async_trait::async_trait]
impl EventHandler for MyHandler {
    async fn handle_event(&self, _client: QueryClient, event: Event) {
        match event {
            Event::TextMessage(event) => {
                println!("Received text message from {}: {}", event.invoker_id, event.message);
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), QueryError> {
    let client = QueryClient::connect(("localhost", 10011)).await?;

    // Login and select virtual server
    client.login("username", "password").await?;
    client.use_sid(1).await?;

    // Register for server events
    client.server_notify_register(EventType::TextChannel, Some(0)).await?;

    // Set event handler
    client.set_event_handler(MyHandler::default()).await;

    // ...

    Ok(())
}
```
