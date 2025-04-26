//! wsz is a Rust library for unpacking Winamp skins into sprites. It can also pack the sprites
//! back into a .wsz file and generate a screenshot of the skin as it would appear in Winamp.
//!
//! A guide to the format can be found at <https://winampskins.neocities.org/>
//!
//! Most of this code heavily inspired by <https://github.com/captbaritone/webamp>

pub mod archive;
pub mod error;
pub mod sprites;
pub mod text;

use error::{Result, WszError};

/// Top-level container for a Winamp skin. Start here.
///
/// This is the easiest way to access everything from the archive
#[derive(Debug, Clone)]
pub struct Wsz {
    sprites: std::collections::HashMap<String, sprites::SpriteImage>,
    vis_colors: text::viscolor::VisColors,
    pledit: text::pledit::PleditSettings,
    regions: text::region::Regions,
}

impl Wsz {
    /// Create a new Wsz from a .wsz file
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the .wsz file
    ///
    /// # Returns
    ///
    /// A new Wsz instance
    pub fn from_file_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let contents = archive::unpack_wsz(path)?;
        Self::from_archive(&contents)
    }

    /// Create a new Wsz from a byte slice
    ///
    /// # Arguments
    ///
    /// * `data` - The byte slice containing the Winamp skin
    ///
    /// # Returns
    ///
    /// A new Wsz instance
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let contents = archive::unpack_wsz_bytes(data)?;
        Self::from_archive(&contents)
    }

    /// Create a new Wsz from an WszArchive
    ///
    /// # Arguments
    ///
    /// * `archive` - The archive to create the Wsz from
    ///
    /// # Returns
    ///
    /// A new Wsz instance
    pub fn from_archive(archive: &archive::WszArchive) -> Result<Self> {
        let sprite_manager = sprites::SpriteManager::new();
        let sprites = sprite_manager.extract_all_sprites_from_archive(archive)?;

        // allow these to be not found
        let vis_colors = match text::viscolor::VisColors::from_archive(archive) {
            Ok(vis_colors) => vis_colors,
            Err(WszError::NotFound(_)) => text::viscolor::VisColors::default(),
            Err(e) => {
                return Err(e);
            }
        };

        let pledit = match text::pledit::PleditSettings::from_archive(archive) {
            Ok(pledit) => pledit,
            Err(WszError::NotFound(_)) => text::pledit::PleditSettings::default(),
            Err(e) => {
                return Err(e);
            }
        };

        // discard any regions errors
        let regions = match text::region::Regions::from_archive(archive) {
            Ok(regions) => regions,
            Err(_) => text::region::Regions::default(),
        };

        Ok(Self {
            sprites,
            vis_colors,
            pledit,
            regions,
        })
    }

    /// Get a sprite from the archive
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the sprite to get
    ///
    /// # Returns
    ///
    /// The sprite if it exists, otherwise None
    pub fn get_sprite(&self, name: &str) -> Option<&sprites::SpriteImage> {
        self.sprites.get(name)
    }

    /// Get all sprites from the archive
    ///
    /// # Returns
    ///
    /// A map of sprite names to sprite images
    pub fn get_sprites(&self) -> &std::collections::HashMap<String, sprites::SpriteImage> {
        &self.sprites
    }

    /// Get the visualization colors (viscolor.txt)
    ///
    /// # Returns
    ///
    /// The VisColors instance
    pub fn get_vis_colors(&self) -> &text::viscolor::VisColors {
        &self.vis_colors
    }

    /// Get the playlist editor settings (pledit.txt)
    ///
    /// # Returns
    ///
    /// The PleditSettings instance
    pub fn get_pledit_settings(&self) -> &text::pledit::PleditSettings {
        &self.pledit
    }

    /// Get the regions (region.txt)
    ///
    /// # Returns
    ///
    /// The Regions instance
    pub fn get_regions(&self) -> &text::region::Regions {
        &self.regions
    }

    /// Render a screenshot of the skin
    ///
    /// # Returns
    ///
    /// A WindowImage instance
    pub fn render_screenshot(&self) -> Result<sprites::WindowImage> {
        let mut window_defs = sprites::SpriteWindowManager::new();
        if let Some(bg_color) = self.pledit.normal_bg {
            window_defs.set_bg_color(bg_color);
        }
        let window = window_defs.draw_all_sprites(&self.sprites)?;
        Ok(window)
    }
}
