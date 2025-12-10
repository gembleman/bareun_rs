use thiserror::Error;

#[derive(Error, Debug)]
pub enum BareunError {
    #[error("API key must be provided")]
    MissingApiKey,

    #[error("Failed to connect to server at {host}:{port}: {source}")]
    ConnectionFailed {
        host: String,
        port: u16,
        source: tonic::transport::Error,
    },

    #[error("Permission denied. Check your API key: {apikey}\nServer message: {message}")]
    PermissionDenied { apikey: String, message: String },

    #[error("Server unavailable at {host}:{port}\nServer message: {message}")]
    ServerUnavailable {
        host: String,
        port: u16,
        message: String,
    },

    #[error("Invalid argument: {message}")]
    InvalidArgument { message: String },

    #[error("gRPC error: {0}")]
    GrpcError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),

    #[error("Invalid metadata value")]
    InvalidMetadataValue(#[from] tonic::metadata::errors::InvalidMetadataValue),

    #[error("Invalid custom dictionary name: {0}")]
    InvalidCustomDictName(String),

    #[error("gRPC Status error: {0}")]
    StatusError(#[from] tonic::Status),
}

pub type Result<T> = std::result::Result<T, BareunError>;
