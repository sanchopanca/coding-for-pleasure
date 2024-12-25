use std::collections::HashMap;

use aoc_utils::*;

fn main() {
    part1();
    part2();
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Color {
    R,
    G,
    B,
}

fn part1() {
    let games = parse_input();

    let mut sum = 0;
    'game: for (i, sets) in games.iter().enumerate() {
        for set in sets {
            for (number, color) in set {
                match color {
                    Color::R => {
                        if *number > 12 {
                            continue 'game;
                        }
                    }
                    Color::G => {
                        if *number > 13 {
                            continue 'game;
                        }
                    }
                    Color::B => {
                        if *number > 14 {
                            continue 'game;
                        }
                    }
                }
            }
        }
        sum += i + 1;
    }
    println!("{sum}");
}

fn part2() {
    let games = parse_input();

    let mut sum = 0;
    for sets in games {
        let mut max: HashMap<Color, u32> = HashMap::new();
        for set in sets {
            for (number, color) in set {
                let new = max.get(&color).unwrap_or(&0u32).max(&number);
                max.insert(color, *new);
            }
        }
        sum += max.get(&Color::R).unwrap_or(&0)
            * max.get(&Color::G).unwrap_or(&0)
            * max.get(&Color::B).unwrap_or(&0);
    }
    println!("{sum}");
}

fn parse_input() -> Vec<Vec<Vec<(u32, Color)>>> {
    let input = read_input_to_lines(2);

    let mut games = Vec::new();
    for line in input {
        let (_, sets) = line.split_once(": ").unwrap();
        let sets = sets.split("; ").collect::<Vec<_>>();
        let sets: Vec<Vec<(u32, Color)>> = sets
            .iter()
            .map(|set| set.split(", "))
            .map(|split| {
                split
                    .map(|s| {
                        let (number, color) = s.split_once(' ').unwrap();
                        let number = number.parse::<u32>().unwrap();
                        let color = match color {
                            "red" => Color::R,
                            "green" => Color::G,
                            "blue" => Color::B,
                            _ => unreachable!(),
                        };
                        (number, color)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        games.push(sets);
    }
    games
}
