use structopt::StructOpt;
use std::io;
use log::debug;
use serde_json::json;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long)]
    id: String,
}

fn main() {
    env_logger::init();
    let args = Cli::from_args();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to parse");
    debug!("input: {}", input.trim());

    let msg = json!({"id": id, "value": input.trim()});
    println!("{}", msg);
}
