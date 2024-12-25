use aoc_utils::*;

use itertools::Itertools;

fn main() {
    part1();
    part2();
    part3();
}

fn cost(c: char) -> i32 {
    match c {
        'A' => 0,
        'B' => 1,
        'C' => 3,
        'D' => 5,
        _ => unreachable!(),
    }
}

fn part1() {
    let input = read_input_to_string(ec(1, 1));

    let result: i32 = input.chars().map(cost).sum();

    println!("{}", result);
}

fn part2() {
    let input = read_input_to_string(ec(1, 2));

    assert!(input.len() % 2 == 0);

    let result: i32 = input
        .chars()
        .tuples()
        .map(|(c1, c2)| match (c1, c2) {
            ('x', 'x') => 0,
            ('x', c) | (c, 'x') => cost(c),
            (a, b) => cost(a) + cost(b) + 2,
        })
        .sum();

    println!("{}", result);
}

fn part3() {
    let input = read_input_to_string(ec(1, 3));

    assert!(input.len() % 3 == 0);

    let result: i32 = input
        .chars()
        .tuples()
        .map(|(c1, c2, c3)| match (c1, c2, c3) {
            ('x', 'x', 'x') => 0,
            ('x', 'x', c) | (c, 'x', 'x') | ('x', c, 'x') => cost(c),
            (a, b, 'x') | (a, 'x', b) | ('x', a, b) => cost(a) + cost(b) + 2,
            (a, b, c) => cost(a) + cost(b) + cost(c) + 6,
        })
        .sum();

    println!("{}", result);
}
