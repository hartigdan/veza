use anyhow::Result;
use assert_cmd::prelude::*; // Add methods on commands
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[test]
fn test_main() -> Result<()> {
    let pid = Arc::new(Mutex::new(None));
    {
        let pid2 = Arc::clone(&pid);

        std::thread::spawn(move || {
            let cmd = Command::cargo_bin("serve").unwrap().spawn().unwrap();
            let mut pid2 = pid2.lock().unwrap();
            *pid2 = Some(cmd);
        });
    }
    // Give the server time to start up
    std::thread::sleep(Duration::from_millis(1000));

    // connect is a blocking call
    let (mut _socket, _response) = tungstenite::client(
        url::Url::parse("ws://localhost:8765")?,
        std::net::TcpStream::connect("localhost:8765")?,
    )?;
    println!("Connected");

    // Kill the process where the server is running
    let mut pid = pid.lock().unwrap();
    (*pid).as_mut().unwrap().kill()?;
    Ok(())
}
