//! Sprite definitions for Winamp skin elements

mod balance;
mod cbuttons;
mod eq_ex;
mod eqmain;
mod gen_;
mod main;
mod monoster;
mod numbers;
mod nums_ex;
mod playpaus;
mod pledit;
mod posbar;
mod shufrep;
mod text;
mod titlebar;
mod volume;

use crate::archive::WszArchive;
use crate::error::{Result, WszError};

use std::collections::HashMap;
use std::io::{self, Cursor};
use std::path::Path;

use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgb, Rgba, RgbaImage};

const SPRITE_SHEETS: [&str; 16] = [
    "BALANCE.BMP",
    "CBUTTONS.BMP",
    "MAIN.BMP",
    "MONOSTER.BMP",
    "NUMBERS.BMP",
    "NUMS_EX.BMP",
    "PLAYPAUS.BMP",
    "PLEDIT.BMP",
    "EQ_EX.BMP",
    "EQMAIN.BMP",
    "POSBAR.BMP",
    "SHUFREP.BMP",
    "TEXT.BMP",
    "TITLEBAR.BMP",
    "VOLUME.BMP",
    "GEN.BMP",
];

/// The standard background color used in Winamp skin BMPs (#00C5FF)
const WINAMP_BG_COLOR: Rgba<u8> = Rgba([0, 198, 255, 255]);

/// Represents a sprite within a sprite sheet
#[derive(Debug, Clone)]
pub struct SpriteDefinition {
    /// Name of the sprite
    pub name: String,
    /// Name of the sprite sheet
    pub sprite_sheet: String,
    /// X position (from left)
    pub x: u32,
    /// Y position (from top)
    pub y: u32,
    /// Width in pixelc
    pub width: u32,
    /// Height in pixels
    pub height: u32,
}

pub type SpriteImage = ImageBuffer<Rgba<u8>, Vec<u8>>;

/// Extracts sprites from sprite sheets in the WSZ archive
#[derive(Debug, Clone)]
pub struct SpriteManager {
    definitions: HashMap<String, SpriteDefinition>,
}

impl SpriteManager {
    /// Creates a new SpriteManager
    ///
    /// # Returns
    ///
    /// A new SpriteManager
    pub fn new() -> Self {
        let mut definitions = HashMap::new();

        let mut ins_fn = |def: &SpriteDefinition| {
            definitions.insert(def.name.clone(), def.clone());
        };

        // Add sprites for each sprite sheet
        balance::balance_sprites().iter().for_each(&mut ins_fn);
        cbuttons::cbuttons_sprites().iter().for_each(&mut ins_fn);
        main::main_sprites().iter().for_each(&mut ins_fn);
        monoster::monoster_sprites().iter().for_each(&mut ins_fn);
        numbers::numbers_sprites().iter().for_each(&mut ins_fn);
        nums_ex::nums_ex_sprites().iter().for_each(&mut ins_fn);
        playpaus::playpaus_sprites().iter().for_each(&mut ins_fn);
        pledit::pledit_sprites().iter().for_each(&mut ins_fn);
        eq_ex::eq_ex_sprites().iter().for_each(&mut ins_fn);
        eqmain::eqmain_sprites().iter().for_each(&mut ins_fn);
        posbar::posbar_sprites().iter().for_each(&mut ins_fn);
        shufrep::shufrep_sprites().iter().for_each(&mut ins_fn);
        text::text_sprites().iter().for_each(&mut ins_fn);
        titlebar::titlebar_sprites().iter().for_each(&mut ins_fn);
        volume::volume_sprites().iter().for_each(&mut ins_fn);
        gen_::gen_sprites().iter().for_each(&mut ins_fn);

        Self { definitions }
    }

    /// Extracts all sprites from all known sprite sheets in the archive. Some sheets may be missing.
    ///
    /// # Arguments
    ///
    /// * `archive` - WSZ archive
    ///
    /// # Returns
    ///
    /// A Result containing a HashMap sprite names to sprite images
    pub fn extract_all_sprites_from_archive(&self, archive: &WszArchive) -> Result<HashMap<String, SpriteImage>> {
        let mut sprite_images = HashMap::new();
        for bmp_name in SPRITE_SHEETS {
            if let Ok(sprite_sheet) = self.extract_sprite_sheet_from_archive(archive, bmp_name) {
                sprite_images.extend(sprite_sheet);
            }
        }
        Ok(sprite_images)
    }

