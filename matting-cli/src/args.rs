pub use clap::{ArgGroup, Parser};
pub use log::{info, error, warn};
use std::path::PathBuf;

/// Matting CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("mask_or_trimap")
        .required(true)
        .args(["mask", "trimap"]),
))]
#[command(group(
    ArgGroup::new("action")
        .required(false)
        .args(["fill", "transparent", "replace"]),
))]
pub struct Args {
    /// Target image
    #[arg(short = 'i', long)]
    pub target: PathBuf,

    /// Background mask image
    #[arg(long)]
    pub mask: Option<PathBuf>,

    /// Trimap image
    #[arg(long)]
    pub trimap: Option<PathBuf>,

    /// Save mask path
    #[arg(long)]
    pub save_mask: Option<Option<PathBuf>>,

    /// Output image
    #[arg(short, long, requires = "action")]
    pub output: Option<PathBuf>,

    /// Fill background action
    #[arg(short, long)]
    pub fill: Option<String>,

    /// Transparent background action
    #[arg(short, long)]
    pub transparent: bool,

    /// Replace background action
    #[arg(short, long)]
    pub replace: Option<PathBuf>,

    /// Verbose flag
    #[arg(long)]
    pub verbose: bool,
}

macro_rules! path_str {
    ($v:expr) => {
        String::from($v.to_string_lossy())
    };
}

pub(crate) use path_str;
