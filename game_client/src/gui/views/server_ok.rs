use macroquad::hash;
use macroquad::math::Vec2;
use macroquad::ui::{root_ui, widgets};
use crate::gui::commands::GuiCommand;
use crate::gui::{LoginData, RegisterData};

pub fn show_server_ok_login(
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
                println!("Attempting to login with user={}, psswd={}",
                         login_data.username,
                         login_data.password
                );
                output_cmd = Some(GuiCommand::PassLoginData(login_data.clone()))
            } else if ui.button(None, "Register") {
                println!("Changing view to register");
                output_cmd = Some(GuiCommand::EnterRegisterView)
            }
        });

    output_cmd
}

pub fn show_server_ok_register(
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
            ui.input_password(hash!(), "Password", &mut register_data.password);
            ui.input_password(hash!(), "Password repeated", &mut register_data.password_repeated);

            if ui.button(None, "Register") {
                println!("Attempting to register with user={}, psswd={}/{}",
                         register_data.username,
                         register_data.password,
                         register_data.password_repeated,
                );
                output_cmd = Some(GuiCommand::PassRegisterData(register_data.clone()))
            }

            if ui.button(None, "Login") {
                println!("Changing view to login");
                output_cmd = Some(GuiCommand::EnterLoginView)
            }
        });

    output_cmd
}

