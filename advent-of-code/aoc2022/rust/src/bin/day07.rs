use std::fs;

use counter::Counter;

fn main() {
    let input = fs::read_to_string("../input/07.txt").unwrap();

    let mut dirs: Counter<String> = Counter::new();

    let mut wd = vec![];
    for command in input.split("$ ") {
        if command.starts_with("cd") {
            let dir = command.split(' ').nth(1).unwrap();
            if dir.starts_with("..") {
                wd.pop();
            } else {
                wd.push(dir.trim().to_owned());
            }
        } else {
            for entry in command.lines().skip(1) {
                if entry.starts_with("dir") {
                    continue;
                }
                update_size(
                    &wd,
                    &mut dirs,
                    entry.split(' ').nth(0).unwrap().parse::<usize>().unwrap(),
                );
            }
        }
    }
    let small_size: usize = dirs.values().filter(|x| **x <= 100_000).sum();
    println!("{small_size}");

    let space_left = 70_000_000 - dirs[&"/".to_owned()];
    let mut sizes = dirs.values().map(|x| *x).collect::<Vec<usize>>();
    sizes.sort();

    for size in sizes {
        if (space_left + size) > 30_000_000 {
            println!("{size}");
            break;
        }
    }
}

fn update_size(working_directory: &Vec<String>, dirs: &mut Counter<String>, size: usize) {
    for i in 0..working_directory.len() {
        let path = working_directory[..=i].join("/");
        dirs[&path] += size;
    }
}
