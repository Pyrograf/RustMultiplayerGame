use macroquad::color::Color;
use macroquad::ui::{root_ui, Skin};

pub fn common_skin() -> Skin {
    let window_style = root_ui()
        .style_builder()
        .color(Color::from_rgba(200, 200, 230, 255))
        .build();

    let button_style = root_ui()
        .style_builder()
        .color(Color::from_rgba(200, 200, 180, 255))
        .build();

    Skin {
        button_style,
        window_style,
        ..root_ui().default_skin()
    }
}

