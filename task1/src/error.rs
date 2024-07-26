use thiserror::Error;

#[derive(Error, Debug)]
pub enum DLogProofError {
    #[error("Deserializing error: {0}")]
    DeserializingError(String),

    #[error("Serializing error: {0}")]
    SerializingError(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
