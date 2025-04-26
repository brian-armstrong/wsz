//! Parser for pledit.txt files used in Winamp skins
//!
//! pledit.txt defines colors and font for the playlist editor

use std::collections::HashMap;

use image::Rgb;

use crate::archive::WszArchive;
use crate::error::{Result, WszError};

/// Playlist editor settings (colors and font)
#[derive(Debug, Clone)]
pub struct PleditSettings {
    /// Normal text color
    pub normal: Option<Rgb<u8>>,
    /// Current text color
    pub current: Option<Rgb<u8>>,
    /// Normal background color
    pub normal_bg: Option<Rgb<u8>>,
    /// Selected background color
    pub selected_bg: Option<Rgb<u8>>,
    /// Font name
    pub font: Option<String>,
    /// Additional custom settings not defined in the standard
    pub custom: HashMap<String, String>,
}

impl PleditSettings {
    fn new() -> Self {
        Self {
            normal: None,
            current: None,
            normal_bg: None,
            selected_bg: None,
            font: None,
            custom: HashMap::new(),
        }
    }

    /// Find pledit.txt in the archive contents
    pub fn from_archive(archive: &WszArchive) -> Result<Self> {
        let pledit_txt = archive
            .iter()
            .find(|(name, _)| name.split('/').last().unwrap().to_lowercase() == "pledit.txt")
            .ok_or(WszError::NotFound("pledit.txt".to_string()))?;

        Self::from_string(&String::from_utf8_lossy(pledit_txt.1).to_string())
    }

    /// Parse pledit.txt content into settings
    pub fn from_string(content: &str) -> Result<Self> {
        let mut settings = Self::new();

        let mut current_section = String::new();

        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();

            // Skip empty lines
            if line.is_empty() {
                continue;
            }

            // Skip comments
            if line.starts_with(';') {
                continue;
            }

            // Check for section headers
            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len() - 1].to_string();
                continue;
            }

            // Parse key-value pairs
            if let Some(pos) = line.find('=') {
                let key = line[..pos].trim();
                let value = line[pos + 1..].trim();

                match current_section.as_str() {
                    "Text" => {
                        match key.to_lowercase().as_str() {
                            "normal" => {
                                settings.normal = Some(parse_hex_color(value)?);
                            }
                            "current" => {
                                settings.current = Some(parse_hex_color(value)?);
                            }
                            "normalbg" => {
                                settings.normal_bg = Some(parse_hex_color(value)?);
                            }
                            "selectedbg" => {
                                settings.selected_bg = Some(parse_hex_color(value)?);
                            }
                            "font" => {
                                settings.font = Some(value.to_string());
                            }
                            _ => {
                                // Store unknown keys as custom settings
                                settings.custom.insert(key.to_string(), value.to_string());
                            }
                        }
                    }
                    // Store settings from other sections as custom settings
                    _ if !current_section.is_empty() => {
                        let full_key = format!("{}.{}", current_section, key);
                        settings.custom.insert(full_key, value.to_string());
                    }
                    _ => {
                        return Err(WszError::InvalidFormat {
                            line: line_num + 1,
                            error: "Key-value pair outside of any section".to_string(),
                        });
                    }
                }
            } else {
                // Line is not a key-value pair and not a section header
                return Err(WszError::InvalidFormat {
                    line: line_num + 1,
                    error: format!("Invalid line format: '{}'", line),
                });
            }
        }

        Ok(settings)
    }
}

/// Parse a hex color string (e.g. "#9BBBAD" or "9BBBAD")
fn parse_hex_color(hex: &str) -> Result<Rgb<u8>> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        return Err(WszError::InvalidFormat {
            line: 0,
            error: format!("Invalid hex color format: '{}'", hex),
        });
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| WszError::InvalidFormat {
        line: 0,
        error: format!("Invalid hex value for red: '{}'", &hex[0..2]),
    })?;

    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| WszError::InvalidFormat {
        line: 0,
        error: format!("Invalid hex value for green: '{}'", &hex[2..4]),
    })?;

    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| WszError::InvalidFormat {
        line: 0,
        error: format!("Invalid hex value for blue: '{}'", &hex[4..6]),
    })?;

    Ok(Rgb([r, g, b]))
}

impl Default for PleditSettings {
    fn default() -> Self {
        Self::new()
    }
}
