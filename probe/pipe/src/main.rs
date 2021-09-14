use cmd_lib::run_fun;
use log::debug;
use serde_json::json;
use std::{io, time::Duration};
use structopt::StructOpt;
use std::thread;

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

    while(true)
    {
        let val = run_fun! (
            bash -c ${input}
        )
        .expect("error");

        let msg = json!({"id": args.id, "value": val.trim()});
        println!("{}", msg);

        thread::sleep(Duration::new(1, 0));
    }
}
