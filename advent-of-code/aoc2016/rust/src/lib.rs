use std::fs;

pub fn read_input_to_string(day: u8) -> String {
    fs::read_to_string(format!("../input/{:02}.txt", day)).unwrap().trim().to_owned()
}