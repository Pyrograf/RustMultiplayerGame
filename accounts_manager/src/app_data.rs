#[derive(Debug)]
pub struct AppData {
    _dummy: u32
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            _dummy: 0
        }
    }
}