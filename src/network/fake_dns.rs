use hickory_proto::rr::{RecordData, rdata, RecordType};
use hickory_proto::serialize::binary::BinEncoder;
use hickory_proto::{error::ProtoError, op::Header, rr::Record};
use hickory_server::authority::{MessageRequest, MessageResponseBuilder};
use regex::Regex;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::UdpSocket;

use crate::errors::ResolveError;
use crate::Sended;

type ResolveResult = Result<Vec<u8>, ResolveError<Sended>>;

pub(crate) fn is_matching(message: &MessageRequest, re: &Regex) -> bool {
    let query = message.query();

    query.query_type() == RecordType::A && re.is_match(&query.name().to_string())
}

pub(crate) fn build_fake_response(
    message: &MessageRequest,
    ip: Ipv4Addr,
    ttl: u32
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

pub(crate) async fn resolve(bytes: Vec<u8>) -> ResolveResult {
    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    let remote_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 53);
    sock.connect(&remote_addr).await?;

    sock.send(&bytes).await?;
    let mut data = [0u8; 1472];
    let len = sock.recv(&mut data).await?;

    let bytes = data[..len].to_vec();

    Ok(bytes)
}
