//! Logic for unpacking and packing Winamp skins from archive (zip) format

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::Path;

use zip::ZipArchive;

use crate::error::Result;
use crate::sprites::SpriteManager;
use image::ImageFormat;

pub type WszArchive = HashMap<String, Vec<u8>>;

/// Unpacks a Winamp skin file (.wsz) into memory
///
/// # Arguments
///
/// * `path` - Path to the Winamp skin file
///
/// # Returns
///
/// A Result containing a HashMap of file names to their contents as bytes
pub fn unpack_wsz<P: AsRef<Path>>(path: P) -> Result<WszArchive> {
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut contents = HashMap::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let name = file.name().to_string();

        // Skip directories
        if file.is_dir() {
            continue;
        }

        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        contents.insert(name, data);
    }

    Ok(contents)
}

/// Unpacks a Winamp skin file (.wsz) from a byte array in memory
///
/// # Arguments
///
/// * `data` - Byte slice containing the WSZ file data
///
/// # Returns
///
/// A Result containing a HashMap of file names to their contents as bytes
pub fn unpack_wsz_bytes(data: &[u8]) -> Result<WszArchive> {
    // Create a cursor to read the data from memory
    let cursor = Cursor::new(data);

    // Open the zip archive from the cursor
    let mut archive = ZipArchive::new(cursor)?;
    let mut contents = HashMap::new();

    // Process each file in the archive
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let name = file.name().to_string();

        // Skip directories
        if file.is_dir() {
            continue;
        }

        // Read the file contents
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        contents.insert(name, data);
    }

    Ok(contents)
}

/// Packs a directory containing Winamp sprite files back into a WSZ file
///
/// # Arguments
///
/// * `dir_path` - Path to the directory containing the extracted skin
/// * `output_path` - Path where the WSZ file will be saved
///
/// # Returns
///
/// A Result indicating success or failure
pub fn pack_wsz_dir<P: AsRef<Path>>(dir_path: P, output_path: P) -> Result<()> {
    let dir_path = dir_path.as_ref();
    let output_path = output_path.as_ref();

    // Create a file to write the ZIP/WSZ data
    let file = File::create(output_path)?;
    let mut zip = zip::ZipWriter::new(file);

    // Options for file compression
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);

    // Keep track of files we've added to prevent duplicates
    let mut added_files = std::collections::HashSet::new();

    // Process each subdirectory (which should be BMP names)
    let mut entries = fs::read_dir(dir_path)?.filter_map(|e| e.ok()).collect::<Vec<_>>();

    // Sort entries for consistent output
    entries.sort_by(|a, b| a.path().cmp(&b.path()));

    // First, collect all sprites by BMP file
    let mut sprites_by_sheet = HashMap::new();

    let sprite_manager = SpriteManager::new();
    let all_sprite_defs = sprite_manager.get_sprite_definitions();
    let all_sprite_sheets = SpriteManager::sprite_sheet_names();

    for entry in &entries {
        let path = entry.path();
        if path.is_dir() {
            let bmp_name = path
                .with_extension("BMP")
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_uppercase();

            // Skip if not a BMP directory
            if !all_sprite_sheets.contains(&bmp_name) {
                continue;
            }

            let sprite_defs = all_sprite_defs
                .values()
                .filter(|def| def.sprite_sheet == bmp_name)
                .collect::<Vec<_>>();

            if sprite_defs.is_empty() {
                continue;
            }

            // Load all sprite images from this directory
            let mut sprite_images = HashMap::new();
            let subdir_entries = fs::read_dir(&path)?.filter_map(|e| e.ok()).collect::<Vec<_>>();

            for subentry in subdir_entries {
                let sprite_path = subentry.path();
                if sprite_path.is_file() && sprite_path.extension().map_or(false, |ext| ext == "png") {
                    let sprite_name = sprite_path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or_default()
                        .to_string()
                        .to_uppercase();

                    if !sprite_defs.iter().any(|def| def.name == sprite_name) {
                        continue;
                    }

                    // Load the sprite image
                    match image::open(&sprite_path) {
                        Ok(img) => {
                            let rgba_img = img.to_rgba8();
                            sprite_images.insert(sprite_name, rgba_img);
                        }
                        Err(e) => {
                            eprintln!("Error loading sprite {}: {}", sprite_path.display(), e);
                        }
                    }
                }
            }

            sprites_by_sheet.insert(bmp_name, sprite_images);
        }
    }

    // Now reconstruct and add each BMP file
    for (sheet_name, sprite_images) in sprites_by_sheet {
        // Reconstruct the BMP image
        match sprite_manager.construct_sprite_sheet(&sprite_images, &sheet_name) {
            Ok(sprite_sheet) => {
                // Create a buffer to hold the BMP data
                let mut bmp_data = Vec::new();
                let mut cursor = Cursor::new(&mut bmp_data);

                // Save the image as BMP
                sprite_sheet.write_to(&mut cursor, ImageFormat::Bmp)?;

                // Add the BMP to the ZIP
                let file_name = format!("{}", sheet_name);
                zip.start_file(&file_name, options)?;
                zip.write_all(&bmp_data)?;

                added_files.insert(file_name);
            }
            Err(e) => {
                eprintln!("Error reconstructing {}.BMP: {}", sheet_name, e);
            }
        }
    }

    // Add any non-BMP files from the top level directory
    for entry in entries {
        let path = entry.path();
        if path.is_file() {
            let file_name = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_string();

            // Skip if we've already added this file
            if added_files.contains(&file_name) {
                continue;
            }

            // Read the file data
            let file_data = fs::read(&path)?;

            // Add the file to the ZIP
            zip.start_file(&file_name, options)?;
            zip.write_all(&file_data)?;

            added_files.insert(file_name);
        }
    }

    // Finalize the ZIP file
    zip.finish()?;

    Ok(())
}
