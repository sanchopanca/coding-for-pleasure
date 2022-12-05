use std::fs;

fn main() {
    let input = fs::read_to_string("../input/05.txt").unwrap();
    let mut split = input.split("\n\n");
    let stacks = parse_stacks_of_crates(split.next().unwrap());
    let instructions = parse_unloading_instructions(split.next().unwrap());

    part1(stacks.clone(), instructions.clone());
    part2(stacks, instructions);
}

fn part1(mut stacks: Vec<Vec<char>>, instructions: Vec<(usize, usize, usize)>) {
    for (n, from, to) in instructions {
        for _ in 0..n {
            let c = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(c);
        }
    }
    let res: String = stacks.iter().map(|v| v.last().unwrap()).collect();
    println!("{}", res);
}

fn part2(mut stacks: Vec<Vec<char>>, instructions: Vec<(usize, usize, usize)>) {
    for (n, from, to) in instructions {
        let range = (stacks[from - 1].len() - n)..;
        let mut drained: Vec<_> = stacks[from - 1].drain(range).collect();
        stacks[to - 1].append(&mut drained);
    }
    let res: String = stacks.iter().map(|v| v.last().unwrap()).collect();
    println!("{}", res);
}

fn parse_stacks_of_crates(s: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<_> = s.lines().collect();
    let indexes: Vec<usize> = lines
        .pop()
        .unwrap()
        .split("   ")
        .map(|x| x.trim().parse().unwrap())
        .collect();
    let len = *indexes.last().unwrap();
    lines.reverse();

    let mut res = Vec::new();
    for _ in 0..len {
        res.push(Vec::new());
    }

    for line in lines {
        let bytes = line.as_bytes();
        for i in 0..len {
            let index = i * 4 + 1;
            let letter = bytes[index];
            if letter != b' ' {
                res[i].push(letter as char);
            }
        }
    }
    res
}

fn parse_unloading_instructions(s: &str) -> Vec<(usize, usize, usize)> {
    let mut res = Vec::new();
    for line in s.lines() {
        let v: Vec<_> = line.split(' ').collect();
        res.push((
            v[1].parse().unwrap(),
            v[3].parse().unwrap(),
            v[5].parse().unwrap(),
        ));
    }
    res
}
