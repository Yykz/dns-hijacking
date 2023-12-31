mod fake_dns;
use fake_dns::{build_fake_response, is_matching, resolve_domain};

use hickory_proto::serialize::binary::{BinDecodable, BinDecoder};
use hickory_server::authority::MessageRequest;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

use crate::cli::Options;
use crate::errors::ProcessQueryError;

use crate::Sended;

use chrono::Local;

pub(crate) async fn process_query(
    bytes: Vec<u8>,
    tx: Sender<Sended>,
    addr: SocketAddr,
    opt: Arc<Options>,
) -> Result<(), ProcessQueryError<Sended>> {
    let mut bindecoder = BinDecoder::new(&bytes);
    let message = MessageRequest::read(&mut bindecoder).map_err(ProcessQueryError::Read)?;

    let bytes = match is_matching(&message, &opt.entries) {
        Some(entry) => {
            let bytes = build_fake_response(&message, entry, opt.ttl)
                .map_err(ProcessQueryError::BuildFakeAnswer)?;
            if opt.verbose >= 1 {
                println!(
                    "[{}] Redirected request of type '{}' for {} to {}",
                    Local::now().format("%H:%M:%S"),
                    message.query().query_type(),
                    message.query().name(),
                    entry
                        .rdata
                        .ip_addr()
                        .map(|ip| ip.to_string())
                        .unwrap_or_default()
                );
            }
            bytes
        }
        None => {
            let bytes = resolve_domain(bytes).await?;
            if opt.verbose >= 2 {
                println!(
                    "[{}] Forwarded request of type '{}' for {}",
                    Local::now().format("%H:%M:%S"),
                    message.query().query_type(),
                    message.query().name(),
                );
            }
            bytes
        }
    };

    tx.send((bytes, addr))
        .await
        .map_err(ProcessQueryError::SendChannel)?;

    Ok(())
}
