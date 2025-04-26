//! Parser for viscolor.txt files used in Winamp skins

use crate::archive::WszArchive;
use crate::error::{Result, WszError};
use image::Rgb;
use std::str::FromStr;

pub const VIS_COLOR_BG: usize = 0;
pub const VIS_COLOR_BG_DOTS: usize = 1;
pub const VIS_COLOR_SPEC_15: usize = 2;
pub const VIS_COLOR_SPEC_14: usize = 3;
pub const VIS_COLOR_SPEC_13: usize = 4;
pub const VIS_COLOR_SPEC_12: usize = 5;
pub const VIS_COLOR_SPEC_11: usize = 6;
pub const VIS_COLOR_SPEC_10: usize = 7;
pub const VIS_COLOR_SPEC_9: usize = 8;
pub const VIS_COLOR_SPEC_8: usize = 9;
pub const VIS_COLOR_SPEC_7: usize = 10;
pub const VIS_COLOR_SPEC_6: usize = 11;
pub const VIS_COLOR_SPEC_5: usize = 12;
pub const VIS_COLOR_SPEC_4: usize = 13;
pub const VIS_COLOR_SPEC_3: usize = 14;
pub const VIS_COLOR_SPEC_2: usize = 15;
pub const VIS_COLOR_SPEC_1: usize = 16;
pub const VIS_COLOR_SPEC_0: usize = 17;
pub const VIS_COLOR_OSC_1: usize = 18;
pub const VIS_COLOR_OSC_2: usize = 19;
pub const VIS_COLOR_OSC_3: usize = 20;
pub const VIS_COLOR_OSC_4: usize = 21;
pub const VIS_COLOR_OSC_5: usize = 22;
pub const VIS_COLOR_PEAK_DOTS: usize = 23;

/// Represents the visualization colors from viscolor.txt
#[derive(Debug, Clone)]
pub struct VisColors {
    // colors with indices given by viscolor spec
    colors: Vec<Rgb<u8>>,
}

impl VisColors {
    fn new() -> Self {
        Self { colors: Vec::new() }
    }

    /// Creates a VisColors collection from a WSZ archive
    pub fn from_archive(archive: &WszArchive) -> Result<Self> {
        let viscolor_txt = archive
            .iter()
            .find(|(name, _)| name.split('/').last().unwrap().to_lowercase() == "viscolor.txt")
            .ok_or(WszError::NotFound("viscolor.txt".to_string()))?;

        Self::from_string(&String::from_utf8_lossy(viscolor_txt.1).to_string())
    }

    /// Creates a VisColors collection from a string
    pub fn from_string(content: &str) -> Result<Self> {
        let mut colors = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            // Skip empty lines
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Remove comments
            let line = match line.split("//").next() {
                Some(content) => content.trim(),
                None => continue,
            };

            if !line.contains(",") {
                continue;
            }

            // Parse RGB values
            let rgb_parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
            if rgb_parts.len() < 3 {
                return Err(WszError::InvalidFormat {
                    line: line_num + 1,
                    error: "Expected three comma-separated RGB values".to_string(),
                });
            }

            // Parse the RGB values
            let r = u8::from_str(rgb_parts[0]).map_err(|_| WszError::InvalidFormat {
                line: line_num + 1,
                error: format!("Invalid color value: '{}'", rgb_parts[0]),
            })?;
            let g = u8::from_str(rgb_parts[1]).map_err(|_| WszError::InvalidFormat {
                line: line_num + 1,
                error: format!("Invalid color value: '{}'", rgb_parts[1]),
            })?;
            let b = u8::from_str(rgb_parts[2]).map_err(|_| WszError::InvalidFormat {
                line: line_num + 1,
                error: format!("Invalid color value: '{}'", rgb_parts[2]),
            })?;

            colors.push(Rgb([r, g, b]));
        }

        Ok(Self { colors })
    }

    /// Number of colors in the collection
    pub fn len(&self) -> usize {
        self.colors.len()
    }

    /// Checks if collection is empty
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    /// Get a color by index
    pub fn get(&self, index: usize) -> Option<Rgb<u8>> {
        self.colors.get(index).copied()
    }

    pub fn vis_color(&self, value: usize) -> Option<Rgb<u8>> {
        if value > 15 {
            return None;
        }
        self.colors.get(15 - value + VIS_COLOR_SPEC_15).copied()
    }

    pub fn vis_colors(&self) -> Vec<Rgb<u8>> {
        self.colors
            .iter()
            .enumerate()
            .filter(|(i, _)| *i >= VIS_COLOR_SPEC_15 && *i <= VIS_COLOR_SPEC_0)
            .map(|(_, c)| c.clone())
            .rev()
            .collect()
    }

    pub fn osc_color(&self, value: usize) -> Option<Rgb<u8>> {
        if value > 5 {
            return None;
        }
        self.colors.get(value + VIS_COLOR_OSC_1).copied()
    }

    pub fn osc_colors(&self) -> Vec<Rgb<u8>> {
        self.colors
            .iter()
            .enumerate()
            .filter(|(i, _)| *i >= VIS_COLOR_OSC_1 && *i <= VIS_COLOR_OSC_5)
            .map(|(_, c)| c.clone())
            .collect()
    }

    pub fn bg_color(&self) -> Option<Rgb<u8>> {
        self.colors.get(VIS_COLOR_BG).copied()
    }

    pub fn bg_dots_color(&self) -> Option<Rgb<u8>> {
        self.colors.get(VIS_COLOR_BG_DOTS).copied()
    }

    pub fn peak_dots_color(&self) -> Option<Rgb<u8>> {
        self.colors.get(VIS_COLOR_PEAK_DOTS).copied()
    }
}

impl Default for VisColors {
    fn default() -> Self {
        Self::new()
    }
}
