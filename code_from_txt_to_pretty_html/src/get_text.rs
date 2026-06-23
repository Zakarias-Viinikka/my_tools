use std::error::Error;

pub fn get_text(path: &str) -> Result<String, Box<dyn Error>> {
    Ok(std::fs::read_to_string(path)?)
}
