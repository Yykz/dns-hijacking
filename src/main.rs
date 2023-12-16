use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

mod errors;

mod cli;
use cli::parse;

mod network;
use network::process_request;

type Sended = (Vec<u8>, SocketAddr);

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let options = parse();

    let sock = UdpSocket::bind("127.0.0.1:53").await?;
    let sock = Arc::new(sock);

    let opt = Arc::new(options);

    let (tx, mut rx) = mpsc::channel::<Sended>((1472 + 32) * 3);
    let s = sock.clone();
    tokio::spawn(async move {
        while let Some((bytes, addr)) = rx.recv().await {
            s.send_to(&bytes, &addr).await.unwrap();
        }
    });

    let mut buf = [0; 1472];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        let bytes = buf[..len].to_vec();
        let tx = tx.clone();
        let opt = opt.clone();
        tokio::spawn(async move {
            if let Err(err) = process_request(bytes, tx, addr, opt).await {
                eprintln!("{:?}", err);
            }
        });
    }
}
