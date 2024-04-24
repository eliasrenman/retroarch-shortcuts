use crate::{
    console::Console,
    database::GameDatabase,
    io::{get_config, path_exists, read_crc_checksum, traverse_directory, write_rom_shortcut},
    media::{download_boxart, download_title},
};
use serde::Serialize;
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::exit,
};

pub fn install() {
    println!("Installing shortcuts");
    let config = get_config();
    let rom_paths = get_rom_paths(config.clone());

    println!("Found {} roms", rom_paths.len());

    let roms = map_roms(rom_paths);
    println!("Managed to link {} roms", roms.len());
    let output_dir = config.get("outputDir").unwrap().as_str().unwrap();

    // Make sure to recursively create all the directories
    if !path_exists(format!(r"{}\titles", output_dir).as_str()) {
        let _ = fs::create_dir_all(format!(r"{}\titles", output_dir).as_str());
    }
    if !path_exists(format!(r"{}\boxart", output_dir).as_str()) {
        let _ = fs::create_dir_all(format!(r"{}\boxart", output_dir).as_str());
    }

    let retro_arch_exec = config.get("retroArchExec").unwrap();
    let core_dir = config.get("emulatorDir").unwrap();
    for rom in roms {
        println!("Writing {} to file", rom.name);
        let _ = write_rom_shortcut(
            output_dir,
            retro_arch_exec.as_str().unwrap(),
            core_dir.as_str().unwrap(),
            rom.clone(),
        );

        println!("Starting download of media");

        let _ = download_title(
            rom.clone(),
            format!(r"{}\titles\{}.png", output_dir, rom.clone().name).as_str(),
        );
        let _ = download_boxart(
            rom.clone(),
            format!(r"{}\boxart\{}.png", output_dir, rom.clone().name).as_str(),
        );
    }
    println!("Finished creating shortcuts");
    exit(0);
}

fn map_roms(rom_paths: Vec<PathBuf>) -> Vec<Rom> {
    let database = GameDatabase::new("./gamedb.buf.gzip", true).unwrap();

    rom_paths
        .iter()
        .filter_map(|path| {
            let checksum = read_crc_checksum(path.clone());
            let game_entry_option = database.get_by_crc(checksum);
            if game_entry_option.is_none() {
                println!(
                    "Game entry not found for {}",
                    path.file_name().unwrap().to_str().unwrap()
                );
                return None;
            }
            let game_entry = game_entry_option.unwrap();
            let console = Console::by_title(game_entry.get_console());
            return Some(Rom {
                path: path.clone(),
                checksum,
                console: console,
                name: game_entry.get_name().to_string(),
            });
        })
        .collect()
}

#[derive(Serialize, Clone)]
pub struct Rom {
    pub path: PathBuf,
    pub checksum: u32,
    pub console: Console,
    pub name: String,
}

fn get_rom_paths(config: Value) -> Vec<PathBuf> {
    let rom_path = config.get("romDir").unwrap().as_str().unwrap();
    let paths = traverse_directory(Path::new(rom_path));
    match paths {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            panic!("Failed reading rom directory")
        }
    }
}

pub fn uninstall() {
    println!("Uninstalling shortcuts");
    let config = get_config();
    let output_dir = config.get("outputDir").unwrap().as_str().unwrap();
    let _ = fs::remove_dir_all(Path::new(output_dir));

    exit(0);
}
