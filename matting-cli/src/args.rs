pub use clap::Parser;
use std::path::PathBuf;

/// Matting CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Target image
    #[arg(short = 'i', long)]
    pub target: PathBuf,

    /// Trimap image
    #[arg(short = 'm', long)]
    pub trimap: PathBuf,

    /// Output image
    #[arg(short, long)]
    pub output: PathBuf,
}

macro_rules! path_str {
    ($v:expr) => {
        String::from($v.to_string_lossy())
    };
}

pub(crate) use path_str;
