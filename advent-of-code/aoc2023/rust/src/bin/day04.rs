use std::collections::HashSet;

use aoc2023::*;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read_input_to_lines(4);
    let mut sum = 0;
    for line in input {
        let got_right = solve_card(&line) as u32;
        if got_right > 0 {
            sum += 2_u32.pow(got_right - 1);
        }
    }
    println!("{sum}");
}

fn part2() {
    let input = read_input_to_lines(4);
    let mut counts = vec![1; input.len()];
    for (i, line) in input.iter().enumerate() {
        let got_right = solve_card(line);
        for j in 1..=got_right {
            counts[i + j] += counts[i];
        }
    }
    println!("{}", counts.iter().sum::<usize>());
}

fn solve_card(line: &str) -> usize {
    let (_, row) = line.split_once(':').unwrap();
    let (winning, mine) = row.split_once('|').unwrap();
    let winning = winning
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<HashSet<_>>();
    let mine = mine
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<HashSet<_>>();
    winning.intersection(&mine).count()
}
