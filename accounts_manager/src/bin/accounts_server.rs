use std::sync::Arc;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use user_management::AccountsManagerServer;

#[tokio::main]
async fn main() {
    println!("Accounts server!");
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("debug"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let server = AccountsManagerServer::run().await.unwrap();

    let ctrlc_notify = Arc::new(tokio::sync::Notify::new());
    let ctrlc_notify_shared = ctrlc_notify.clone();

    ctrlc::set_handler(move || {
        ctrlc_notify_shared.notify_one();
    }).expect("Error setting Ctrl-C handler");

    // Wait until Ctrl+C is triggered
    ctrlc_notify.notified().await;

    server.shutdown_gracefully_await().await.unwrap();
}
