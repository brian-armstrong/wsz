use super::{SpriteDefinition, SpriteWindowDefinition, WindowType};

pub fn posbar_sprites() -> Vec<SpriteDefinition> {
    vec![
        SpriteDefinition {
            name: "MAIN_POSITION_SLIDER_BACKGROUND".to_string(),
            sprite_sheet: "POSBAR.BMP".to_string(),
            x: 0,
            y: 0,
            width: 248,
            height: 10,
        },
        SpriteDefinition {
            name: "MAIN_POSITION_SLIDER_THUMB".to_string(),
            sprite_sheet: "POSBAR.BMP".to_string(),
            x: 248,
            y: 0,
            width: 29,
            height: 10,
        },
        SpriteDefinition {
            name: "MAIN_POSITION_SLIDER_THUMB_SELECTED".to_string(),
            sprite_sheet: "POSBAR.BMP".to_string(),
            x: 278,
            y: 0,
            width: 29,
            height: 10,
        },
    ]
}

pub fn posbar_window_sprites() -> Vec<SpriteWindowDefinition> {
    vec![
        SpriteWindowDefinition {
            name: "MAIN_POSITION_SLIDER_BACKGROUND".to_string(),
            sprite_name: "MAIN_POSITION_SLIDER_BACKGROUND".to_string(),
            window_type: WindowType::Main,
            layer: 1,
            x: 17,
            y: 72,
            width: 248,
            height: 10,
        },
        SpriteWindowDefinition {
            name: "MAIN_POSITION_SLIDER_THUMB".to_string(),
            sprite_name: "MAIN_POSITION_SLIDER_THUMB".to_string(),
            window_type: WindowType::Main,
            layer: 2,
            x: 17,
            y: 72,
            width: 29,
            height: 10,
        },
    ]
}
