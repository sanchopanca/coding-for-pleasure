use std::iter::zip;

use aoc2023::*;

fn main() {
    // part1();
    part2();
}

fn part2() {
    let input = read_input_to_lines(12);

    let mut sum = 0;
    for line in input {
        let (condition1, condition2) = line.split_once(' ').unwrap();
        let condition1 = [condition1, condition1, condition1, condition1, condition1].join("?");
        let condition2 = [condition2, condition2, condition2, condition2, condition2].join(",");
        let condition2 = condition2
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let questions = condition1.chars().filter(|c| *c == '?').count();
        println!("{condition1}");
    }
}

fn part1() {
    let input = read_input_to_lines(12);

    let mut sum = 0;
    for line in input {
        let (condition1, condition2) = line.split_once(' ').unwrap();
        let condition2 = condition2
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let questions = condition1.chars().filter(|c| *c == '?').count();
        for mask in 0..(1 << questions) {
            let condition1 = substitute(condition1, mask, questions);
            if check(&condition1, &condition2) {
                sum += 1
            }
        }
    }
    println!("Part 1: {}", sum);
}

fn check(condition1: &str, condition2: &[i32]) -> bool {
    let broken_segements = condition1
        .split('.')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    if broken_segements.len() != condition2.len() {
        return false;
    }

    zip(broken_segements, condition2).all(|(a, b)| a.len() == *b as usize)
}

fn substitute(condition: &str, mut mask: u32, len: usize) -> String {
    let mut condition = condition.to_owned();
    for _ in 0..len {
        let to = if (mask & 1) == 1 { "#" } else { "." };
        condition = condition.replacen('?', to, 1);
        mask >>= 1;
    }
    condition
}
