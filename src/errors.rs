use hickory_proto::error::ProtoError;
use std::io;
use tokio::sync::mpsc::error::SendError;

#[derive(Debug)]
pub enum ProcessBytesError<T> {
    Read(ProtoError),
    Build(ProtoError),
    Resolve(ResolveError<T>),
    Send(SendError<T>),
}

impl<T> From<ResolveError<T>> for ProcessBytesError<T> {
    fn from(err: ResolveError<T>) -> Self {
        Self::Resolve(err)
    }
}

#[derive(Debug)]
pub enum ResolveError<T> {
    IO(io::Error),
    Send(SendError<T>),
}

impl<T> From<SendError<T>> for ResolveError<T> {
    fn from(err: SendError<T>) -> Self {
        Self::Send(err)
    }
}

impl<T> From<io::Error> for ResolveError<T> {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}
