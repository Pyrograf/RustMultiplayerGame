use macroquad::hash;
use macroquad::math::Vec2;
use macroquad::ui::{root_ui, widgets};
use crate::gui::commands::GuiCommand;

pub fn show_exit_pupup(popup_pos: Vec2, popup_size: Vec2) -> Option<GuiCommand> {
    let mut output_cmd = None;

    widgets::Window::new(hash!(), popup_pos, popup_size)
        .label("Confirm Exit")
        .movable(false)
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, "Are you sure you want to exit?");
            if ui.button(None, "Abort") {
                // gui_manager.request_gui_command(GuiCommand::AbortShutdownDialog);
                output_cmd = Some(GuiCommand::AbortShutdownDialog);
            }
            if ui.button(None, "Exit") {
                // gui_manager.request_gui_command(GuiCommand::ProceedShutdownDialog);
                output_cmd  = Some(GuiCommand::ProceedShutdownDialog);
            }
        });

    output_cmd
}