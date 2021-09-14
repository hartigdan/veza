use structopt::StructOpt;
use std::io;
use log::debug;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long)]
    debug: bool,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to parse");

    debug!("input: {}", input);
    let args = Cli::from_args();

    println!("{}", input);
}
