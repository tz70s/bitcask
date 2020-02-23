//! Main entry binary for running server side bitcask application.
//! To simplify the client-side operation, we use an asynchronous http-based server.

use slog::{error, info};

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    let logger = bitcask::logger::Logger::new();
    info!(logger.log, "Initialize environment for bitcask server");

    // load config
    let config = match bitcask::Config::file("conf/default.json") {
        Ok(c) => c,
        Err(e) => {
            error!(logger.log, "Can't load bitcask configuration, please check again"; "error" => %e);
            return Err(e);
        }
    };

    info!(logger.log, "Bitcask configurations"; "host" => &config.host, "port" => &config.port);

    bitcask::server::run(logger, config).await?;

    Ok(())
}
