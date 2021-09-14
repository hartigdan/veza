use clap::Clap;
use std::error::Error;
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

mod opts;

/// Process a probe signal stream
async fn process(
    peer_tx: mpsc::UnboundedSender<String>,
    stream: tokio::net::TcpStream,
    _addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    // We expect full json objects, one per line.
    let mut lines = Framed::new(stream, LinesCodec::new());

    // Process incoming messages until our stream is exhausted by a disconnect.
    loop {
        tokio::select! {
            result = lines.next() => match result {
                Some(Ok(msg)) => {
                    peer_tx.send(msg)?;
                }
                Some(Err(e)) => {
                    eprintln!(
                        "an error occurred while processing messages; error = {:?}",
                        e
                    );
                }
                // The stream has been exhausted.
                None => break,
            },
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cmd_line_opts = opts::CmdLineOpts::parse();

    let listener = tokio::net::TcpListener::bind(&cmd_line_opts.tcp_bind_address).await?;

    // Create a channel where input from each client is collected and then written to stdout.
    let (tx, mut rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        loop {
            if let Some(msg) = rx.recv().await {
                println!("{}", msg);
            }
        }
    });

    loop {
        // Asynchronously wait for an inbound TcpStream.
        let (stream, addr) = listener.accept().await?;

        // Each client gets its own producer end of the mpsc channel.
        let peer_tx = tx.clone();

        tokio::spawn(async move {
            if let Err(e) = process(peer_tx, stream, addr).await {
                eprintln!("an error occurred; error = {:?}", e);
            }
        });
    }
}
