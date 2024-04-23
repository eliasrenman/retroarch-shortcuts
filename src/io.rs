use crate::globals::PATH;
use crate::logic::Rom;
use crc32fast::Hasher;
use serde_json::{json, to_string_pretty, Value};
use std::fs::{self, metadata, read_dir, read_to_string, write, File};
use std::io::prelude::*;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

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
        "windows" => create_windows_exe(output_dir, retroarch_path, core_dir, rom),
        // "macos" => println!("Running on macOS!"),
        _ => println!("Running on an unknown operating system!"),
    };
}

fn create_windows_exe(output_dir: &str, retroarch_path: &str, core_dir: &str, rom: Rom) {
    let name = rom.name.clone();
    // Write content to the batch script file
    let temp_rs = r".\temp.rs";
    let _ = write_file(&temp_rs, retroarch_path, core_dir, rom);
    compile_file(&temp_rs, format!(r"{}\{}.exe", output_dir, name).as_str());
    // Delete the temporary batch script
    fs::remove_file(temp_rs).expect("Failed to delete temporary batch script");
}
/// Create a plugin file at runtime which will be converted to shared library
fn write_file(path: &str, retroarch_path: &str, core_dir: &str, rom: Rom) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    file.write_all(
        format!(
            r#"
        use std::process::Command;
        fn main() {{
            let _ = Command::new(r"{retroarch_path}")
            .arg(r"-L")
            .arg(r"{core_dir}\{core_name}")
            .arg(r"{rom_path}")
            .output()
            .expect("Failed to execute retroarch.exe");
        }}
    "#,
            retroarch_path = retroarch_path,
            core_dir = core_dir,
            core_name = rom.console.core_name(),
            rom_path = rom.path.to_str().unwrap()
        )
        .as_bytes(),
    )?;

    Ok(())
}

fn compile_file(path: &str, output_path: &str) {
    let mut compile_file = Command::new("cmd");
    compile_file
        .args(&[
            "/C",
            "rustc",
            "-o",
            output_path,
            "--target=x86_64-pc-windows-msvc",
            path,
        ])
        .status()
        .expect("process failed to execute");
}
