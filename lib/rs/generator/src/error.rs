use std::fmt::Formatter;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Json(serde_json::Error),
    InvalidFileName(String),
    NamespaceCollision(String),
    UnknownType(String),
    CircularDependency,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(e) => {
                write!(f, "{e}")
            }
            Error::Json(e) => {
                write!(f, "{e}")
            },
            Error::InvalidFileName(s) => {
                write!(f, "Invalid file name: {s}")
            },
            Error::NamespaceCollision(s) => {
                write!(f, "Namespace collision: {s}")
            },
            Error::UnknownType(s) => {
                write!(f, "Unknown type: {s}")
            },
            Error::CircularDependency => {
                write!(f, "Circular dependency! TODO: Show subgraph.")
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Json(value)
    }
}

impl std::error::Error for Error {}