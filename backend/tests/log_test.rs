use backend::setup::log_setup::logging_init_setup;
use std::fs;
use std::thread;
use std::time::Duration;
use tracing::*;

/// This test checks whether the logging setup correctly logs the expected messages
/// into a log file, and ensures that any log files created during the test are
/// cleaned up afterward.

#[test]
fn test_log_file() {
    // Initialize the logging system with the "test.log" filename prefix.
    // This will create a log file in the "./logs" directory.
    {
        let _guard = logging_init_setup("test.log");

        // Log an info message
        info!("This is an info message!");
        // Log a warning message
        warn!("This is a warning message!");
        // Log an error message
        error!("This is an error message!");
    }

    // Define a closure for cleanup to ensure log files containing "test.log" in the name are deleted.
    let cleanup = || {
        let log_dir = "./logs"; // The directory where logs are stored
        let log_file_pattern = "test.log"; // The log file prefix we're targeting for deletion

        // Iterate over all files in the log directory and delete any file containing "test.log" in its name.
        if let Ok(entries) = fs::read_dir(log_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let file_name = entry.file_name();
                if file_name.to_string_lossy().contains(log_file_pattern) {
                    let file_path = entry.path();
                    // Attempt to remove the file, and log any errors that occur.
                    if let Err(e) = fs::remove_file(&file_path) {
                        eprintln!("Failed to delete log file {:?}: {}", file_path, e);
                    }
                }
            }
        }
    };

    // CleanupGuard is used to ensure that cleanup happens when the test finishes,
    // whether it passes or fails (i.e., during a panic).
    struct CleanupGuard<F: FnOnce()>(Option<F>);

    impl<F: FnOnce()> Drop for CleanupGuard<F> {
        fn drop(&mut self) {
            if let Some(cleanup) = self.0.take() {
                cleanup(); // Call the cleanup closure when this guard is dropped
            }
        }
    }

    // Create a cleanup guard, which ensures the cleanup code is run after the test completes.
    let _cleanup_guard = CleanupGuard(Some(cleanup));

    // Ensure that log messages are written to the log file by adding a brief sleep.
    // This gives the log system enough time to flush the messages to disk.
    thread::sleep(Duration::from_secs(1));

    // Path to the log directory and the file we're testing
    let log_dir = "./logs";
    let log_file_pattern = "test.log";

    // Search the log directory for any file that contains "test.log" in its name
    let log_file_path = fs::read_dir(log_dir)
        .expect("Failed to read log directory")
        .filter_map(|entry| entry.ok()) // Ignore invalid entries
        .find(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .contains(log_file_pattern)
        })
        .expect("Log file not found")
        .path(); // Get the path of the found log file

    // Read the content of the log file
    let log_content = fs::read_to_string(&log_file_path).expect("Failed to read log file");

    // Verify that the log file contains the expected log messages in JSON format.
    // These are the log messages that we logged earlier with info!, warn!, and error!.
    assert!(
        log_content
            .contains(r#""fields":{"message":"This is an info message!"},"target":"log_test""#),
        "Log file does not contain the expected info message"
    );
    assert!(
        log_content
            .contains(r#""fields":{"message":"This is a warning message!"},"target":"log_test""#),
        "Log file does not contain the expected warning message"
    );
    assert!(
        log_content
            .contains(r#""fields":{"message":"This is an error message!"},"target":"log_test""#),
        "Log file does not contain the expected error message"
    );
}
