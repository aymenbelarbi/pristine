//! Logging initialization

/// Initialize logging with the given filter
pub fn init_logging(filter: &str) {
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .init();
}

/// Initialize logging with JSON output
pub fn init_logging_json(filter: &str) {
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .json()
        .init();
}
