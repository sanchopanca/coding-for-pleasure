use std::collections::HashMap;

use aoc2023::*;
use regex::Regex;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read_input_to_lines(8);
    let instructions = input[0].to_owned();
    let mut instructions = instructions.chars().cycle();

    let map = parse_input(&input);

    let mut steps = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        let (left, right) = map.get(current).unwrap();
        match instructions.next() {
            Some('L') => current = left,
            Some('R') => current = right,
            _ => panic!(),
        }
        steps += 1;
    }

    println!("{steps}");
}

fn part2() {
    let input = read_input_to_lines(8);
    let instructions = input[0].to_owned();
    let instructions = instructions.chars().cycle();

    let map = parse_input(&input);

    let starting = map.keys().filter(|k| k.ends_with('A')).collect::<Vec<_>>();

    let mut stops = Vec::new();
    for position in starting {
        let mut steps = 0u64;
        let mut instructions = instructions.clone();
        let mut current = position;
        while !current.ends_with('Z') {
            let next_instruction = instructions.next().unwrap();
            let (left, right) = map.get(current).unwrap();
            match next_instruction {
                'L' => current = left,
                'R' => current = right,
                _ => panic!(),
            }
            steps += 1;
        }
        stops.push(steps);
    }
    println!("{}", lcm(&stops));
}

fn parse_input(input: &[String]) -> HashMap<String, (String, String)> {
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let lines = input.iter().skip(1).collect::<Vec<_>>();

    let mut map = HashMap::new();
    for line in lines {
        let captures = re.captures(line).unwrap();
        let (_, [name, left, right]) = captures.extract();
        map.insert(name.to_owned(), (left.to_owned(), right.to_owned()));
    }

    map
}
