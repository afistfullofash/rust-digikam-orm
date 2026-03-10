use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

pub fn init_logging() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("none"));

    // JSON formatting layer for structured output
    let json_layer = fmt::layer()
        .json() // Output as JSON
        .with_current_span(true) // Include current span info
        .with_span_list(true) // Include full span hierarchy
        .with_thread_ids(true) // Include thread ID
        .with_thread_names(true) // Include thread name
        .with_file(true) // Include source file
        .with_line_number(true) // Include line number
        .with_target(true) // Include log target (module path)
        .with_span_events(FmtSpan::CLOSE) // Log when spans close
        .flatten_event(true); // Flatten event fields to top level

    // Initialize the global subscriber
    tracing_subscriber::registry()
        .with(env_filter)
        .with(json_layer)
        .init();
}
