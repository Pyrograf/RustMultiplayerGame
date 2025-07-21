use macroquad::hash;
use macroquad::math::Vec2;
use macroquad::ui::{root_ui, widgets};
use crate::gui::commands::GuiCommand;

pub fn show_server_off(window_position: Vec2, window_size: Vec2, reason: &str) -> Option<GuiCommand> {
    let mut output_cmd = None;

    widgets::Window::new(hash!(), window_position, window_size)
        .label("Launcher - Server Offline")
        .movable(false)
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, &format!("Server is offline: '{reason}'"));
            if ui.button(None, "Confirm Exit") {
                output_cmd = Some(GuiCommand::AckServerOffline);
            }
        });

    output_cmd
}