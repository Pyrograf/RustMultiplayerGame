#[derive(Debug, PartialOrd, PartialEq)]
pub enum GuiCommand {
    ServerOff,
    ServerOn,
    Shutdown,
}