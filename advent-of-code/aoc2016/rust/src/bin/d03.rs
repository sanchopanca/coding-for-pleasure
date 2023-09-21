use itertools::Itertools;
use aoc2016::{read_input_to_lines, transpose};
fn main() {
    let s = read_input_to_lines(3);
    let triplets: Vec<Vec<i32>> = s.into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut valid2 = 0;
    for (a, b, c) in transpose(&triplets).iter().flatten().tuples() {
        let mut triplet = vec![*a, *b, *c];
        triplet.sort();
        if triplet[2] < triplet[0] + triplet[1] {
            valid2 += 1;
        }
    }

    let mut valid = 0;
    for mut triplet in triplets {
        triplet.sort();
        if triplet[2] < triplet[0] + triplet[1] {
            valid += 1;
        }
    }

    println!("part 1: {}", valid);
    println!("part 2: {}", valid2);
}