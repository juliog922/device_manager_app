use crate::Error;
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};

/// Initializes the logging system with a rolling file appender and non-blocking logging.
///
/// This function sets up logging with the following features:
/// - Log entries are written to a file.
/// - Logs are rotated (i.e., archived) on an **hourly** basis.
/// - Log entries are formatted in **JSON**.
/// - The log files are stored in the `./logs` directory.
///
/// # Arguments
///
/// - `filename_prefix`: A string slice specifying the prefix for log file names. Each log file
///   will start with this prefix and be followed by a timestamp indicating the rotation time.
///
/// # Returns
///
/// This function returns a `Result<WorkerGuard, Error>`, where:
/// - `WorkerGuard`: A guard that ensures the background logging task continues to run for
///   non-blocking logging. **You must retain this in your application** to avoid losing log entries.
/// - `Error`: An error returned if the rolling file appender cannot be initialized, preventing
///   logging setup from completing.
///
/// # Panics
///
/// The function will panic if it fails to initialize the rolling file appender.
/// This can occur if there's an issue with file creation or access to the log directory.
pub fn logging_init_setup(filename_prefix: &str) -> Result<WorkerGuard, Error> {
    // Create a rolling file appender that automatically rotates log files every hour.
    // The log files will be named using the provided `filename_prefix` and stored in the `./logs` directory.
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::HOURLY) // Rotate log files on an hourly basis.
        .filename_prefix(filename_prefix) // Set the file prefix for the log file.
        .build("./logs") // Log files are saved in the './logs' directory.
        .map_err(|err| Error::Custom(format!("Failed to initialize log file: {}", err)))?; // Return an error if setup fails.

    // Create a non-blocking logger using the rolling file appender.
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // Configure the tracing subscriber:
    // - Use non-blocking logging to avoid blocking the main thread.
    // - Log messages are formatted in JSON format.
    tracing_subscriber::fmt()
        .with_writer(non_blocking) // Use the non-blocking logger to write log messages.
        .json() // Log messages are formatted as JSON.
        .init(); // Activate the logging system with this configuration.

    // Return the guard, which ensures that logging continues in the background.
    Ok(guard)
}
