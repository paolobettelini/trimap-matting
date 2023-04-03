pub use clap::{ArgGroup, Parser};
use std::path::PathBuf;

pub use std::net::{IpAddr, Ipv4Addr};

/// Matting WEB
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// WWW static files folder
    #[arg(short, long)]
    pub www: PathBuf,

    /// Listening address
    #[arg(short, long, default_value_t = default_address())]
    pub address: IpAddr,

    /// Listening port
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,

    /// Log file
    #[arg(short, long)]
    pub log_file: Option<PathBuf>,
}

fn default_address() -> IpAddr {
    IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
}

macro_rules! path_str {
    ($v:expr) => {
        String::from($v.to_string_lossy())
    };
}

pub(crate) use path_str;
