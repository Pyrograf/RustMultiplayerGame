use macroquad::hash;
use macroquad::math::Vec2;
use macroquad::ui::{root_ui, widgets};
use crate::gui::commands::GuiCommand;

pub fn show_server_ok_login(
    window_position: Vec2, 
    window_size: Vec2, 
    motd: &str,
) -> Option<GuiCommand> {
    widgets::Window::new(hash!(), window_position, window_size)
        .label("Launcher")
        .movable(false)
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, "Account manager server OK");
            ui.label(None, &format!("Motd = '{motd}'"));
        });
    None
}






















// use std::cell::RefCell;
// use std::fmt::{Debug, Formatter};
// use std::ops::Mul;
// use std::rc::Rc;
// use std::time::Duration;
// use macroquad::color::Color;
// use macroquad::hash;
// use macroquad::math::{vec2, RectOffset, Vec2};
// use macroquad::prelude::*;
// use macroquad::ui::{root_ui, widgets, Skin};
// use crate::gui::GuiSettings;
//
// const DEFAULT_LAUNCHER_WINDOW_SIZE: Vec2 = vec2(400.0, 280.0);



//
// pub fn update_and_draw(&mut self) {
//     // Position and size to center window in screen
//     let window_position = self.get_launcher_window_scaled_position();
//     let window_size = self.get_launcher_window_scaled_size();
//
//     // Start using launcher GUI skin
//     root_ui().push_skin(&self.skin);
//
//     match &mut self.recent_view {
//
//         GuiLauncherView::Login(login_data) => {
//             widgets::Window::new(
//                 hash!(),
//                 window_position,
//                 window_size
//             )
//                 .label("Login")
//                 .movable(false)
//                 .titlebar(true)
//                 .ui(&mut *root_ui(), |ui| {
//                     ui.input_text(hash!(), "User", &mut login_data.str_input_user);
//                     ui.input_password(hash!(), "Password", &mut login_data.str_input_password);
//
//                     if ui.button(None, "Login") {
//                         println!("Attempting to login with user={}, psswd={}",
//                                  login_data.str_input_user,
//                                  login_data.str_input_password
//                         );
//                         // TODO
//                     }
//
//                     if ui.button(None, "Register") {
//                         println!("Changing view to register");
//                         self.next_view = Some(GuiLauncherView::Register(RegisterViewData::default()))
//                     }
//                 });
//         }
//
//         GuiLauncherView::Register(register_data) => {
//             widgets::Window::new(
//                 hash!(),
//                 window_position,
//                 window_size
//             )
//                 .label("Register")
//                 .movable(false)
//                 .titlebar(true)
//                 .ui(&mut *root_ui(), |ui| {
//                     ui.input_text(hash!(), "User", &mut register_data.str_input_user);
//                     ui.input_password(hash!(), "Password", &mut register_data.str_input_password_1);
//                     ui.input_password(hash!(), "Password repeated", &mut register_data.str_input_password_2);
//
//                     if ui.button(None, "Register") {
//                         println!("Attempting to register with user={}, psswd={}/{}",
//                                  register_data.str_input_user,
//                                  register_data.str_input_password_1,
//                                  register_data.str_input_password_2,
//                         );
//                         std::thread::sleep(Duration::from_secs(1));
//                         // TODO
//                     }
//
//                     if ui.button(None, "Login") {
//                         println!("Changing view to login");
//                         self.next_view = Some(GuiLauncherView::Login(LoginViewData::default()))
//                     }
//                 });
//         }
//
//         GuiLauncherView::Error => {
//             widgets::Window::new(
//                 hash!(),
//                 window_position,
//                 window_size
//             )
//                 .label("Error")
//                 .movable(false)
//                 .titlebar(true)
//                 .ui(&mut *root_ui(), |ui| {
//                     if ui.button(None, "Back") {
//                         println!("Changing view to login");
//                         self.next_view = Some(GuiLauncherView::Login(LoginViewData::default()))
//                     }
//                 });
//         }
//
//     }
//
//     // Finish using launcher GUI skin
//     root_ui().pop_skin();
//
//     // Swap views
//     if let Some(next_view) = self.next_view.take() {
//         println!("Transition launcher view: {:?} -> {:?}.", self.recent_view, next_view);
//         self.recent_view = next_view;
//     }
//
// }









