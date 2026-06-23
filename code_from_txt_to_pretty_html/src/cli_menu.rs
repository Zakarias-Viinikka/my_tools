//use std::fmt::format;
use std::io;
use std::process::Command;

use std::path::Path;

pub fn get_file_name() -> Result<String, String> {
    let mut aditional_info = "".to_string();
    loop {
        //clear_console();
        display_main_menu(&aditional_info);
        match read_line() {
            Ok(input) if input == "quit".to_string() => {
                return Err("User requested to quit".to_string());
            }
            Ok(file_name) => {
                if check_if_file_looks_good(&file_name) {
                    let path = file_name + ".rs";
                    return Ok(path);
                } else {
                    aditional_info = format!("File [{file_name}.rs] not found, try again.");
                }
            }
            Err(e) => return Err(e.to_string()),
        }
    }
}

fn check_if_file_looks_good(file_name: &str) -> bool {
    Path::new(&(file_name.to_owned() + ".rs")).exists()
}

fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().ok();
    } else {
        Command::new("clear").status().ok();
    }
}

fn display_main_menu(aditional_info: &str) {
    println!("Instructions:");
    println!("Write the filename without the '.rs'");
    println!("For now the program only handles rs's");
    println!("");
    println!("enter 'quit' to exit program instead");

    println!("");
    println!("waiting for user input:");

    if aditional_info != "" {
        println!("");
        println!("{aditional_info}");
    }
}

fn read_line() -> io::Result<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line.trim().to_string())
}

fn display_confirmation_menu(pretty_text: &str, aditional_info: &str) {
    clear_console();
    println!("{pretty_text} \n");
    print!("\x1b[0m");

    println!("Instructions:");
    println!("y to confirm:");
    println!("n to quit: \n");

    println!("{aditional_info}");
}

pub fn confirm_all_good(text: &str) -> bool {
    let aditional_info = "".to_string();
    loop {
        display_confirmation_menu(&text, &aditional_info);
        match read_line() {
            Ok(input) if input == "n".to_string() => {
                break;
            }
            Ok(input) if input == "y".to_string() => return true,
            Err(e) => panic!("{}", e),
            _ => (),
        }
    }
    false
}
