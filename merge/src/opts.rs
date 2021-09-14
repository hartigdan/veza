use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "0.1.0")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct CmdLineOpts {
    /// TCP address and port to listen on, e. g. 127.0.0.1:8080
    pub tcp_bind_address: String,
}