// pub struct GuiLauncher {
//     gui_settings: Rc<RefCell<GuiSettings>>,
//     recent_view: GuiLauncherView,
//     next_view: Option<GuiLauncherView>,
//     skin: Skin,
// }
//
// enum GuiLauncherView {
//     Login(LoginViewData),
//     Register(RegisterViewData),
//     Error,
// }
//
// impl Debug for GuiLauncherView {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", match self {
//             GuiLauncherView::Login(_) => "Login",
//             GuiLauncherView::Register(_) => "Register",
//             GuiLauncherView::Error => "Error",
//         })
//     }
// }
//
// struct LoginViewData {
//     str_input_user: String,
//     str_input_password: String,
// }
//
// impl Default for LoginViewData {
//     fn default() -> Self {
//         Self {
//             str_input_user: String::default(),
//             str_input_password: String::default(),
//         }
//     }
// }
//
// struct RegisterViewData {
//     str_input_user: String,
//     str_input_password_1: String,
//     str_input_password_2: String,
// }
//
// impl Default for RegisterViewData {
//     fn default() -> Self {
//         Self {
//             str_input_user: String::default(),
//             str_input_password_1: String::default(),
//             str_input_password_2: String::default(),
//         }
//     }
// }
//
// // TODO validate RegisterViewData password and user (initial) then will be response from server
//
// impl GuiLauncher {
//     pub async fn new(gui_settings: Rc<RefCell<GuiSettings>>) -> Self {
//         Self {
//             gui_settings,
//             recent_view: GuiLauncherView::Login(LoginViewData::default()),
//             next_view: None,
//             skin: Self::launcher_skin().await,
//         }
//     }
//
//     pub fn update_and_draw(&mut self) {
//         // Position and size to center window in screen
//         let window_position = self.get_launcher_window_scaled_position();
//         let window_size = self.get_launcher_window_scaled_size();
//
//         // Start using launcher GUI skin
//         root_ui().push_skin(&self.skin);
//
//         match &mut self.recent_view {
//
//             GuiLauncherView::Login(login_data) => {
//                 widgets::Window::new(
//                     hash!(),
//                     window_position,
//                     window_size
//                 )
//                     .label("Login")
//                     .movable(false)
//                     .titlebar(true)
//                     .ui(&mut *root_ui(), |ui| {
//                         ui.input_text(hash!(), "User", &mut login_data.str_input_user);
//                         ui.input_password(hash!(), "Password", &mut login_data.str_input_password);
//
//                         if ui.button(None, "Login") {
//                             println!("Attempting to login with user={}, psswd={}",
//                                      login_data.str_input_user,
//                                      login_data.str_input_password
//                             );
//                             // TODO
//                         }
//
//                         if ui.button(None, "Register") {
//                             println!("Changing view to register");
//                             self.next_view = Some(GuiLauncherView::Register(RegisterViewData::default()))
//                         }
//                     });
//             }
//
//             GuiLauncherView::Register(register_data) => {
//                 widgets::Window::new(
//                     hash!(),
//                     window_position,
//                     window_size
//                 )
//                     .label("Register")
//                     .movable(false)
//                     .titlebar(true)
//                     .ui(&mut *root_ui(), |ui| {
//                         ui.input_text(hash!(), "User", &mut register_data.str_input_user);
//                         ui.input_password(hash!(), "Password", &mut register_data.str_input_password_1);
//                         ui.input_password(hash!(), "Password repeated", &mut register_data.str_input_password_2);
//
//                         if ui.button(None, "Register") {
//                             println!("Attempting to register with user={}, psswd={}/{}",
//                                      register_data.str_input_user,
//                                      register_data.str_input_password_1,
//                                      register_data.str_input_password_2,
//                             );
//                             std::thread::sleep(Duration::from_secs(1));
//                             // TODO
//                         }
//
//                         if ui.button(None, "Login") {
//                             println!("Changing view to login");
//                             self.next_view = Some(GuiLauncherView::Login(LoginViewData::default()))
//                         }
//                     });
//             }
//
//             GuiLauncherView::Error => {
//                 widgets::Window::new(
//                     hash!(),
//                     window_position,
//                     window_size
//                 )
//                     .label("Error")
//                     .movable(false)
//                     .titlebar(true)
//                     .ui(&mut *root_ui(), |ui| {
//                         if ui.button(None, "Back") {
//                             println!("Changing view to login");
//                             self.next_view = Some(GuiLauncherView::Login(LoginViewData::default()))
//                         }
//                     });
//             }
//
//         }
//
//         // Finish using launcher GUI skin
//         root_ui().pop_skin();
//
//         // Swap views
//         if let Some(next_view) = self.next_view.take() {
//             println!("Transition launcher view: {:?} -> {:?}.", self.recent_view, next_view);
//             self.recent_view = next_view;
//         }
//
//     }
//
//     async fn launcher_skin() -> Skin {
//         let window_style = root_ui()
//             .style_builder()
//             .color(Color::from_rgba(200, 200, 230, 255))
//             .build();
//
//         let button_style = root_ui()
//             .style_builder()
//             .color(Color::from_rgba(200, 200, 180, 255))
//             .build();
//
//         Skin {
//             button_style,
//             window_style,
//             ..root_ui().default_skin()
//         }
//     }
//
//     fn get_launcher_window_scaled_size(&self) -> Vec2 {
//         DEFAULT_LAUNCHER_WINDOW_SIZE.mul(self.gui_settings.borrow().scale)
//     }
//
//     fn get_launcher_window_scaled_position(&self) -> Vec2 {
//         let launcher_window_scaled_size = self.get_launcher_window_scaled_size();
//         vec2(
//             (screen_width() - launcher_window_scaled_size.x) / 2.0,
//             (screen_height() - launcher_window_scaled_size.y) / 2.0,
//         )
//     }
// }