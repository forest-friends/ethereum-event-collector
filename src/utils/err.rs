#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("ConfigLoadError: {0}")]
    ConfigLoadError(#[from] config::ConfigError),
    #[error("UrlParseError: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Web3Error: {0}")]
    Web3Error(#[from] web3::Error),
    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),
    #[error("TokioJoinError: {0}")]
    TokioJoinError(#[from] tokio::task::JoinError),
}