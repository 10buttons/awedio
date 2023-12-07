/// Awedio error.
///
/// These errors mostly originate in decoder libraries. If the library error is
/// just a std::io::Error it is unwrapped into the IoError variant otherwise the
/// error is wrapped in the FormatError variant.
#[derive(Debug)]
pub enum Error {
    /// A I/O operation failed
    IoError(std::io::Error),
    /// An error other than std::io::Error occurred. The real type of the boxed
    /// error is normally the error type of the format decoding library.
    FormatError(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IoError(e) => e.source(),
            Error::FormatError(e) => Some(e.as_ref()),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::IoError(ref err) => err.fmt(f),
            Error::FormatError(ref inner) => write!(f, "format error: {}", inner),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}
