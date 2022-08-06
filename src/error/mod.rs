pub mod deberror;


trait ErrorKind {
    fn as_str(&self) -> String;
}

use std::error::Error as StdError;
trait Error: StdError {
    fn new<T>(kind: impl ErrorKind, error: T) -> Self;
    fn kind(&self) -> dyn ErrorKind;
}