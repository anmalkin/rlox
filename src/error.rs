use std::io;

#[derive(Debug)]
pub enum Error {
    Compiler,
    Runtime,
    IO(io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compiler => write!(f, "Compiler error."),
            Self::Runtime => write!(f, "Runtime error"),
            Self::IO(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

pub type RloxResult = Result<(), Error>;
