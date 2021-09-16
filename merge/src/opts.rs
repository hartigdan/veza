use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "0.1.0")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct CmdLineOpts {
    /// Address and port to listen on for TCP connections, e. g. 127.0.0.1:8888.
    #[clap(short, long, default_value = "127.0.0.1:8888")]
    pub bind: String,
    /// Address and port to forward merged data to via TCP.
    pub target: String,
}
