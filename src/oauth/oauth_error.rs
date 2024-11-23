#[derive(Debug)]
pub enum OAuthError {
    TcpError,
    HttpError,
    ResponseParsingError,
}

impl From<reqwest::Error> for OAuthError {
    fn from(_: reqwest::Error) -> Self {
        OAuthError::HttpError
    }
}

impl From<serde_json::Error> for OAuthError {
    fn from(_: serde_json::Error) -> Self {
        OAuthError::ResponseParsingError
    }
}