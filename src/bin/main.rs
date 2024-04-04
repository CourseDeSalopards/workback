use std::{io::Write, sync::{Arc, Mutex}, thread};
use log::{error, warn, info, debug, trace, LevelFilter};

use dotenv::dotenv;

extern crate workback;

use prost_types::Value;
use workback::{
    colors, local_env, roomapi
};

use local_env::*;

#[macro_export]
macro_rules! loop_sleep {
    () => {
        if *LOOP_SLEEP_DURATION != 0 {
            std::thread::sleep(std::time::Duration::from_millis(*LOOP_SLEEP_DURATION));
        }
    };
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Load environment variables from .env file, TODO: only in debug mode.
    dotenv().ok();

    local_env::check_vars();

    // Initialize logger
    let level: LevelFilter = match RUST_ENV.as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    env_logger::Builder::new()
        .format(|buf, record| {
            let level = match record.level() {
                log::Level::Error => colors::RED,
                log::Level::Warn => colors::YELLOW,
                log::Level::Info => colors::GREEN,
                log::Level::Debug => colors::BLUE,
                log::Level::Trace => colors::CYAN,
            };
            writeln!(buf, "[{}{}{}] - {}", level, record.level(), colors::RESET, record.args())
        })
        .filter(None, level)
        .target(env_logger::Target::Stdout)
        .write_style(env_logger::WriteStyle::Always)
        .init();

    info!("workback ready");

    let pool: Vec<String> = Vec::new();
    let pool = Arc::new(Mutex::new(pool));
    
    let (tx, rx) = std::sync::mpsc::channel() as (std::sync::mpsc::Sender<String>, std::sync::mpsc::Receiver<String>);

    let players: Vec<String> = vec![];

    // match roomapi::roomapi::listen_to_event("playerJoined").await {
    //     Ok(response) => {
    //         info!("{:?}", response);
    //     },
    //     Err(_) => {
    //         info!("err osef")
    //     }
    // }

    match roomapi::roomapi::save_variable("test22", Value {
        kind: Some(prost_types::value::Kind::StringValue("abcdef".to_string()))
    }).await {
        Ok(response) => {
            info!("{:?}", response);
        },
        Err(_) => {
            error!("err osef")
        }
    }

    let pool = Arc::clone(&pool);
    loop {
        loop_sleep!();

        info!("Loop")

        // let mut pool = match pool.lock() {
        //     Ok(p) => p,
        //     Err(e) => {
        //         error!("Failed to lock: {}", e);
        //         continue;
        //     }
        // };
    }

    //TODO
}
