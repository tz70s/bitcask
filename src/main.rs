//! Main entry binary for running server side bitcask application.
//! To simplify the client-side operation, we use an asynchronous http-based server.

use slog::{error, info};

fn main() {
    let logger = bitcask::logger::Logger::new();
    info!(logger.log, "Start bitcask server");

    // load config
    let config = match bitcask::Config::file("conf/default.json") {
        Ok(c) => c,
        Err(e) => {
            error!(logger.log, "Can't load bitcask configuration, please check again"; "error" => e.to_string());
            return;
        }
    };

    info!(logger.log, "Bitcask configurations"; "host" => config.host, "port" => config.port);
}