    /// Extracts all sprites from a specific BMP file in the archive, supporting nested directories
    ///
    /// # Arguments
    ///
    /// * `archive` - WSZ archive
    /// * `sprite_sheet_name` - Name of the sprite sheet to extract sprites from (case insensitive)
    ///
    /// # Returns
    ///
    /// A Result containing a HashMap of sprite names to sprite images
    pub fn extract_sprite_sheet_from_archive(
        &self,
        archive: &WszArchive,
        sprite_sheet_name: &str,
    ) -> Result<HashMap<String, SpriteImage>> {
        if !SPRITE_SHEETS.contains(&sprite_sheet_name) {
            return Err(WszError::ArgumentError(format!(
                "{} not a known sprite sheet",
                sprite_sheet_name
            )));
        }

        // First, try to find the BMP file directly (case insensitive)
        let key = archive
            .keys()
            .find(|k| k.to_uppercase() == sprite_sheet_name.to_uppercase());

        // If not found, look for it in any subdirectory
        let key = match key {
            Some(k) => k,
            None => {
                // Look for any file ending with this filename (in any directory)
                archive
                    .keys()
                    .find(|k| {
                        let path = Path::new(k);
                        let file_name = path.file_name().and_then(|s| s.to_str());
                        file_name.map_or(false, |s| s.to_uppercase() == sprite_sheet_name.to_uppercase())
                    })
                    .ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::NotFound,
                            format!("{} not found in skin (at any path)", sprite_sheet_name),
                        )
                    })?
            }
        };

        let data = &archive[key];
        let cursor = Cursor::new(data);

        // Decode the image
        let img = image::ImageReader::new(cursor)
            .with_guessed_format()
            .map_err(WszError::Io)?
            .decode()
            .map_err(WszError::ImageError)?;

        // Extract each sprite
        let mut sprite_images = HashMap::new();
        for (sprite_name, def) in self.definitions.iter() {
            if def.sprite_sheet == sprite_sheet_name {
                let sprite_img = self.extract_sprite(sprite_name, &img)?;
                sprite_images.insert(sprite_name.clone(), sprite_img);
            }
        }

        Ok(sprite_images)
    }

    /// Creates a new image large enough to fit all sprites from a specific file
    /// and places those sprites at their defined positions
    ///
    /// # Arguments
    ///
    /// * `sprites` - A map of sprite names to their image data
    /// * `sprite_sheet_name` - The name of the sprite sheet to construct
    ///
    /// # Returns
    ///
    /// A Result containing the reconstructed image with all sprites in their proper positions
    pub fn construct_sprite_sheet(
        &self,
        sprites: &HashMap<String, SpriteImage>,
        sprite_sheet_name: &str,
    ) -> Result<DynamicImage> {
        // Determine the size needed for the image by finding the maximum extents
        let mut max_width = 0;
        let mut max_height = 0;

        for sprite in self.definitions.values() {
            if sprite.sprite_sheet == sprite_sheet_name {
                if let Some(sprite_img) = sprites.get(&sprite.name) {
                    if sprite_img.width() == 0 || sprite_img.height() == 0 {
                        continue;
                    }

                    let right_edge = sprite.x + sprite_img.width();
                    let bottom_edge = sprite.y + sprite_img.height();

                    max_width = max_width.max(right_edge);
                    max_height = max_height.max(bottom_edge);
                }
            }
        }

        // Create a new image filled with the Winamp background color
        let img = RgbaImage::from_pixel(max_width, max_height, WINAMP_BG_COLOR);
        let mut image = DynamicImage::ImageRgba8(img);

        // Place each sprite onto the image at its defined position
        for sprite_def in self.definitions.values() {
            if sprite_def.sprite_sheet == sprite_sheet_name {
                if let Some(sprite_img) = sprites.get(&sprite_def.name) {
                    // Copy the sprite to its position on the reconstructed image
                    image.copy_from(sprite_img, sprite_def.x, sprite_def.y)?;
                }
            }
        }

        Ok(image)
    }

    /// Extracts a specific sprite from a sprite sheet image
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the sprite
    /// * `sprite_sheet` - Source image containing the sprite
    ///
    /// # Returns
    ///
    /// A Result containing the extracted sprite as a new image.
    /// 
    /// If the sprite is partially off the sheet, it will be drawn with whatever part overlaps the sheet.
    pub fn extract_sprite(&self, name: &str, sprite_sheet: &DynamicImage) -> Result<SpriteImage> {
        let def = self
            .get_sprite_definition(name)
            .ok_or(WszError::ArgumentError(format!("Sprite {} not found", name)))?;

        if def.x > sprite_sheet.width() || def.y > sprite_sheet.height() {
            return Ok(ImageBuffer::new(0, 0));
        }

        let width = if def.x + def.width > sprite_sheet.width() { sprite_sheet.width() - def.x } else { def.width };
        let height = if def.y + def.height > sprite_sheet.height() { sprite_sheet.height() - def.y } else { def.height };

        let mut sprite_img = ImageBuffer::new(width, height);

        for (dst_x, dst_y, pixel) in sprite_img.enumerate_pixels_mut() {
            let src_x = def.x + dst_x;
            let src_y = def.y + dst_y;

            if src_x >= sprite_sheet.width() || src_y >= sprite_sheet.height() {
                continue;
            }

            *pixel = sprite_sheet.get_pixel(src_x, src_y);
        }

        Ok(sprite_img)
    }

    /// Returns the names of all known sprite sheets
    ///
    /// # Returns
    ///
    /// A Vec of the sprite sheet names
    pub fn sprite_sheet_names() -> Vec<String> {
        SPRITE_SHEETS.iter().map(|s| s.to_string()).collect()
    }

    /// Returns a sprite definition by name
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the sprite
    ///
    /// # Returns
    ///
    /// A reference to the sprite definition
    pub fn get_sprite_definition(&self, name: &str) -> Option<&SpriteDefinition> {
        self.definitions.get(name)
    }

    /// Returns all sprite definitions
    ///
    /// # Returns
    ///
    /// A reference to the internal HashMap of sprite definitions
    pub fn get_sprite_definitions(&self) -> &HashMap<String, SpriteDefinition> {
        &self.definitions
    }
}

