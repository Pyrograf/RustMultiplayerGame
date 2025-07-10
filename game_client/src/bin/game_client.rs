use macroquad::prelude::*;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use game_client::backend_logic::BackendLogic;
use game_client::gui::constrain_screen_size;
use game_client::gui::manager::GuiManager;
use game_client::gui::views::GuiRenderer;

#[macroquad::main("Client")]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("debug"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let backend_logic = BackendLogic::run();
    let mut gui_manager = GuiManager::new();

    let mut gui_renderer = GuiRenderer::new().await;
    
    prevent_quit();
    
    loop {
        // Common graphics related stuff
        clear_background(GRAY);

        constrain_screen_size();

        // GUI update & draw
        gui_manager.update();

        gui_renderer.update(&mut gui_manager);

        gui_renderer.draw_gui(&gui_manager);

        if gui_manager.is_close_requested() {
            break;
        }

        next_frame().await;
    }

    // TODO close backend_logic

    tracing::info!("Game client ended");
    println!("Game client endedasdasdas");
}