use std::fs;

pub fn day01() {
    let directions = fs::read_to_string("../input/01.txt").unwrap();
    let mut floor = 0;
    let mut found = false;
    for (i, d) in directions.chars().enumerate() {
        if d == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor == -1 && !found {
            println!("part2: {}", i + 1);
            found = true;
        }
    }
    println!("part1: {}", floor);
}