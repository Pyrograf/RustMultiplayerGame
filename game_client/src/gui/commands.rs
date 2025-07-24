use accounts_manager::JwtToken;
use crate::gui::{LoginData, LoginFailedReason, RegisterData, RegisterFailedReason};

#[derive(Debug, PartialOrd, PartialEq)]
pub enum GuiCommand {
    ServerOff {
        reason: String,
    },
    ServerOn {
        motd: String,
    },
    AckServerOffline,

    ShowShutdownDialog,
    AbortShutdownDialog,
    ProceedShutdownDialog,

    EnterRegisterView,
    EnterLoginView(Option<LoginData>),
    
    PassLoginData(LoginData),
    LoginFailed(LoginFailedReason),
    LoginSuccess((String, JwtToken)),
    
    PassRegisterData(RegisterData),
    RegisterFailed(RegisterFailedReason),
    RegisterSuccess(String),
}
