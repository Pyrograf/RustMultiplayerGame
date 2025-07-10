mod integrations;

use std::sync::OnceLock;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

static TESTS_TRACE_SETUP_INIT: OnceLock<()> = OnceLock::new();

pub fn tests_trace_setup() {
    let _ = TESTS_TRACE_SETUP_INIT.get_or_init(|| tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("debug"))
        .with(tracing_subscriber::fmt::layer())
        .init()
    );
}