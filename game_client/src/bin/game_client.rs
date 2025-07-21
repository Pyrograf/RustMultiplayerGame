use std::sync::Arc;
use macroquad::prelude::*;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use game_client::backend_logic::BackendLogic;
use game_client::gui::GuiRenderer;
use game_client::gui::manager::GuiManager;

#[macroquad::main("Client")]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("debug"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let backend_logic = Arc::new(BackendLogic::new());

    let mut gui_manager = GuiManager::new(backend_logic);

    let mut gui_renderer = GuiRenderer::new();

    prevent_quit();

    loop {
        // Process events queued in GUI manager
        gui_manager.update();

        // Updates internal state of GUI renderer
        // Captures events from GUI and forward them to GUI manager
        // Both can mutate because:
        // - GUI renderer changes its internal state
        // - GUI manager gets enqueued events
        // Just draw state of GUI manager
        gui_renderer.update_draw_gui(&mut gui_manager);

        if gui_manager.is_close_requested() {
            break;
        }

        next_frame().await;
    }
    tracing::info!("Game client ended");
}
