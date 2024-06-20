pub mod github;

use scones::run;
use tracing::{event, Level};
use tracing_appender::rolling::{RollingFileAppender, Rotation};

#[tokio::main]
async fn main() {
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("gym-scones.log")
        .build("log") // write log files to the 'log' directory
        .expect("failed to initialize rolling file appender");

    let (file_non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let (stdout_non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
        .with_writer(file_non_blocking)
        .with_writer(stdout_non_blocking)
        .init();

    event!(Level::INFO, "Content path");

    run().await
}
