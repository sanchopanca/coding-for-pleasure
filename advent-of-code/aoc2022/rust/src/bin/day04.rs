use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("../input/04.txt").unwrap();
    let mut res1 = 0;
    let mut res2 = 0;
    for l in input.lines() {
        let x: Vec<u32> = l
            .split(|c| !char::is_ascii_digit(&c))
            .map(|d| d.parse().unwrap())
            .collect();
        let set1: HashSet<u32> = HashSet::from_iter(x[0]..=x[1]);
        let set2: HashSet<u32> = HashSet::from_iter(x[2]..=x[3]);
        if set1.is_subset(&set2) || set1.is_superset(&set2) {
            res1 += 1;
        }
        if !(&set1 & &set2).is_empty() {
            res2 += 1;
        }
    }
    println!("{}", res1);
    println!("{}", res2);
}
