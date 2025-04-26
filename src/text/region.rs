//! Parser for region.txt files used in Winamp skins
//!
//! region.txt defines transparent regions for the window
//!
//! Each region is a list of vertices which define a polygon. The outside of the polygon is the transparent region.

use crate::archive::WszArchive;
use crate::error::{Result, WszError};

enum RegionType {
    Main,
    MainShade,
    Equalizer,
    EqualizerShade,
}

impl RegionType {
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Normal" => Ok(Self::Main),
            "WindowShade" => Ok(Self::MainShade),
            "Equalizer" => Ok(Self::Equalizer),
            "EqualizerWS" => Ok(Self::EqualizerShade),
            _ => Err(WszError::InvalidFormat {
                line: 0,
                error: format!("Invalid region type: '{}'", s),
            }),
        }
    }
}

struct Region {
    pub num_points: Vec<usize>,
    pub points: Vec<u32>,
}

impl Region {
    fn new() -> Self {
        Self {
            num_points: Vec::new(),
            points: Vec::new(),
        }
    }
}

/// Transparent regions
#[derive(Debug, Clone)]
pub struct Regions {
    /// Main window region
    pub main: Option<Vec<Vec<(u32, u32)>>>,
    /// Main window shade region
    pub main_shade: Option<Vec<Vec<(u32, u32)>>>,
    /// Equalizer window region
    pub equalizer: Option<Vec<Vec<(u32, u32)>>>,
    /// Equalizer window shade region
    pub equalizer_shade: Option<Vec<Vec<(u32, u32)>>>,
}

impl Regions {
    fn new() -> Self {
        Self {
            main: None,
            main_shade: None,
            equalizer: None,
            equalizer_shade: None,
        }
    }

    /// Find region.txt in the archive contents
    pub fn from_archive(archive: &WszArchive) -> Result<Self> {
        let region_txt = archive
            .iter()
            .find(|(name, _)| name.split('/').last().unwrap().to_lowercase() == "region.txt")
            .ok_or(WszError::NotFound("region.txt".to_string()))?;

        Self::from_string(&String::from_utf8_lossy(region_txt.1).to_string())
    }

    /// Parse region.txt content into regions
    pub fn from_string(content: &str) -> Result<Self> {
        let mut regions = Self::new();

        let mut current_section = None;
        let mut current_region = Region::new();

        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();

            // Skip empty lines
            if line.is_empty() {
                continue;
            }

            // Skip comments
            if line.starts_with(";") {
                continue;
            }

            // Check for section headers
            if line.starts_with('[') && line.ends_with(']') {
                current_section = Some(RegionType::from_str(line[1..line.len() - 1].to_string().as_str())?);
                current_region = Region::new();
                continue;
            }

            // Parse key-value pairs
            if let Some(pos) = line.find('=') {
                let key = line[..pos].trim();
                let value = line[pos + 1..].trim();

                match current_section {
                    Some(_) => {
                        match key.to_lowercase().as_str() {
                            "numpoints" => {
                                current_region.num_points =
                                    value.split(',').map(|s| s.trim().parse::<usize>().unwrap()).collect();
                            }
                            "pointlist" => {
                                // PointList can be either comma or space separated
                                current_region.points = value
                                    .split(|c: char| c.is_whitespace() || c == ',')
                                    .filter(|s| !s.is_empty())
                                    .map(|s| s.trim().parse::<u32>().unwrap())
                                    .collect();
                            }
                            _ => {
                                // Store unknown keys as custom settings
                                return Err(WszError::InvalidFormat {
                                    line: line_num + 1,
                                    error: format!("Invalid key: '{}'", key),
                                });
                            }
                        }
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

            if current_section.is_some() && current_region.num_points.len() > 0 && current_region.points.len() > 0 {
                if current_region.num_points.iter().sum::<usize>() != current_region.points.len() / 2 {
                    return Err(WszError::InvalidFormat {
                        line: line_num + 1,
                        error: "Number of points does not match number of points in the region".to_string(),
                    });
                }

                let mut points_list = Vec::new();
                let mut current_offset = 0;
                for i in 0..current_region.num_points.len() {
                    let mut points = Vec::new();
                    for j in 0..current_region.num_points[i] {
                        let index = (current_offset + j) * 2;
                        points.push((current_region.points[index], current_region.points[index + 1]));
                    }
                    points_list.push(points);
                    current_offset += current_region.num_points[i];
                }

                match current_section {
                    Some(RegionType::Main) => {
                        regions.main = Some(points_list);
                    }
                    Some(RegionType::MainShade) => {
                        regions.main_shade = Some(points_list);
                    }
                    Some(RegionType::Equalizer) => {
                        regions.equalizer = Some(points_list);
                    }
                    Some(RegionType::EqualizerShade) => {
                        regions.equalizer_shade = Some(points_list);
                    }
                    _ => {
                        unreachable!()
                    }
                }

                current_section = None;
                current_region = Region::new();
            }
        }

        Ok(regions)
    }
}

impl Default for Regions {
    fn default() -> Self {
        Self::new()
    }
}
