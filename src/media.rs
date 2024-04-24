use attohttpc::get;

use crate::logic::Rom;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str;

pub fn download_title(rom: Rom, title_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let encoded_title = rom.name.replace(" ", "%20");
    let console_title = rom.console.to_title();

    let url_str = format!(
        "https://github.com/libretro-thumbnails/{}/raw/master/Named_Titles/{}.png",
        console_title.replace(" ", "_"),
        encoded_title
    );
    let response = get(url_str).send()?;
    let bytes = response.bytes()?;
    // let content_string = str::from_utf8(&bytes)?;
    // let content_type = response
    //     .headers()
    //     .get("content-type")
    //     .unwrap()
    //     .to_str()
    //     .unwrap();
    // if content_type == "image/png" {
    //     return download_title(&content_string[..content_string.len() - 4], title_path).await;
    // }
    println!("Attempting to write image to {}", title_path);
    let mut out = File::create(Path::new(title_path))?;
    out.write_all(&bytes)?;
    Ok(())
}
