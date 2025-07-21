pub mod manager;
pub mod views;
pub mod commands;
pub mod settings;
mod skins;
mod widgets;

use macroquad::math::{vec2, Vec2};
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::{screen_height, screen_width};
use crate::gui::manager::{GuiManager, GuiState, GuiStateLogin, GuiStateRegister, GuiStateServerOk};
use crate::gui::settings::GuiSettings;
use macroquad::prelude::*;
use macroquad::ui::{root_ui, Skin};
use std::ops::Mul;
use crate::gui::skins::common_skin;
use crate::gui::views::{show_server_checking, show_server_off, show_server_ok_login, show_server_ok_register};
use crate::gui::widgets::show_exit_pupup;

#[derive(Debug, PartialOrd, PartialEq, Default, Clone)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Debug, PartialOrd, PartialEq, Default, Clone)]
pub struct RegisterData {
    pub username: String,
    pub password: String,
    pub password_repeated: String,
}

/// Infos:
/// * Screen - entire viewport
/// * Window - top level GUI container
const WINDOW_MINIMAL_SIZE: Vec2 = vec2(800.0, 600.0);

pub fn constrain_screen_size() {
    if screen_width() < WINDOW_MINIMAL_SIZE.x || screen_height() < WINDOW_MINIMAL_SIZE.y {
        set_window_size(
            screen_width().max(WINDOW_MINIMAL_SIZE.x).round() as u32 + 1,
            screen_height().max(WINDOW_MINIMAL_SIZE.y).round() as u32 + 1
        );
    }
}

pub struct GuiRenderer {
    common_skin: Skin,
}

const DEFAULT_LAUNCHER_WINDOW_SIZE: Vec2 = vec2(400.0, 280.0);

impl GuiRenderer {
    pub fn new() -> Self {
        Self {
            common_skin: common_skin(),
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
        // Start drawing
        clear_background(GRAY);

        constrain_screen_size();

        let window_position = Self::get_launcher_window_scaled_position(&gui_manager.gui_settings);
        let window_size = Self::get_launcher_window_scaled_size(&gui_manager.gui_settings);

        // Select skin
        root_ui().push_skin(&self.common_skin);

        // Draw view based on GUIManager state
        let gui_command_to_send = match &mut gui_manager.state {
            GuiState::ServerCheckingInProgress => {
                show_server_checking(window_position, window_size)
            },

            GuiState::ServerIsOff { reason, was_acked: _ } => {
                show_server_off(window_position, window_size, reason)
            },

            // TODO complete
            GuiState::ServerIsOk { motd, state } => match state {
                GuiStateServerOk::Login(state_login) => match state_login {
                    GuiStateLogin::EnteringData(login_data) => {
                        show_server_ok_login(window_position, window_size, motd, login_data)
                    },
                    _ => None,
                },
                GuiStateServerOk::Register(state_register) => match state_register {
                    GuiStateRegister::EnteringData(register_data) => {
                        show_server_ok_register(window_position, window_size, motd, register_data)
                    },
                    _ => None,
                },
            }

        };

        if let Some(gui_command_to_send) = gui_command_to_send {
            gui_manager.request_gui_command(gui_command_to_send);
        }

        // Draw confirmation popup if needed
        if gui_manager.should_show_exit_dialog() {
            let popup_size = vec2(screen_width() - 40.0, 120.0);
            let popup_pos = vec2(
                (screen_width() - popup_size.x) / 2.0,
                (screen_height() - popup_size.y) / 2.0,
            );

            if let Some(show_exit_cmd) = show_exit_pupup(popup_pos, popup_size) {
                gui_manager.request_gui_command(show_exit_cmd);
            }
        }

        root_ui().pop_skin();
    }
}
