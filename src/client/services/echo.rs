//! Echo Service Client Implementation
//! This file shows:
//! 1. Simple gRPC client wrapper implementation
//! 2. Generic input handling with Into<String>
//! 3. Client-side validation

use tonic::{Request, Status, Code};
use crate::proto::echo::{
    echo_service_client::EchoServiceClient,
    EchoRequest,
};
use super::super::client::GrpcClient;

// Client wrapper with generated gRPC client
#[derive(Clone)]
pub struct EchoService {
    // Internal generated client instance
    client: EchoServiceClient<tonic::transport::Channel>,
}

// Extension method for main client
impl GrpcClient {
    // Create new echo service instance
    pub fn echo(&self) -> EchoService {
        EchoService {
            client: EchoServiceClient::new(self.get_channel())
        }
    }
}

// Main service implementation
impl EchoService {
    // Echo method that accepts any string-like input
    pub async fn echo(&mut self, message: impl Into<String>) -> Result<String, Status> {
        let message = message.into();
        
        // Client-side validation before making RPC call
        if message.trim().is_empty() {
            return Err(Status::new(
                Code::InvalidArgument,
                "empty message is not allowed"
            ));
        }

        // Create and send request
        let request = Request::new(EchoRequest { message });
        let response = self.client.echo(request).await?;
        Ok(response.into_inner().message)
    }
}

// Test for not allowing empty messages to be sent
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_echo_empty_message() {
        let client = GrpcClient::builder("http://[::1]:50051")
            .unwrap()
            .connect()
            .unwrap();
            
        let mut echo = client.echo();
        
        let err = echo.echo("").await.unwrap_err();
        assert_eq!(err.code(), Code::InvalidArgument);
        assert!(err.message().contains("empty message"));

        let err = echo.echo("    ").await.unwrap_err();
        assert_eq!(err.code(), Code::InvalidArgument);
        assert!(err.message().contains("empty message"));
    }
}
