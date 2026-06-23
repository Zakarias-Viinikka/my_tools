#!/usr/bin/env -S cargo run --

mod cli_menu;
mod get_text;
mod prettify_text;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = cli_menu::get_file_name()?;
    let text = get_text::get_text(&file_path)?;

    let pretty_text = prettify_text::prettify_text(text);

    let check_if_user_happy_with_text = cli_menu::confirm_all_good(&pretty_text);

    if check_if_user_happy_with_text {
        let new_file_name = new_file_name(file_path);
        println!("Created file {new_file_name}");
        std::fs::write(new_file_name, pretty_text)?;
        Ok(())
    } else {
        println!("user not happy with file they picked");
        Ok(())
    }
}

fn new_file_name(file_path: String) -> String {
    if let Some((file_name, _)) = file_path.rsplit_once('.') {
        return format!("{file_name}_prettified.txt");
    } else {
        panic!("failed at creating/writing to file");
    }
}
