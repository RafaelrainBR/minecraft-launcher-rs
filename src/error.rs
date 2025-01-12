use zip_extract::ZipExtractError;

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    IoError(std::io::Error),
    SerializationError(serde_json::Error),
    NoVersionManifestError,
    VersionNotFound(String),
    VersionNotSelectedError,
    LibraryDownloadNotFound(String),
    NativeLibraryExtractError(ZipExtractError),
    ClientDownloadNotFound(String),
    RuntimeNotFound(String),
    TokioError(tokio::task::JoinError),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::RequestError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerializationError(err)
    }
}

impl From<ZipExtractError> for Error {
    fn from(err: ZipExtractError) -> Self {
        Error::NativeLibraryExtractError(err)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Error::TokioError(err)
    }
}
