use thiserror::Error;

pub type Result<T, E = MinraError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum MinraError {
    #[error("{0}")]
    InitError(#[from] InitError),
}

#[derive(Error, Debug)]
pub enum InitError {
    #[error("Could not connect to database")]
    DbError,
    #[error("Tauri error: {0}")]
    TauriError(#[from] tauri::path::Error),
    #[error("Sea-orm error: {0}")]
    SeaOrmError(#[from] sea_orm::DbErr),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Derialization error: {0}")]
    DeserError(#[from] toml::de::Error),
    #[error("Derialization error: {0}")]
    SerError(#[from] toml::ser::Error),
    #[error("Error: {0}")]
    UncategorizedError(String),
}
