use clap::Clap;
use std::error::Error;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
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

    let (tx, mut rx): (
        mpsc::UnboundedSender<String>,
        mpsc::UnboundedReceiver<String>,
    ) = mpsc::unbounded_channel();

    // Create a client that pushes the output to the `serve` entity.

    let mut sender = tokio::net::TcpStream::connect(&cmd_line_opts.target).await?;
    tokio::spawn(async move {
        loop {
            if let Some(mut msg) = rx.recv().await {
                msg.push_str("\n");
                sender.write_all(msg.as_bytes()).await.unwrap();
            }
        }
    });

    // Create a server that accepts input from each probe.

    let listener = tokio::net::TcpListener::bind(&cmd_line_opts.bind).await?;
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
