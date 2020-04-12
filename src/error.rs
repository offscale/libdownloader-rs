// Result<OsString, (SendRequestError|std::core::convert::Infallible|actix_http::error::PayloadError)
#[derive(Debug)]
pub enum DownloadError {
    Boxed(Box<dyn std::error::Error + Send + Sync>),
    HttpError(http::Error),
    Infallible,
    Io(std::io::Error),
    None,
    ParseUri(http::uri::InvalidUri),
}

impl From<Box<dyn std::error::Error + Send + Sync>> for DownloadError {
    fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
        DownloadError::Boxed(e)
    }
}

impl From<http::Error> for DownloadError {
    fn from(e: http::Error) -> Self {
        DownloadError::HttpError(e)
    }
}

impl From<std::convert::Infallible> for DownloadError {
    fn from(_: std::convert::Infallible) -> Self {
        DownloadError::Infallible
    }
}

impl From<std::io::Error> for DownloadError {
    fn from(e: std::io::Error) -> Self {
        DownloadError::Io(e)
    }
}

impl From<http::uri::InvalidUri> for DownloadError {
    fn from(e: http::uri::InvalidUri) -> Self {
        DownloadError::ParseUri(e)
    }
}
