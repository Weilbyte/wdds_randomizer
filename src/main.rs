use ansi_term::{Style};
use inquire::{Text, Select, validator::StringValidator};
use std::path::Path;

pub mod data;
pub mod backup;
pub mod ttarchext;
pub mod randomize;

fn main() {
    println!("{}alking {}ead: The {}efinitive {}eries {}\n", 
        Style::new().bold().paint("W"),
        Style::new().bold().paint("D"),
        Style::new().bold().paint("D"),
        Style::new().bold().paint("S"),
        Style::new().bold().paint("Randomizer"),
    );

    let game_path_validator: StringValidator = &|input| {
        let path = Path::new(input);
        if path.exists() && path.join("WDC.exe").exists() && path.join("Archives").exists() {
            Ok(())
        } else {
            Err(String::from("Invalid game path"))
        }
    };

    let game_dir_path = Text::new("Game path")
        .with_help_message("Path to the folder containing WDC.exe")
        .with_validator(game_path_validator)
        .prompt()
        .unwrap();

    let archives_root = Path::new(&game_dir_path).join("Archives");

    let options: Vec<&str> = vec!["Randomize", "Restore original"];
    match Select::new("Select an option", &options).prompt() {
        Ok(opt) => {
            if opt.index == 0 {
                option_randomize(&archives_root);
            } else {
                option_restore(&archives_root);
            }
        },
        Err(e) => println!("Error selecting option: {}", e),
    }
}

fn option_restore(archives_root: &Path) {
    backup::restore(&Path::new(&archives_root));
    println!("Restored archives.");
}

fn option_randomize(archives_root: &Path) {
    backup::backup(&archives_root);

    randomize::randomize(&archives_root);
}
