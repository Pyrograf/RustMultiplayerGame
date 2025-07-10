use std::collections::VecDeque;
use macroquad::input::is_quit_requested;
use crate::gui::commands::GuiCommand;
use crate::gui::settings::GuiSettings;



// if is_quit_requested() {
// println!("Quit requested!");
// break;
// }

#[derive(Debug)]
pub struct GuiManager {
    pub state: GuiState,
    pub gui_settings: GuiSettings,
    close_requested: bool,
    cmds_queue: VecDeque<GuiCommand>,
}

impl GuiManager {
    pub fn new() -> Self {
        Self {
            state: GuiState::ServerCheckInProgress,
            gui_settings: GuiSettings::default(),
            close_requested: false,
            cmds_queue: VecDeque::new(),
        }
    }

    pub fn update(&mut self) {
        while let Some(gcmd) = self.cmds_queue.pop_front() {
            match gcmd {
                GuiCommand::ServerOff => {},
                GuiCommand::ServerOn => {},
                GuiCommand::Shutdown => {
                    self.close_requested = true;
                },
            }
        }
    }

    pub fn request_gui_command(&mut self, gcmd: GuiCommand) {
        self.cmds_queue.push_back(gcmd);
    }

    pub fn is_close_requested(&self) -> bool {
        self.close_requested
    }
}

/// State represents logic in user interaction flow,
/// not directly related to what user see.
#[derive(Debug)]
pub enum GuiState {
    /// Server unknown yet
    ServerCheckInProgress,

    ServerOff {
        reason: String,
    },

    ServerOk(GuiStateServerOk),
}

#[derive(Debug)]
pub enum GuiStateServerOk {
    /// Logi
    Login {
        username: String,
        password: String,
    },

    LoginInProcess,

    LoginFailed {
        reason: String,
    },

    Register {
        username: String,
        password: String,
    },

    RegisterInProcess,

    RegisterFailed {
        reason: String,
    },
}