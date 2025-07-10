use std::ops::Mul;
use macroquad::hash;
use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets, Skin};
use crate::gui::commands::GuiCommand;
use crate::gui::manager::{GuiManager, GuiState};
use crate::gui::settings::GuiSettings;
use crate::gui::views::launcher::launcher_skin;

pub mod launcher;

pub struct GuiRenderer {
    launcher_skin: Skin,
}

const DEFAULT_LAUNCHER_WINDOW_SIZE: Vec2 = vec2(400.0, 280.0);

impl GuiRenderer {
    pub async fn new() -> Self {
        Self {
            launcher_skin: launcher_skin().await
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
    
    pub fn update(&mut self, gui_manager: &mut GuiManager) {
        if is_quit_requested() {
            // TODO add closing dialog
            // self.close_requested = true
            gui_manager.request_gui_command(GuiCommand::Shutdown);
        }
    }

    pub fn draw_gui(&self, gui_manager: &GuiManager) {
        
        let window_position = Self::get_launcher_window_scaled_position(&gui_manager.gui_settings);
        let window_size = Self::get_launcher_window_scaled_size(&gui_manager.gui_settings);

        root_ui().push_skin(&self.launcher_skin);

        match &gui_manager.state {
            GuiState::ServerCheckInProgress => {
                widgets::Window::new(
                    hash!(),
                    window_position,
                    window_size
                )
                    .label("Login")
                    .movable(false)
                    .titlebar(true)
                    .ui(&mut *root_ui(), |ui| {
                        ui.label(None, "Hello")
                    });
            },

            GuiState::ServerOff {reason} => {
                //only ack to terminate
            },

            GuiState::ServerOk(_server_ok_state) => {

            },
        }

        root_ui().pop_skin();
    }
}