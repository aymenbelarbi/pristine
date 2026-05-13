//! Tracing initialization

/// Initialize tracing with the given filter
pub fn init_tracing(filter: &str) {
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .init();
}

/// Initialize tracing with JSON output for production
pub fn init_tracing_json(filter: &str) {
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .json()
        .init();
}
