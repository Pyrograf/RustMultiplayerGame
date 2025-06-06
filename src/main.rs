use std::cell::RefCell;
use std::rc::Rc;
use macroquad::prelude::*;

use RustMultiplayerGame::gui::{constrain_screen_size, GuiSettings, launcher::GuiLauncher};

#[macroquad::main("Client")]
async fn main() {
    let gui_settings = Rc::new(RefCell::new(GuiSettings::default()));

    let mut gui_launcher = GuiLauncher::new(gui_settings.clone()).await;

    loop {
        clear_background(GRAY);

        constrain_screen_size();

        gui_launcher.update_and_draw();

        next_frame().await;
    }
}