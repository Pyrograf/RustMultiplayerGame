use macroquad::hash;
use macroquad::math::Vec2;
use macroquad::ui::{root_ui, widgets};
use crate::gui::commands::GuiCommand;

pub fn show_server_checking(window_position: Vec2, window_size: Vec2) -> Option<GuiCommand> {
    widgets::Window::new(hash!(), window_position, window_size)
        .label("Launcher")
        .movable(false)
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, "Checking account manager server")
        });
    None
}