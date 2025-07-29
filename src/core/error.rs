use thiserror::Error;

#[derive(Error, Debug)]
pub enum PipelineError {
    #[error("Source error: {0}")]
    Source(#[from] anyhow::Error),
    
    #[error("Sink error: {0}")]
    Sink(String),
    
    #[error("Transform error: {0}")]
    Transform(String),
    
    #[error("Schema error: {0}")]
    Schema(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, PipelineError>;