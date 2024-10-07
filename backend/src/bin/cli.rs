use tracing::*;


fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG) // Capture debug messages
        .init();
    debug!("This is a debug message!"); // Log a debug message
}