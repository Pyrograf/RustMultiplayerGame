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
}
