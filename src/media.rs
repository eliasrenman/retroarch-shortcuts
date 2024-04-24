use attohttpc::get;

use crate::logic::Rom;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str;

pub fn download_title(rom: Rom, title_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    download_thumbnail("Titles", rom, title_path)
}
pub fn download_boxart(rom: Rom, boxart_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    download_thumbnail("Boxarts", rom, boxart_path)
}
pub fn download_thumbnail(
    dtype: &str,
    rom: Rom,
    boxart_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let encoded_title = rom.name.replace(" ", "%20");
    let console_title = rom.console.to_title();

    let url_str = format!(
        "https://github.com/libretro-thumbnails/{}/raw/master/Named_{}/{}.png",
        console_title.replace(" ", "_"),
        dtype,
        encoded_title
    );
    let response = get(url_str).send()?;
    let bytes = response.bytes()?;
    let mut out = File::create(Path::new(boxart_path))?;
    out.write_all(&bytes)?;
    Ok(())
}
