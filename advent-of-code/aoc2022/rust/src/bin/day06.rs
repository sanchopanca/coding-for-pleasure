use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("../input/06.txt")
        .unwrap()
        .trim()
        .to_owned();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut res = 4;
    for (a, b, c, d) in input.chars().tuple_windows() {
        if HashSet::<_>::from_iter([a, b, c, d]).len() == 4 {
            break;
        }
        res += 1;
    }
    println!("{}", res);
}

fn part2(input: &str) {
    let mut res = 14;
    for from in 0.. {
        if HashSet::<_>::from_iter(input.chars().skip(from).take(14)).len() == 14 {
            break;
        }
        res += 1;
    }
    println!("{}", res);
}
