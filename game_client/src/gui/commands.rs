use crate::gui::{LoginData, RegisterData};

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
    EnterLoginView,
    
    PassLoginData(LoginData),
    PassRegisterData(RegisterData),
}
