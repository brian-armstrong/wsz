use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use wsz::archive::{pack_wsz_dir, unpack_wsz};
use wsz::sprites::SpriteManager;
use wsz::Wsz;

/// Extracts the name of a file without its extension
fn get_filename_without_extension(path: &str) -> String {
    let path = Path::new(path);
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string()
}

fn print_usage(program: &str) {
    eprintln!("Usage:");
    eprintln!("  Extract:       {} --extract <path_to_wsz_file>", program);
    eprintln!("  Pack:          {} --pack <directory_to_pack>", program);
    eprintln!("  Screenshot:    {} --screenshot <path_to_wsz_file>", program);
}

fn pack_sprites(args: &Vec<String>) {
    if args.len() < 3 {
        eprintln!("Error: No directory specified for packing");
        print_usage(&args[0]);
        process::exit(1);
    }

    let dir_path = Path::new(&args[2]);
    println!("Packing directory: {}", dir_path.display());

    // Create output WSZ path by adding .wsz extension to directory name
    let wsz_name = format!(
        "{}.wsz",
        dir_path.file_name().and_then(|s| s.to_str()).unwrap_or("skin")
    );

    let output_path = dir_path.with_file_name(wsz_name);

    match pack_wsz_dir(dir_path, &output_path) {
        Ok(()) => println!("Successfully packed WSZ file!"),
        Err(err) => {
            eprintln!("Error packing WSZ file: {}", err);
            process::exit(1);
        }
    }
}

fn extract_sprites(args: &Vec<String>) {
    // Extract mode
    let wsz_path = &args[2];

    // Extract base directory name from wsz filename
    let base_dir = get_filename_without_extension(wsz_path);

    // Try to unzip the file
    match unpack_wsz(wsz_path) {
        Ok(contents) => {
            println!("Found {} files", contents.len());

            let sprite_manager = SpriteManager::new();
            let sprite_sheets = SpriteManager::sprite_sheet_names();

            for sheet in sprite_sheets {
                // Extract sprites from this BMP
                match sprite_manager.extract_sprite_sheet_from_archive(&contents, &sheet) {
                    Ok(sprites) => {
                        if !sprites.is_empty() {
                            // Get BMP name without path and extension for directory name and sprite lookup
                            let bmp_base = Path::new(&sheet)
                                .file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or(&sheet)
                                .to_uppercase();

                            let sprite_dir = PathBuf::from(&base_dir).join(&bmp_base);
                            fs::create_dir_all(&sprite_dir)
                                .expect(&format!("Failed to create directory for {}", bmp_base));

                            // Save each sprite as a PNG file in appropriate directory
                            for (name, sprite_img) in &sprites {
                                if sprite_img.width() == 0 || sprite_img.height() == 0 {
                                    println!("Skipping sprite {} because it has no size", name);
                                    continue;
                                }

                                let output_path = sprite_dir.join(format!("{}.png", name));
                                sprite_img
                                    .save(&output_path)
                                    .expect(&format!("Failed to save sprite {}", name));
                            }

                            println!("Extracted {} sprites from {}", sprites.len(), sheet);
                        } else {
                            println!("No sprites found in {}", sheet);
                        }
                    }
                    Err(err) => {
                        eprintln!("Error extracting sprites from {}: {}", sheet, err);
                    }
                }
            }

            // preserve any non-bmp files
            for (name, content) in &contents {
                // remove any path from the name
                let name = Path::new(name).file_name().unwrap().to_str().unwrap();
                let lower_name = name.to_lowercase();
                if !lower_name.ends_with(".bmp") {
                    let output_path = PathBuf::from(&base_dir).join(&name);
                    fs::write(&output_path, content).expect(&format!("Failed to save {}", name));
                    println!("Saved {} to {}", name, output_path.display());
                }
            }
        }
        Err(err) => {
            eprintln!("Error unpacking WSZ file: {}", err);
            process::exit(1);
        }
    }
}

fn screenshot(args: &Vec<String>) {
    let wsz_path = &args[2];
    let screenshot_path = "screenshot.png";

    let wsz = Wsz::from_file_path(wsz_path).unwrap();
    let screenshot = wsz.render_screenshot().unwrap();
    screenshot.save(screenshot_path).unwrap();

    println!("Created screenshot at {}", screenshot_path);
}

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a path was provided
    if args.len() < 2 {
        print_usage(&args[0]);
        process::exit(1);
    }

    if args[1] == "--pack" {
        pack_sprites(&args);
    } else if args[1] == "--screenshot" {
        screenshot(&args);
    } else if args[1] == "--extract" {
        extract_sprites(&args);
    } else {
        eprintln!("Invalid command: {}", args[1]);
        print_usage(&args[0]);
        process::exit(1);
    }
}
