use std::collections::VecDeque;
use std::sync::Arc;
use crate::backend_logic::BackendLogic;
use crate::gui::commands::GuiCommand;
use crate::gui::settings::GuiSettings;


#[derive(Debug)]
pub struct GuiManager {
    pub state: GuiState,
    pub gui_settings: GuiSettings,
    close_requested: bool,
    cmds_queue: VecDeque<GuiCommand>,
    backend_logic: Arc<BackendLogic>,
    show_exit_dialog: bool,
}

impl GuiManager {
    pub fn new(backend_logic: Arc<BackendLogic>) -> Self {
        Self {
            state: GuiState::ServerCheckingInProgress,
            gui_settings: GuiSettings::default(),
            close_requested: false,
            cmds_queue: VecDeque::new(),
            backend_logic,
            show_exit_dialog:  false,
        }
    }

    pub fn update(&mut self) {
        // Process in-state
        // Note: no changing state of manager, just emit commands
        if macroquad::input::is_quit_requested() {
            // TODO add closing dialog
            // self.close_requested = true
            self.request_gui_command(GuiCommand::ShowShutdownDialog);
        }

        match &mut self.state {
            GuiState::ServerCheckingInProgress => {
                let server_status_result = self.backend_logic.get_server_status();

                match server_status_result {
                    Ok(server_status) => {
                        tracing::info!("Server is on, motd='{}'", server_status.motd);
                        self.request_gui_command(GuiCommand::ServerOn { motd: server_status.motd });
                    },
                    Err(error) => {
                        tracing::error!("Server checking status failed!");
                        self.request_gui_command(GuiCommand::ServerOff { reason: "Status failed".to_owned() });
                    },
                }
            },
            GuiState::ServerIsOff { reason, was_acked } => {
                // TODO await user ack
                // tracing::error!("Server is off, termnating...");
                // self.request_gui_command(GuiCommand::Shutdown);
                if *was_acked {
                    self.close_requested = true;
                }

            },
            GuiState::ServerIsOk { motd, state} => {}
        }

        // Process commands
        // Can change state of manager
        while let Some(gcmd) = self.cmds_queue.pop_front() {
            match gcmd {
                GuiCommand::ServerOff { reason } => {
                    self.state = GuiState::ServerIsOff { reason, was_acked: false };
                },
                GuiCommand::ServerOn { motd } => {
                    self.state = GuiState::ServerIsOk {
                        motd,
                        state: GuiStateServerOk::Login {
                            username: "".to_owned(),
                            password: "".to_owned(),
                        },
                    };
                },
                GuiCommand::AckServerOffline => {
                    self.close_requested = true;
                },
                GuiCommand::ShowShutdownDialog => {
                    self.show_exit_dialog = true;
                },
                GuiCommand::AbortShutdownDialog => {
                    self.show_exit_dialog = false;
                },
                GuiCommand::ProceedShutdownDialog => {
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

    pub fn should_show_exit_dialog(&self) -> bool {
        self.show_exit_dialog
    }
}

/// State represents logic in user interaction flow,
/// not directly related to what user see.
#[derive(Debug)]
pub enum GuiState {
    ServerCheckingInProgress,

    ServerIsOff {
        reason: String,
        was_acked: bool,
    },

    // TODO probably states related to each view -> if same data it will be transfered during view construction
    ServerIsOk {
        motd: String,
        state: GuiStateServerOk,
    },
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