use std::fmt::Display;

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
    UnsupportedPlatform(String),
    IcedError(iced::Error),
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

impl From<iced::Error> for Error {
    fn from(err: iced::Error) -> Self {
        Error::IcedError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RequestError(err) => write!(f, "Request error: {}", err),
            Error::IoError(err) => write!(f, "IO error: {}", err),
            Error::SerializationError(err) => write!(f, "Serialization error: {}", err),
            Error::NoVersionManifestError => write!(f, "No version manifest error"),
            Error::VersionNotFound(id) => write!(f, "Version not found: {}", id),
            Error::VersionNotSelectedError => write!(f, "Version not selected error"),
            Error::LibraryDownloadNotFound(id) => write!(f, "Library download not found: {}", id),
            Error::NativeLibraryExtractError(err) => {
                write!(f, "Native library extract error: {}", err)
            }
            Error::ClientDownloadNotFound(id) => write!(f, "Client download not found: {}", id),
            Error::RuntimeNotFound(id) => write!(f, "Runtime not found: {}", id),
            Error::TokioError(err) => write!(f, "Tokio error: {}", err),
            Error::UnsupportedPlatform(platform) => write!(f, "Unsupported platform: {}", platform),
            Error::IcedError(err) => write!(f, "Iced error: {}", err),
        }
    }
}
