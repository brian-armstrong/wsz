# wsz

Rust crate to unpack, pack, and screenshot Winamp skins. Borrows heavily from [https://github.com/captbaritone/webamp/tree/master](webamp).

[docs.rs](https://docs.rs/wsz/latest/wsz)

## Utility

This crate is both a library and a utility. The utility offers the following options

 - `--extract {path_to_wsz_file}` creates a directory with extracted sprites
 - `--pack {path_to_extracted_directory}` packs a directory back into a wsz file
 - `--screenshot {path_to_wsz_file}` creates a mockup screenshot into screenshot.png

## License

MIT License. Winamp is a copyright of Nullsoft.
