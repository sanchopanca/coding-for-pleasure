use std::collections::HashSet;

use aoc_utils::*;

use substring::Substring;

fn main() {
    part1();
    part2();
    // part3();
}

fn part1() {
    let input = read_input_to_string(ec(2, 1));
    let (words, inscription) = input.split_once("\n\n").unwrap();
    let words = words
        .strip_prefix("WORDS:")
        .unwrap()
        .split(',')
        .collect::<Vec<_>>();

    let mut result = 0;
    for word in words {
        result += inscription.split(word).count() - 1
    }
    println!("{result}")
}

fn part2() {
    let input = read_input_to_string(ec(2, 2));

    let (words, inscription) = input.split_once("\n\n").unwrap();
    let words = words
        .strip_prefix("WORDS:")
        .unwrap()
        .split(',')
        .collect::<Vec<_>>();

    let mut result = HashSet::new();
    for word in words {
        result.extend(positions(inscription, word));
        result.extend(positions(
            inscription,
            &word.chars().rev().collect::<String>(),
        ));
    }
    println!("{:?}", result.len())
}

fn positions(haystack: &str, needle: &str) -> HashSet<usize> {
    let mut result = HashSet::new();

    if haystack.starts_with(needle) {
        result.extend(0..needle.len());
    }

    let l = haystack.len();

    let mut new_haystack = haystack;

    for i in 0..l {
        new_haystack = new_haystack.substring(1, new_haystack.len());
        if new_haystack.starts_with(needle) {
            result.extend((i + 1)..=(i + needle.len()));
        }
    }
    result
}
