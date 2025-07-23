use macroquad::hash;
use macroquad::math::Vec2;
use macroquad::ui::{root_ui, widgets};
use crate::gui::commands::GuiCommand;
use crate::gui::{LoginData, RegisterData};

pub mod login {
    use crate::gui::LoginFailedReason;
    use super::*;

    pub fn show_login_entering_data(
        window_position: Vec2,
        window_size: Vec2,
        motd: &str,
        login_data: &mut LoginData,
    ) -> Option<GuiCommand> {
        let mut output_cmd = None;

        widgets::Window::new(hash!(), window_position, window_size)
            .label("Launcher - Login")
            .movable(false)
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, "Enter username and password to login to existing account");
                ui.label(None, &format!("Motd = '{motd}'"));

                ui.input_text(hash!(), "User", &mut login_data.username);
                ui.input_password(hash!(), "Password", &mut login_data.password);

                if ui.button(None, "Login") {
                    tracing::trace!("Attempting to login with user={}, psswd={}",
                         login_data.username,
                         login_data.password
                    );
                    output_cmd = Some(GuiCommand::PassLoginData(login_data.clone()));
                } else if ui.button(None, "Register") {
                    tracing::trace!("Changing view to register");
                    output_cmd = Some(GuiCommand::EnterRegisterView);
                }
            });

        output_cmd
    }

    pub fn show_login_processing_data(
        window_position: Vec2,
        window_size: Vec2,
        motd: &str,
        username: &str,
    ) -> Option<GuiCommand> {
        let mut output_cmd = None;

        widgets::Window::new(hash!(), window_position, window_size)
            .label("Launcher - Login")
            .movable(false)
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, &format!("Loggining to '{username}'"));
                ui.label(None, &format!("Motd = '{motd}'"));
                //TODO
            });

        output_cmd
    }

    pub fn show_login_failed(
        window_position: Vec2,
        window_size: Vec2,
        motd: &str,
        reason: &LoginFailedReason,
    ) -> Option<GuiCommand> {
        let mut output_cmd = None;

        widgets::Window::new(hash!(), window_position, window_size)
            .label("Launcher - Login")
            .movable(false)
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, &format!("Loggining to '{}' failed reason '{}'", reason.username, reason.reason));
                ui.label(None, &format!("Motd = '{motd}'"));
                //TODO
            });

        output_cmd
    }

    pub fn show_login_success(
        window_position: Vec2,
        window_size: Vec2,
        motd: &str,
        username: &str,
    ) -> Option<GuiCommand> {
        let mut output_cmd = None;

        widgets::Window::new(hash!(), window_position, window_size)
            .label("Launcher - Login")
            .movable(false)
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, &format!("Loggining to '{username}' success"));
                ui.label(None, &format!("Motd = '{motd}'"));
                //TODO
            });

        output_cmd
    }
}

pub mod register {
    use crate::gui::RegisterFailedReason;
    use crate::gui::views::{MINIMAL_PASSWORD_LENGTH, MINIMAL_USERNAME_LENGTH};
    use super::*;

    pub fn show_register_entering_data(
        window_position: Vec2,
        window_size: Vec2,
        motd: &str,
        register_data: &mut RegisterData,
    ) -> Option<GuiCommand> {
        let mut output_cmd = None;

        widgets::Window::new(hash!(), window_position, window_size)
            .label("Launcher - Register")
            .movable(false)
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, "Enter username and password to register new account");
                ui.label(None, &format!("Motd = '{motd}'"));

                ui.input_text(hash!(), "User", &mut register_data.username);

                // Username
                let username_ok = register_data.username.len() >= MINIMAL_USERNAME_LENGTH;
                let username_feedback_message = if username_ok {
                    "Username OK"
                } else {
                    "Username too short"
                };
                ui.label(None, username_feedback_message);

                // Passwords
                ui.input_password(hash!(), "Password", &mut register_data.password);
                ui.input_password(hash!(), "Password repeated", &mut register_data.password_repeated);

                let (password_feedback_message, password_ok) = if register_data.password.is_empty() {
                    ("Password empty", false)
                } else if register_data.password != register_data.password_repeated {
                    ("Passwords not match", false)
                } else if register_data.password.len() < MINIMAL_PASSWORD_LENGTH {
                    ("Passwords too short", false)
                } else {
                    ("Password OK", true)
                };
                ui.label(None, password_feedback_message);

                // Proceed with registration
                if username_ok && password_ok {
                    if ui.button(None, "Register") {
                        tracing::trace!("Attempting to register with user={}, psswd={}/{}",
                                 register_data.username,
                                 register_data.password,
                                 register_data.password_repeated,
                        );
                        output_cmd = Some(GuiCommand::PassRegisterData(register_data.clone()))
                    }
                }

                // Come back to login page
                if ui.button(None, "Login") {
                    tracing::trace!("Changing view to login");
                    output_cmd = Some(GuiCommand::EnterLoginView(None))
                }
            });

        output_cmd
    }

    pub fn show_register_processing_data(
        window_position: Vec2,
        window_size: Vec2,
        motd: &str,
        username: &str,
    ) -> Option<GuiCommand> {
        let mut output_cmd = None;

        widgets::Window::new(hash!(), window_position, window_size)
            .label("Launcher - Register")
            .movable(false)
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, &format!("Registering '{username}'"));
                ui.label(None, &format!("Motd = '{motd}'"));
                // Probably not visible to user due to blocking when creating account
                // TODO remove, and block inside inputing data,hide register buttonjust
            });

        output_cmd
    }

    pub fn show_register_failed(
        window_position: Vec2,
        window_size: Vec2,
        motd: &str,
        reason: &RegisterFailedReason,
    ) -> Option<GuiCommand> {
        let mut output_cmd = None;

        widgets::Window::new(hash!(), window_position, window_size)
            .label("Launcher - Register")
            .movable(false)
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, &format!("Registering '{}' failed '{}'", reason.username, reason.reason));
                ui.label(None, &format!("Motd = '{motd}'"));
                if ui.button(None, "Meh") {
                    tracing::trace!("Register failed acked");
                    output_cmd = Some(GuiCommand::EnterLoginView(None))
                }
            });

        output_cmd
    }

    pub fn show_register_success(
        window_position: Vec2,
        window_size: Vec2,
        motd: &str,
        username: &str,
    ) -> Option<GuiCommand> {
        let mut output_cmd = None;

        widgets::Window::new(hash!(), window_position, window_size)
            .label("Launcher - Register")
            .movable(false)
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, &format!("Registering '{username}' success"));
                ui.label(None, &format!("Motd = '{motd}'"));
                if ui.button(None, "Yay!") {
                    tracing::trace!("Register success proceed");
                    output_cmd = Some(GuiCommand::EnterLoginView(Some(LoginData {
                        username: username.to_string(),
                        password: String::new(),
                    })))
                }
            });

        output_cmd
    }
}
