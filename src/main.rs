use std::process::exit;

use create_config::create_config;
use globals::PATH;
use inquire::Select;
use io::path_exists;
use logic::{install, uninstall};

mod console;
mod create_config;
mod database;
mod generated;
mod globals;
mod io;
mod logic;

fn main() {
    loop {
        if !path_exists(PATH) {
            println!("Missing config file, Please create one.");
            create_config()
        }
        main_selector()
    }
}

fn main_selector() {
    let main_selector = Select::new(
        "What do you want to do today?",
        vec![
            "Install / update - install shortcuts in output directory",
            "Uninstall - Remove shortcuts",
            "Update Config",
            "Quit",
        ],
    )
    .prompt()
    .unwrap();
    match main_selector {
        "Install / update - install shortcuts in output directory" => install(),
        "Uninstall - Remove shortcuts" => uninstall(),
        "Update Config" => create_config(),
        "Quit" => exit(0),
        _ => panic!("Invalid option"),
    }
}
