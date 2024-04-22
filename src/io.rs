use crate::globals::PATH;
use crate::logic::Rom;
use crc32fast::Hasher;
use mslnk::ShellLink;
use serde_json::{json, to_string_pretty, Value};
use std::fs::{metadata, read_dir, read_to_string, write, File};
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn path_exists(path: &str) -> bool {
    metadata(path).is_ok()
}

pub fn write_json_to_file(path: &str, json: Value) -> Result<(), std::io::Error> {
    write(path, to_string_pretty(&json).unwrap())
}

pub fn read_json_file(path: &str) -> Result<Value, &str> {
    let file = read_to_string(path);

    Ok(serde_json::from_str(file.unwrap().as_str()).unwrap())
}

pub fn traverse_directory(directory: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut paths = Vec::new();

    // Iterate through the entries in the directory
    for entry in read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a directory
        if path.is_dir() {
            // If it's a directory, recursively call traverse_directory
            let mut sub_paths = traverse_directory(&path)?;
            paths.append(&mut sub_paths);
        } else {
            // If it's a file, add its path to the vector
            paths.push(path);
        }
    }
    Ok(paths)
}

pub fn get_config() -> Value {
    let config_result = read_json_file(PATH);
    match config_result {
        Ok(val) => val,
        Err(val) => {
            println!("{}", val);
            panic!("Failed reading config");
        }
    }
}

pub fn read_crc_checksum(path: PathBuf) -> u32 {
    let mut file = File::open(path).unwrap();

    // Initialize the CRC32 hasher
    let mut hasher = Hasher::new();

    // Read the file contents in chunks and update the hasher
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    // Get the CRC32 checksum value
    hasher.finalize()
}

pub fn write_rom_shortcut(output_dir: &str, retroarch_path: &str, core_dir: &str, rom: Rom) {
    match std::env::consts::OS {
        "linux" => {
            let _ = write_json_to_file(
                &format!("{}/{}.json", output_dir, rom.name),
                json!({
                    "rom": rom.path,
                    "retroarch": retroarch_path,
                    "name": rom.name,
                    "core": format!("{}/{}", core_dir, rom.console.core_name())
                }),
            );
        }
        "windows" => create_windows_lnk(output_dir, retroarch_path, core_dir, rom),
        // "macos" => println!("Running on macOS!"),
        _ => println!("Running on an unknown operating system!"),
    };
}

fn create_windows_lnk(output_dir: &str, retroarch_path: &str, core_dir: &str, rom: Rom) {
    let target = format!(
        r"{} -L {}/{} {}",
        retroarch_path,
        core_dir,
        rom.console.core_name(),
        rom.path
    );
    let lnk = format!("{}/{}.lnk", output_dir, rom.name);
    let sl = ShellLink::new(target).unwrap();
    sl.set_name(Option(rom.name.to_string()));
    sl.create_lnk(lnk).unwrap();
}
