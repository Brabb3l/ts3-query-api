mod client;

pub mod request;

pub mod parser;
pub mod responses;

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
