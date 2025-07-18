use crate::gui::commands::GuiCommand;
use crate::gui::manager::{GuiManager, GuiState};
use crate::gui::settings::GuiSettings;
use crate::gui::views::launcher::launcher_skin;
use macroquad::hash;
use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets, Skin};
use std::ops::Mul;

pub mod launcher;

pub struct GuiRenderer {
    launcher_skin: Skin,
    // show_exit_dialog: bool,
}

const DEFAULT_LAUNCHER_WINDOW_SIZE: Vec2 = vec2(400.0, 280.0);

impl GuiRenderer {
    pub fn new() -> Self {
        Self {
            launcher_skin: launcher_skin(),
        }
    }

    fn get_launcher_window_scaled_size(gui_settings: &GuiSettings) -> Vec2 {
        DEFAULT_LAUNCHER_WINDOW_SIZE.mul(gui_settings.scale)
    }

    fn get_launcher_window_scaled_position(gui_settings: &GuiSettings) -> Vec2 {
        let launcher_window_scaled_size = Self::get_launcher_window_scaled_size(gui_settings);
        vec2(
            (screen_width() - launcher_window_scaled_size.x) / 2.0,
            (screen_height() - launcher_window_scaled_size.y) / 2.0,
        )
    }

    // Just draw state of manager
    pub fn update_draw_gui(&self, gui_manager: &mut GuiManager) {
        let window_position = Self::get_launcher_window_scaled_position(&gui_manager.gui_settings);
        let window_size = Self::get_launcher_window_scaled_size(&gui_manager.gui_settings);

        root_ui().push_skin(&self.launcher_skin);

        // TODo do it better
        let mut gui_command_to_send = None;

        match &gui_manager.state {
            GuiState::ServerCheckingInProgress => {
                widgets::Window::new(hash!(), window_position, window_size)
                    .label("Launcher")
                    .movable(false)
                    .titlebar(true)
                    .ui(&mut *root_ui(), |ui| {
                        ui.label(None, "Checking account manager server")
                    });
            },

            GuiState::ServerIsOff { reason, was_acked: _ } => {
                widgets::Window::new(hash!(), window_position, window_size)
                    .label("Launcher - Server Offline")
                    .movable(false)
                    .titlebar(true)
                    .ui(&mut *root_ui(), |ui| {
                        ui.label(None, &format!("Server is offline: '{reason}'"));
                        if ui.button(None, "Confirm Exit") {
                            gui_command_to_send = Some(GuiCommand::AckServerOffline);
                        }
                    });
            },

            GuiState::ServerIsOk { motd, state } => {
                widgets::Window::new(hash!(), window_position, window_size)
                    .label("Launcher")
                    .movable(false)
                    .titlebar(true)
                    .ui(&mut *root_ui(), |ui| {
                        ui.label(None, "Account manager server OK")
                    });
            }
        }

        if  let Some(gui_command_to_send) = gui_command_to_send {
            gui_manager.request_gui_command(gui_command_to_send);
        }


        // Draw confirmation popup if needed
        if gui_manager.should_show_exit_dialog() {
            let popup_size = vec2(screen_width() - 40.0, 120.0);
            let popup_pos = vec2(
                (screen_width() - popup_size.x) / 2.0,
                (screen_height() - popup_size.y) / 2.0,
            );

            widgets::Window::new(hash!(), popup_pos, popup_size)
                .label("Confirm Exit")
                .movable(false)
                .titlebar(true)
                .ui(&mut *root_ui(), |ui| {
                    ui.label(None, "Are you sure you want to exit?");
                    if ui.button(None, "Abort") {
                        gui_manager.request_gui_command(GuiCommand::AbortShutdownDialog);
                    }
                    if ui.button(None, "Exit") {
                        gui_manager.request_gui_command(GuiCommand::ProceedShutdownDialog);
                    }
                });
        }


        root_ui().pop_skin();
    }
}
