use thiserror::Error;

#[derive(Debug, Error)]
pub enum DfilesError {
    #[error("Profile not found: {0}")]
    ProfileNotFound(String),

    #[error("Config parse error: {0}")]
    ConfigParse(#[from] toml::de::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Symlink error: {src} -> {dst}: {reason}")]
    Symlink {
        src: String,
        dst: String,
        reason: String,
    },

    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
