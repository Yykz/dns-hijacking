use hickory_proto::rr::{rdata, RecordData, RecordType};
use hickory_proto::serialize::binary::BinEncoder;
use hickory_proto::{error::ProtoError, op::Header, rr::Record};
use hickory_server::authority::{MessageRequest, MessageResponseBuilder};
use regex::Regex;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;

use crate::errors::ResolveError;

type ResolveResult = Result<Vec<u8>, ResolveError>;

pub(crate) fn is_matching(message: &MessageRequest, re: &Regex) -> bool {
    let query = message.query();

    query.query_type() == RecordType::A && re.is_match(&query.name().to_string())
}

pub(crate) fn build_fake_response(
    message: &MessageRequest,
    ip: Ipv4Addr,
    ttl: u32,
) -> Result<Vec<u8>, ProtoError> {
    let builder = MessageResponseBuilder::from_message_request(message);
    let header = Header::response_from_request(message.header());

    let name = message.query().original().name().clone();
    let record = Record::from_rdata(name, ttl, rdata::A(ip).into_rdata());

    let message_response = builder.build(header, [&record], [], [], []);

    let mut buf = vec![0; 1472];
    let mut binencoder = BinEncoder::new(&mut buf);
    message_response.destructive_emit(&mut binencoder)?;
    Ok(binencoder.into_bytes().to_vec())
}

pub(crate) async fn resolve_domain(bytes: Vec<u8>) -> ResolveResult {
    let sock = UdpSocket::bind("0.0.0.0:0")
        .await
        .map_err(ResolveError::Bind)?;
    let remote_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 53);
    sock.connect(&remote_addr)
        .await
        .map_err(ResolveError::Connect)?;

    sock.send(&bytes).await.map_err(ResolveError::Send)?;
    let mut data = [0u8; 1472];
    let received = timeout(Duration::from_secs(1), sock.recv(&mut data))
        .await
        .map_err(ResolveError::Timeout)?;
    let len = received.map_err(ResolveError::Receive)?;

    let bytes = data[..len].to_vec();

    Ok(bytes)
}
