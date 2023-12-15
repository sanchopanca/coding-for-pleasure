use aoc2023::*;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read_input_to_string(15);
    let sum: usize = input.split(',').map(hash).sum();
    println!("{}", sum);
}

fn part2() {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];

    let input = read_input_to_string(15);
    for part in input.split(',') {
        if part.contains('-') {
            let (a, _) = part.split_once('-').unwrap();
            let h = hash(a);
            let index = boxes[h].iter().position(|(s, _)| *s == a);
            if let Some(index) = index {
                boxes[h].remove(index);
            }
        } else {
            let (a, b) = part.split_once('=').unwrap();
            let h = hash(a);
            let index = boxes[h].iter().position(|(s, _)| *s == a);
            if let Some(index) = index {
                boxes[h][index] = (a, b.parse::<usize>().unwrap());
            } else {
                boxes[h].push((a, b.parse::<usize>().unwrap()));
            }
        }
    }

    let mut sum = 0;
    for (i, b) in boxes.iter().enumerate() {
        let mut box_sum = 0;
        for (j, (_, focal_len)) in b.iter().enumerate() {
            box_sum += (i + 1) * (j + 1) * focal_len;
        }
        sum += box_sum;
    }

    println!("{}", sum);
}

fn hash(s: &str) -> usize {
    let mut val = 0usize;
    for c in s.chars() {
        val += c as usize;
        val *= 17;
        val %= 256;
    }
    val
}
