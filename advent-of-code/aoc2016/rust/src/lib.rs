use std::fs;

pub fn read_input_to_string(day: u8) -> String {
    fs::read_to_string(format!("../input/{:02}.txt", day)).unwrap()
        .trim()
        .to_owned()
}

pub fn read_input_to_lines(day: u8) -> Vec<String> {
    fs::read_to_string(format!("../input/{:02}.txt", day)).unwrap()
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_owned())
        .collect()
}

pub fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
    where T: Clone
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}