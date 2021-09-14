use cmd_lib::run_fun;
use log::debug;
use serde_json::json;
use std::thread;
use std::{io, time::Duration};
use structopt::StructOpt;

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

    loop {
        let val = run_fun!(
            bash -c ${input}
        )
        .expect("error");

        let msg = json!({"id": args.id, "value": val.trim()});
        println!("{}", msg);

        thread::sleep(Duration::from_millis(500));
    }
}
