use crate::logic::Rom;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::str;

pub async fn download_title(rom: Rom, title_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let encoded_title = rom.name.replace(" ", "%20");
    let console_title = rom.console.to_title();

    let url_str = format!(
        "https://github.com/libretro-thumbnails/{}/raw/master/Named_Titles/{}.png",
        console_title.replace(" ", "_"),
        encoded_title
    );
    let mut response = attohttpc::get(url_str).send().await?;
    let bytes = response.bytes()?;
    let content_string = str::from_utf8(&bytes)?;

    if response.headers().get("content-type") == Some(&"image/png".into()) {
        return download_cover(&content_string[..content_string.len() - 4], cover_path).await;
    }

    let mut out = File::create(cover_path)?;
    out.write_all(&bytes)?;
    Ok(())
}
