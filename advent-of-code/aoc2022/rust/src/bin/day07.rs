use std::{collections::HashMap, fs, path::Path};

fn main() {
    let input = fs::read_to_string("../input/07.txt").unwrap();

    let dirs = HashMap::new();

    let mut wd = Path::new("/");
    let commands = input.split("$ ");
}