const WINDOW_HEIGHT: u32 = 435;
const WINDOW_WIDTH: u32 = 275;
const MAX_LAYER: u32 = 3;
const MAIN_WINDOW_START_Y: u32 = 0;
const MAIN_WINDOW_START_X: u32 = 0;
const EQUALIZER_WINDOW_START_Y: u32 = 116;
const EQUALIZER_WINDOW_START_X: u32 = 0;
const PLAYLIST_WINDOW_START_Y: u32 = 232;
const PLAYLIST_WINDOW_START_X: u32 = 0;

pub type WindowImage = ImageBuffer<Rgba<u8>, Vec<u8>>;

/// Type of window sprite
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowType {
    /// Main window
    Main,
    /// Equalizer window
    Equalizer,
    /// Playlist window
    Playlist,
}

/// Represents a sprite within a window image   
#[derive(Debug, Clone)]
pub struct SpriteWindowDefinition {
    /// Name of the element
    pub name: String,
    /// Name of the sprite in the sprite sheet
    pub sprite_name: String,
    /// Type of window
    pub window_type: WindowType,
    /// Layer of the sprite
    pub layer: u32,
    /// X position (from left)
    pub x: u32,
    /// Y position (from top)
    pub y: u32,
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
}

/// Rasterizes sprites into an image resembling a Winamp window
pub struct SpriteWindowManager {
    definitions: HashMap<String, SpriteWindowDefinition>,
    bg_color: Rgb<u8>,
}

