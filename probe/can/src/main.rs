use structopt::StructOpt;
use anyhow::Result;
use socketcan::CANSocket;
use serde_json::json;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long)]
    id: String,

    interface: String,

    #[structopt(help = "filter syntax: id:byte:bit")]
    filter: String,
}

fn main() -> Result<()>{
    let args = Cli::from_args();
    let socket = CANSocket::open(args.interface.as_str())?;
    let filter_tuple: Vec<&str> = args.filter.split(':').collect();
    loop
    {
        let packet = socket.read_frame()?;
        if packet.id() == filter_tuple[0].parse::<u32>()?
        {
            if filter_tuple.len() > 1
            {
                let byte = filter_tuple[1].parse::<usize>()?;
                let data = packet.data();
                let mut value = data[byte];

                if filter_tuple.len() > 2
                {
                    let bit = filter_tuple[2].parse::<u32>()?;
                    value = (value >> bit) & 1;
                }

                let msg = json!({"id": args.id, "value": value});
                println!("{}", msg);
            }
        }
    }
    Ok(())
}
