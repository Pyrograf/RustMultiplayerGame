pub mod launcher;

use macroquad::math::{vec2, Vec2};
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::{screen_height, screen_width};

/// Infos:
/// * Screen - entire viewport
/// * Window - top level GUI container

const WINDOW_MINIMAL_SIZE: Vec2 = vec2(800.0, 600.0);

pub fn constrain_screen_size() {
    if screen_width() < WINDOW_MINIMAL_SIZE.x || screen_height() < WINDOW_MINIMAL_SIZE.y {
        set_window_size(
            screen_width().max(WINDOW_MINIMAL_SIZE.x).round() as u32 + 1,
            screen_height().max(WINDOW_MINIMAL_SIZE.y).round() as u32 + 1
        );
    }
}

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