impl SpriteWindowManager {
    pub fn new() -> Self {
        let mut definitions = HashMap::new();

        let mut ins_fn = |def: &SpriteWindowDefinition| {
            definitions.insert(def.name.clone(), def.clone());
        };

        balance::balance_window_sprites().iter().for_each(&mut ins_fn);
        cbuttons::cbuttons_window_sprites().iter().for_each(&mut ins_fn);
        main::main_window_sprites().iter().for_each(&mut ins_fn);
        monoster::monoster_window_sprites().iter().for_each(&mut ins_fn);
        playpaus::playpaus_window_sprites().iter().for_each(&mut ins_fn);
        pledit::pledit_window_sprites().iter().for_each(&mut ins_fn);
        eqmain::eqmain_window_sprites().iter().for_each(&mut ins_fn);
        posbar::posbar_window_sprites().iter().for_each(&mut ins_fn);
        shufrep::shufrep_window_sprites().iter().for_each(&mut ins_fn);
        titlebar::titlebar_window_sprites().iter().for_each(&mut ins_fn);
        volume::volume_window_sprites().iter().for_each(&mut ins_fn);

        Self {
            definitions,
            bg_color: Rgb([0, 0, 0]),
        }
    }

    /// Draws all sprites in a window
    pub fn draw_all_sprites(&self, sprites: &HashMap<String, SpriteImage>) -> Result<WindowImage> {
        let mut window = ImageBuffer::from_pixel(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            Rgba([self.bg_color.0[0], self.bg_color.0[1], self.bg_color.0[2], 255]),
        );
        for layer in 0..MAX_LAYER {
            for sprite_def in self.definitions.values() {
                if sprite_def.layer == layer {
                    if let Some(sprite) = sprites.get(sprite_def.sprite_name.as_str()) {
                        self.draw_sprite(&mut window, sprite, &sprite_def.name)?;
                    }
                }
            }
        }
        Ok(window)
    }

    /// Draws a sprite in a window
    pub fn draw_sprite(&self, window: &mut WindowImage, sprite: &SpriteImage, window_sprite_name: &str) -> Result<()> {
        let sprite_def = self
            .definitions
            .get(window_sprite_name)
            .ok_or(WszError::ArgumentError(format!(
                "Sprite {} not found",
                window_sprite_name
            )))?;

        let (start_x, start_y) = match sprite_def.window_type {
            WindowType::Main => (MAIN_WINDOW_START_X, MAIN_WINDOW_START_Y),
            WindowType::Equalizer => (EQUALIZER_WINDOW_START_X, EQUALIZER_WINDOW_START_Y),
            WindowType::Playlist => (PLAYLIST_WINDOW_START_X, PLAYLIST_WINDOW_START_Y),
        };

        let start_x = start_x + sprite_def.x;
        let start_y = start_y + sprite_def.y;

        if start_x + sprite_def.width > window.width() || start_y + sprite_def.height > window.height() {
            return Err(WszError::ArgumentError("Sprite is out of bounds".to_string()));
        }

        for (src_x, src_y, pixel) in sprite.enumerate_pixels() {
            let dst_x = start_x + src_x;
            let dst_y = start_y + src_y;

            window.put_pixel(dst_x, dst_y, *pixel);
        }

        Ok(())
    }

    /// Sets the background color of the window
    ///
    /// # Arguments
    ///
    /// * `color` - Background color
    pub fn set_bg_color(&mut self, color: Rgb<u8>) {
        self.bg_color = color;
    }

    /// Sets the position of a window sprite
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the window sprite
    /// * `x` - X position
    /// * `y` - Y position
    pub fn set_sprite_position(&mut self, name: &str, x: u32, y: u32) {
        if let Some(sprite_def) = self.definitions.get_mut(name) {
            sprite_def.x = x;
            sprite_def.y = y;
        }
    }

    /// Sets the sprite sheet name of a window sprite (changes what is drawn)
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the window sprite
    /// * `new_name` - New sprite sheet name of the sprite
    pub fn set_sprite_name(&mut self, name: &str, new_name: &str) {
        if let Some(sprite_def) = self.definitions.get_mut(name) {
            sprite_def.sprite_name = new_name.to_string();
        }
    }

    /// Adds a window sprite to the sprite manager
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the window sprite
    /// * `definition` - SpriteWindowDefinition
    pub fn add_window_sprite(&mut self, name: &str, definition: &SpriteWindowDefinition) {
        self.definitions.insert(name.to_string(), definition.clone());
    }

    /// Removes a window sprite from the sprite manager
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the window sprite
    pub fn remove_window_sprite(&mut self, name: &str) {
        self.definitions.remove(name);
    }
}
