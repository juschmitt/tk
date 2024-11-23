use std::io;

#[derive(Debug)]
pub enum OAuthError {
    Tcp,
    Http,
    ResponseParsing,
    RedirectParsing,
    State,
}

impl From<reqwest::Error> for OAuthError {
    fn from(_: reqwest::Error) -> Self {
        OAuthError::Http
    }
}

impl From<serde_json::Error> for OAuthError {
    fn from(_: serde_json::Error) -> Self {
        OAuthError::ResponseParsing
    }
}

impl From<io::Error> for OAuthError {
    fn from(_: io::Error) -> Self {
        OAuthError::Tcp
    }
}