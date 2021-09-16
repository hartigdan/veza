use std::io::BufRead;
use std::io::BufReader;
use std::sync::mpsc::channel;

fn main() -> anyhow::Result<()> {
    let (tx, rx): (
        std::sync::mpsc::Sender<String>,
        std::sync::mpsc::Receiver<String>,
    ) = channel();

    // read JSON from mux line by line

    std::thread::spawn(move || -> anyhow::Result<()> {
        let input = std::net::TcpListener::bind("localhost:8081")?;
        for stream in input.incoming() {
            let peer_tx = tx.clone();

            let client_buffered = BufReader::new(stream?);
            for l in client_buffered.lines() {
                peer_tx.send(l?)?;
            }
        }
        Ok(())
    });

    // write JSON line by line to a connecting websocket

    let output = std::net::TcpListener::bind("localhost:8765")?;
    for stream in output.incoming() {
        let mut websocket = tungstenite::server::accept(stream?)?;
        loop {
            let input = rx.recv()?;
            let msg = tungstenite::protocol::Message::text(input);
            websocket.write_message(msg)?;
        }
    }

    Ok(())
}
