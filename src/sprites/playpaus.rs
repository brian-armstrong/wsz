use super::{SpriteDefinition, SpriteWindowDefinition, WindowType};

pub fn playpaus_sprites() -> Vec<SpriteDefinition> {
    vec![
        SpriteDefinition {
            name: "MAIN_PLAYING_INDICATOR".to_string(),
            sprite_sheet: "PLAYPAUS.BMP".to_string(),
            x: 0,
            y: 0,
            width: 9,
            height: 9,
        },
        SpriteDefinition {
            name: "MAIN_PAUSED_INDICATOR".to_string(),
            sprite_sheet: "PLAYPAUS.BMP".to_string(),
            x: 9,
            y: 0,
            width: 9,
            height: 9,
        },
        SpriteDefinition {
            name: "MAIN_STOPPED_INDICATOR".to_string(),
            sprite_sheet: "PLAYPAUS.BMP".to_string(),
            x: 18,
            y: 0,
            width: 9,
            height: 9,
        },
        SpriteDefinition {
            name: "MAIN_NOT_WORKING_INDICATOR".to_string(),
            sprite_sheet: "PLAYPAUS.BMP".to_string(),
            x: 36,
            y: 0,
            width: 3,
            height: 9,
        },
        SpriteDefinition {
            name: "MAIN_WORKING_INDICATOR".to_string(),
            sprite_sheet: "PLAYPAUS.BMP".to_string(),
            x: 39,
            y: 0,
            width: 3,
            height: 9,
        },
    ]
}

pub fn playpaus_window_sprites() -> Vec<SpriteWindowDefinition> {
    vec![SpriteWindowDefinition {
        name: "MAIN_STOPPED_INDICATOR".to_string(),
        sprite_name: "MAIN_STOPPED_INDICATOR".to_string(),
        window_type: WindowType::Main,
        layer: 1,
        x: 26,
        y: 28,
        width: 9,
        height: 9,
    }]
}
