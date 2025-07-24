use std::collections::VecDeque;
use std::sync::Arc;
use accounts_manager::JwtToken;
use crate::backend_logic::BackendLogic;
use crate::gui::commands::GuiCommand;
use crate::gui::{LoginData, LoginFailedReason, RegisterData, RegisterFailedReason};
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
            self.request_gui_command(GuiCommand::ShowShutdownDialog);
        }

        // Examin managers state
        match &mut self.state {
            GuiState::ServerCheckingInProgress => {
                let server_status_result = self.backend_logic.fetch_server_status();

                let gcmd = match server_status_result {
                    Ok(server_status) => {
                        tracing::info!("Server is on, motd='{}'", server_status.motd);
                        GuiCommand::ServerOn { motd: server_status.motd }
                    },
                    Err(error) => {
                        tracing::error!("Server checking status failed!");
                        GuiCommand::ServerOff { reason: error.to_string() }
                    },
                };
                self.request_gui_command(gcmd);
            },
            GuiState::ServerIsOff { reason } => {},
            GuiState::ServerIsOk { motd, state} => match state {
                GuiStateServerOk::Login(state_login) => match state_login {
                    GuiStateLogin::ProcessingData(login_data) => {
                        // Should enter here after clicking login
                        tracing::info!("Login - processing");
                        let login_account_result = self.backend_logic.request_login_to_account(login_data.clone());
                        let gcmd = match login_account_result {
                            Ok(token) => {
                                tracing::info!("Login account '{}' success!", login_data.username);
                                GuiCommand::LoginSuccess((login_data.username.clone(), token))
                            },
                            Err(error) => {
                                tracing::error!("Login account '{}' failed! reason '{}'", login_data.username, error.to_string());
                                GuiCommand::LoginFailed(LoginFailedReason {
                                    username: login_data.username.clone(),
                                    reason: error.to_string()
                                })
                            }
                        };
                        self.request_gui_command(gcmd);
                    },
                    GuiStateLogin::Success(loggedin) => {
                        tracing::info!("Loggedin - success! {loggedin:?}");
                    },
                    _ => {}
                },
                GuiStateServerOk::Register(state_register) => match state_register {
                    GuiStateRegister::ProcessingData(register_data) => {
                        // Should enter here after clicking register
                        tracing::info!("Register - processing");
                        let register_account_result = self.backend_logic.request_register_new_account(register_data.clone());
                        let gcmd = match register_account_result {
                            Ok(_) => {
                                tracing::info!("Register account '{}' success!", register_data.username);
                                GuiCommand::RegisterSuccess(register_data.username.clone())
                            },
                            Err(error) => {
                                tracing::error!("Register account '{}' failed!", register_data.username);
                                GuiCommand::RegisterFailed(RegisterFailedReason {
                                    username: register_data.username.clone(),
                                    reason: error.to_string()
                                })
                            }
                        };
                        self.request_gui_command(gcmd);
                    },
                    _ => {}
                },
            }
        }

        // Process commands
        // Can change state of manager
        while let Some(gcmd) = self.cmds_queue.pop_front() {
            match gcmd {
                GuiCommand::ServerOff { reason } => {
                    self.state = GuiState::ServerIsOff { reason};
                },
                GuiCommand::ServerOn { motd } => {
                    self.state = GuiState::ServerIsOk {
                        motd,
                        state: GuiStateServerOk::Login(GuiStateLogin::EnteringData(LoginData::default())),
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
                GuiCommand::EnterLoginView(login_data) => {
                    if let GuiState::ServerIsOk { motd, state } = &mut self.state {
                        self.state = GuiState::ServerIsOk {
                            motd: motd.clone(),
                            state: GuiStateServerOk::Login(GuiStateLogin::EnteringData(login_data.unwrap_or_default()))
                        }
                    } else {
                        tracing::warn!("Bad state")
                    }
                },
                GuiCommand::EnterRegisterView => {
                    if let GuiState::ServerIsOk { motd, state } = &mut self.state {
                        self.state = GuiState::ServerIsOk {
                            motd: motd.clone(),
                            state: GuiStateServerOk::Register(GuiStateRegister::EnteringData(RegisterData::default()))
                        }
                    } else {
                        tracing::warn!("Bad state")
                    }
                },
                GuiCommand::PassLoginData(login_data) => {
                    if let GuiState::ServerIsOk { motd, state } = &mut self.state {
                        self.state = GuiState::ServerIsOk {
                            motd: motd.clone(),
                            state: GuiStateServerOk::Login(GuiStateLogin::ProcessingData(login_data))
                        }
                    } else {
                        tracing::warn!("Bad state")
                    }
                },
                GuiCommand::LoginFailed(reason) => {
                    if let GuiState::ServerIsOk { motd, state } = &mut self.state {
                        self.state = GuiState::ServerIsOk {
                            motd: motd.clone(),
                            state: GuiStateServerOk::Login(GuiStateLogin::Failed(reason))
                        }
                    } else {
                        tracing::warn!("Bad state")
                    }
                },
                GuiCommand::LoginSuccess((username, jwt_token)) => {
                    if let GuiState::ServerIsOk { motd, state } = &mut self.state {
                        self.state = GuiState::ServerIsOk {
                            motd: motd.clone(),
                            state: GuiStateServerOk::Login(GuiStateLogin::Success(GuiStateLoggedin { username, jwt_token }))
                        }
                    } else {
                        tracing::warn!("Bad state")
                    }
                },
                GuiCommand::PassRegisterData(register_data) => {
                    if let GuiState::ServerIsOk { motd, state } = &mut self.state {
                        self.state = GuiState::ServerIsOk {
                            motd: motd.clone(),
                            state: GuiStateServerOk::Register(GuiStateRegister::ProcessingData(register_data))
                        }
                    } else {
                        tracing::warn!("Bad state")
                    }
                },
                GuiCommand::RegisterFailed(reason) => {
                    if let GuiState::ServerIsOk { motd, state } = &mut self.state {
                        self.state = GuiState::ServerIsOk {
                            motd: motd.clone(),
                            state: GuiStateServerOk::Register(GuiStateRegister::Failed(reason))
                        }
                    } else {
                        tracing::warn!("Bad state")
                    }
                },
                GuiCommand::RegisterSuccess(username) => {
                    if let GuiState::ServerIsOk { motd, state } = &mut self.state {
                        self.state = GuiState::ServerIsOk {
                            motd: motd.clone(),
                            state: GuiStateServerOk::Register(GuiStateRegister::Success(username))
                        }
                    } else {
                        tracing::warn!("Bad state")
                    }
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
    },

    // TODO probably states related to each view -> if same data it will be transfered during view construction
    ServerIsOk {
        motd: String,
        state: GuiStateServerOk,
    },
}

#[derive(Debug)]
pub enum GuiStateServerOk {
    Login(GuiStateLogin),
    Register(GuiStateRegister),
}

#[derive(Debug)]
pub enum GuiStateLogin {
    EnteringData(LoginData),
    ProcessingData(LoginData),
    Failed(LoginFailedReason), // need some option to collect notifications, ACK come back to entering data
    Success(GuiStateLoggedin), // meybe add inner data
}

#[derive(Debug)]
pub enum GuiStateRegister {
    EnteringData(RegisterData),
    ProcessingData(RegisterData),
    Failed(RegisterFailedReason), // need some option to collect notifications, ACK come back to entering data
    Success(String), // Back to login option
}

#[derive(Debug)]
pub struct GuiStateLoggedin {
    pub username: String,
    pub jwt_token: JwtToken,
}