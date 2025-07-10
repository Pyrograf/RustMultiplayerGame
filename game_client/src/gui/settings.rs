#[derive(Debug)]
pub struct GuiSettings {
    pub scale: f32,
}

impl Default for GuiSettings {
    fn default() -> Self {
        Self {
            scale: 1.0
        }
    }
}