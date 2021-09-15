use anyhow::Result;
use serde_json::json;
use socketcan::CANSocket;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long)]
    id: String,

    #[structopt(
        short,
        long,
        help = "Max count of received packets. Optional. With no count the program runs in an infinite loop"
    )]
    count: Option<u32>,

    #[structopt(help = "Can interface. Example: vcan0")]
    interface: String,

    #[structopt(help = "filter syntax: id:[byte:[bit]]. Optional")]
    filter: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    let socket = CANSocket::open(args.interface.as_str())?;
    let filter_str = args.filter.clone().unwrap_or("0".to_string());
    let filter_tuple: Vec<&str> = filter_str.split(':').collect();

    let mut count = args.count.unwrap_or(1);
    while count > 0 || args.count == None {
        let packet = socket.read_frame()?;
        let mut msg = json!({"id": args.id, "value": packet.data()});

        if args.filter != None {
            if packet.id() == filter_tuple[0].parse::<u32>()? {
                if filter_tuple.len() > 1 {
                    let byte = filter_tuple[1].parse::<usize>()?;
                    let data = packet.data();
                    let mut value = data[byte];

                    if filter_tuple.len() > 2 {
                        let bit = filter_tuple[2].parse::<u32>()?;
                        value = (value >> bit) & 1;
                    }
                    msg = json!({"id": args.id, "value": value});
                }
            }
        }

        println!("{}", msg);

        if args.count != None {
            count -= 1;
        }
    }
    Ok(())
}
