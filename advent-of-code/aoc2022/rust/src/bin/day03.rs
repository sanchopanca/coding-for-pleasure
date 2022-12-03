use std::collections::HashSet;
use std::fs;

fn main() {
    p1();
    p2();
}

fn p1() {
    let input = fs::read_to_string("../input/03.txt").unwrap();
    let mut s = 0;
    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        assert!(first.len() == second.len());

        let first: HashSet<u8> = HashSet::from_iter(first.as_bytes().iter().cloned());
        let second: HashSet<u8> = HashSet::from_iter(second.as_bytes().iter().cloned());

        let common = &first & &second;
        assert!(common.len() == 1);

        s += score(*common.iter().next().unwrap()) as u32;
    }

    println!("{}", s)
}

fn p2() {
    let elves = fs::read_to_string("../input/03.txt")
        .unwrap()
        .lines()
        .map(|s| HashSet::from_iter(s.as_bytes().iter().cloned()))
        .collect::<Vec<HashSet<u8>>>();

    let mut found: HashSet<usize> = HashSet::new();

    let mut res = 0;
    'outer: for i in 0..elves.len() {
        if found.contains(&i) {
            continue;
        }
        for j in (i + 1)..elves.len() {
            if found.contains(&j) {
                continue;
            }
            for k in (j + 1)..elves.len() {
                if found.contains(&k) {
                    continue;
                }
                let intersection = &(&elves[i] & &elves[j]) & &elves[k];
                if intersection.len() == 1 {
                    res += score(*intersection.iter().next().unwrap()) as u32;
                    found.extend([i, j, k].iter());
                    continue 'outer;
                }
            }
        }
    }
    assert!(elves.len() == found.len());
    println!("{}", res);
}

fn score(item: u8) -> u8 {
    if item <= b'Z' {
        item - b'A' + 27
    } else {
        item - b'a' + 1
    }
}
