fn read_line_buffer() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let server = std::net::TcpListener::bind("localhost:8765").unwrap();
    for stream in server.incoming() {
        std::thread::spawn(move || {
            let mut websocket = tungstenite::server::accept(stream.unwrap()).unwrap();
            loop {
                let input = read_line_buffer();
                let msg = tungstenite::protocol::Message::text(input);
                websocket.write_message(msg).unwrap();
            }
        });
    }
}
