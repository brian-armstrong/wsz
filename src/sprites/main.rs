use super::{SpriteDefinition, SpriteWindowDefinition, WindowType};

pub fn main_sprites() -> Vec<SpriteDefinition> {
    vec![SpriteDefinition {
        name: "MAIN_WINDOW_BACKGROUND".to_string(),
        sprite_sheet: "MAIN.BMP".to_string(),
        x: 0,
        y: 0,
        width: 275,
        height: 116,
    }]
}

pub fn main_window_sprites() -> Vec<SpriteWindowDefinition> {
    vec![SpriteWindowDefinition {
        name: "MAIN_WINDOW_BACKGROUND".to_string(),
        sprite_name: "MAIN_WINDOW_BACKGROUND".to_string(),
        window_type: WindowType::Main,
        layer: 0,
        x: 0,
        y: 0,
        width: 275,
        height: 116,
    }]
}
