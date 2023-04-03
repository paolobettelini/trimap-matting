use std::{fs::File, net::IpAddr};

mod args;
use args::*;

mod handler;

#[macro_use]
extern crate lazy_static;

const LOG_ENV: &str = "MATTING_LOG";
const LOG_ENV_STYLE: &str = "MATTING_LOG_STYLE";

pub struct Config {
    pub www: String,
    pub address: IpAddr,
    pub port: u16,
    pub log_file: Option<String>,
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let args = Args::parse();

        Config {
            www: path_str!(args.www),
            address: args.address,
            port: args.port,
            log_file: args.log_file.map(|log_file| path_str!(log_file)),
        }
    };
}

#[tokio::main]
async fn main() {
    // Construct logger
    if std::env::var(LOG_ENV).is_err() {
        std::env::set_var(LOG_ENV, "info");
    }

    let env = env_logger::Env::new()
        .filter(LOG_ENV)
        .write_style(LOG_ENV_STYLE);

    if let Some(log_file) = &CONFIG.log_file {
        let target = if let Ok(file) = File::create(log_file) {
            Box::new(file)
        } else {
            // Init the logger without the file
            env_logger::init_from_env(env);
            log::error!("Could not create the log file");
            return;
        };

        env_logger::Builder::from_env(env)
            .target(env_logger::Target::Pipe(target))
            .init();
    } else {
        env_logger::init_from_env(env);
    }

    // Start service
    start_service(&CONFIG.www, CONFIG.address, CONFIG.port).await;
}

async fn start_service(www: &'static str, address: IpAddr, port: u16) {
    let routes = handler::get_routes(www);

    warp::serve(routes).run((address, port)).await;
}
