use hickory_proto::error::ProtoError;
use std::{error::Error, io};
use tokio::sync::mpsc::error::SendError;

#[derive(Debug)]
pub enum ProcessQueryError<T> {
    Read(ProtoError),
    BuildFakeAnswer(ProtoError),
    Resolve(ResolveError),
    SendChannel(SendError<T>),
}

impl<T> std::fmt::Display for ProcessQueryError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_msg = match self {
            ProcessQueryError::Read(err) => format!("failed to read query {}", err),
            ProcessQueryError::BuildFakeAnswer(err) => {
                format!("failed to build fake answer {}", err)
            }
            ProcessQueryError::Resolve(err) => format!("failed to resolve real ip {}", err),
            ProcessQueryError::SendChannel(err) => format!("failed to send to channel {}", err),
        };
        write!(f, "{}", err_msg)
    }
}

impl<T> From<ResolveError> for ProcessQueryError<T> {
    fn from(err: ResolveError) -> Self {
        Self::Resolve(err)
    }
}

#[derive(Debug)]
pub enum ResolveError {
    Bind(io::Error),
    Connect(io::Error),
    Send(io::Error),
    Receive(io::Error),
}

impl std::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_msg = match self {
            ResolveError::Bind(err) => format!("failed to bind to dns server {:?}", err),
            ResolveError::Connect(err) => format!("failed to connect to dns server {:?}", err),
            ResolveError::Send(err) => format!("failed to send to dns server {:?}", err),
            ResolveError::Receive(err) => format!("failed to receive from dns server {:?}", err),
        };
        write!(f, "{}", err_msg)
    }
}

impl Error for ResolveError {}
