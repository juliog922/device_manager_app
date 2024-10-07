use tracing_appender::{non_blocking::WorkerGuard, rolling::{RollingFileAppender, Rotation}};

/// Initializes the logging system with a rolling file appender.
/// 
/// This function sets up the logging configuration, creating a rolling file appender
/// that writes log messages to a file. The logs are written in JSON format and rotated
/// hourly. The logs are stored in the `./logs` directory.
/// 
/// # Arguments
/// 
/// * `filename_prefix` - A string slice that sets the prefix for the log file names.
/// 
/// # Returns
/// 
/// A `WorkerGuard` that keeps the background logging task alive for non-blocking logging.
/// Make sure to hold onto this guard in your application to avoid losing log entries.
///
/// # Panics
/// 
/// This function will panic if the rolling file appender cannot be initialized.
pub fn logging_init_setup(filename_prefix: &str) -> WorkerGuard {

    // Create a rolling file appender with hourly rotation and the given filename prefix.
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::HOURLY)  // Rotate the logs every hour.
        .filename_prefix(filename_prefix)  // Set the prefix for the log file.
        .build("./logs")  // Store log files in the ./logs directory.
        .expect("failed to initialize rolling file appender");  // Panic if file appender setup fails.

    // Create a non-blocking logger with the rolling file appender.
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // Set up the tracing subscriber with non-blocking writing and JSON formatted output.
    tracing_subscriber::fmt()
        .with_writer(non_blocking)  // Use the non-blocking writer for logging.
        .json()  // Format the logs in JSON format.
        .init();  // Initialize the subscriber to begin logging.

    // Return the guard to keep the logging task alive.
    guard
}
