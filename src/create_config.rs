use inquire::Text;
use serde_json::json;

use crate::io::write_json_to_file;

pub fn create_config() {
    let retro_arch_exec = Text::new("Path to RetroArch executable?").prompt().unwrap();
    let emulator_path = Text::new("Path to emulator directory?").prompt().unwrap();
    let rom_directory = Text::new("Path to rom directory?").prompt().unwrap();
    // let device_selector = Select::new("What device are you on?", vec!["ROG Ally", "Other"])
    //     .prompt()
    //     .unwrap();
    // let output_path = match device_selector {
    //     "ROG Ally" => "Hardcoded ROG Ally value.".to_string(),
    //     "Other" => Text::new("Path to shortcut output?").prompt().unwrap(),
    //     _ => panic!("Invalid option selected"),
    // };
    let output_path = Text::new("Path to shortcut output?").prompt().unwrap();
    let config = json!({
      "outputDir": output_path,
      "romDir": rom_directory,
      "emulatorDir": emulator_path,
      "retroArchExec": retro_arch_exec,
    });
    let result = write_json_to_file("./config.json", config);
    if result.is_err() {
        panic!("Something went wrong with writing to disk");
    }
    println!("Successfully created config!")
}
