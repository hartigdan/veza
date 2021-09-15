fn main() {
    env_logger::init();

    let server = std::net::TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        std::thread::spawn(move || {
            let mut websocket = tungstenite::server::accept(stream.unwrap()).unwrap();
            loop {
                let msg = tungstenite::protocol::Message::text("Hello");
                websocket.write_message(msg).unwrap();
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        });
    }
}