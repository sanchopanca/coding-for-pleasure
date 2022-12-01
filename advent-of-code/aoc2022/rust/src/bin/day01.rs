use std::fs;

fn main() {
    let input = fs::read_to_string("../input/01.txt").unwrap();

    let mut elves = Vec::new();
    for elf in input.split("\n\n") {
        let mut sack: Vec<u32> = Vec::new();
        for foodstuff in elf.trim().split('\n') {
            sack.push(foodstuff.parse().unwrap())
        }
        elves.push(sack);
    }

    let mut elves: Vec<u32> = elves.iter().map(|e| e.iter().sum()).collect();
    elves.sort();
    elves.reverse();

    println!("{}", elves[0]);
    println!("{}", elves[0] + elves[1] + elves[2]);
}
