//! Default log bindings.

use slog::{self, o, Drain};

/// Logger encapsulates default configurations on slog.
/// It's basically an asynchronous, terminal based logger.
pub struct Logger {
    pub log: slog::Logger,
}

impl Logger {
    pub fn new() -> Self {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();

        let log = slog::Logger::root(drain, o!());
        Logger { log }
    }
}
