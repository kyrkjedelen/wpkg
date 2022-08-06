use std::error;
use std::fmt; use std::fmt::{ Formatter, Display, Debug };

pub enum ErrorKind {
    BadPackage,
}
impl ErrorKind {
    fn as_str(&self) -> &str {
        use ErrorKind::*;
        match *self {
            BadPackage => "unvalid package",
        }
    }
}

pub struct Error<T> {
    kind: ErrorKind,
    error: T,
}
impl<T> Error<T> {
    pub fn new(kind: ErrorKind, error: T) -> Self {
        Error { kind, error }
    }
}
impl<T: Display> Error<T> {
    fn as_str(&self) -> String {
        format!("{}: {}", self.kind.as_str(), self.error)
    }
}
impl<T: Display> Display for Error<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.as_str())
    }
}
impl<T: Display> Debug for Error<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}(dbug)", self.as_str())
    }
}

impl<T: Display> error::Error for Error<T> {}