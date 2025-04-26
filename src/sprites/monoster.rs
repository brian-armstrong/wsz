use super::{SpriteDefinition, SpriteWindowDefinition, WindowType};

pub fn monoster_sprites() -> Vec<SpriteDefinition> {
    vec![
        SpriteDefinition {
            name: "MAIN_STEREO".to_string(),
            sprite_sheet: "MONOSTER.BMP".to_string(),
            x: 0,
            y: 12,
            width: 29,
            height: 12,
        },
        SpriteDefinition {
            name: "MAIN_STEREO_ACTIVE".to_string(),
            sprite_sheet: "MONOSTER.BMP".to_string(),
            x: 0,
            y: 0,
            width: 29,
            height: 12,
        },
        SpriteDefinition {
            name: "MAIN_MONO".to_string(),
            sprite_sheet: "MONOSTER.BMP".to_string(),
            x: 29,
            y: 12,
            width: 27,
            height: 12,
        },
        SpriteDefinition {
            name: "MAIN_MONO_ACTIVE".to_string(),
            sprite_sheet: "MONOSTER.BMP".to_string(),
            x: 29,
            y: 0,
            width: 27,
            height: 12,
        },
    ]
}

pub fn monoster_window_sprites() -> Vec<SpriteWindowDefinition> {
    vec![
        SpriteWindowDefinition {
            name: "MAIN_MONO".to_string(),
            sprite_name: "MAIN_MONO".to_string(),
            window_type: WindowType::Main,
            layer: 1,
            x: 212,
            y: 41,
            width: 27,
            height: 12,
        },
        SpriteWindowDefinition {
            name: "MAIN_STEREO".to_string(),
            sprite_name: "MAIN_STEREO".to_string(),
            window_type: WindowType::Main,
            layer: 1,
            x: 239,
            y: 41,
            width: 29,
            height: 12,
        },
    ]
}
