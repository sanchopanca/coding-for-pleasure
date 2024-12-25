pub mod intcode;

use std::{fs, io::Write};

use regex::Regex;

pub fn read_input_to_string(day: u8) -> String {
    fs::read_to_string(format!("../input/{:02}.txt", day))
        .unwrap()
        .trim()
        .to_owned()
}

pub fn read_input_to_lines(day: u8) -> Vec<String> {
    fs::read_to_string(format!("../input/{:02}.txt", day))
        .unwrap()
        .trim_end()
        .split('\n')
        .map(|s| s.trim().to_owned())
        .collect()
}

pub fn read_input_to_char_vectors(day: u8) -> Vec<Vec<char>> {
    read_input_to_lines(day)
        .iter()
        .map(|s| s.chars().collect())
        .collect()
}

pub fn transpose<T>(v: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

pub fn extract_all_numbers<T>(line: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(line)
        .map(|m| m.as_str().parse::<T>().unwrap())
        .collect()
}

pub fn skewer_numbers<T>(numbers: &[T]) -> T
where
    T: std::fmt::Display,
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let as_string = numbers.iter().map(|x| x.to_string()).collect::<String>();

    as_string.parse().unwrap()
}

pub fn lcm<T>(numbers: &[T]) -> T
where
    T: Copy + num::Integer, //     T: std::cmp::PartialOrd + std::ops::Mul<Output = T> + std::ops::Add<Output = T>,
{
    numbers
        .iter()
        .fold(numbers[0], |a, b| num::integer::lcm(a, *b))
}

pub fn pretty_print(v: &[Vec<char>]) {
    for row in v {
        println!("{}", row.iter().collect::<String>());
    }
}

pub fn pretty_print_numbers<T>(v: &[Vec<T>])
where
    T: std::fmt::Display,
{
    for row in v {
        println!(
            "{}",
            row.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("\t")
        );
    }
}

pub fn input<T>(prompt: &str) -> T
where
    T: std::fmt::Display,
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    print!("{prompt} > ");
    std::io::stdout().flush().unwrap();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}
