use std::fs;
use std::collections::HashSet;
use itertools::Itertools;


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

fn main() {
    part1();
    part2();
}


fn part1() {
    let input = fs::read_to_string("../input/09.txt").unwrap();
    let mut head = Position::new();
    let mut tail = Position::new();
    
    let mut visited_by_tail = HashSet::new();
    visited_by_tail.insert(tail);
    for line in input.lines() {
        let (dir, steps) = line.split(' ').collect_tuple().unwrap();
        let steps: u8 = steps.parse().unwrap();
        for _ in 0..steps {
            match dir {
                "L" => head.x -= 1,
                "R" => head.x += 1,
                "U" => head.y += 1,
                "D" => head.y -= 1,
                _ => panic!("Only four directions are supported, {} is not one of them", dir),
            }
            catch_up(&head, &mut tail);
            visited_by_tail.insert(tail);
        }
    }
    println!("{}", visited_by_tail.len());
}

fn part2() {
    let input = fs::read_to_string("../input/09.txt").unwrap();

    let mut rope = [Position::new(); 10];
    
    let mut visited_by_tail = HashSet::new();
    visited_by_tail.insert(rope[9]);
    for line in input.lines() {
        let (dir, steps) = line.split(' ').collect_tuple().unwrap();
        let steps: u8 = steps.parse().unwrap();
        for _ in 0..steps {
            match dir {
                "L" => rope[0].x -= 1,
                "R" => rope[0].x += 1,
                "U" => rope[0].y += 1,
                "D" => rope[0].y -= 1,
                _ => panic!("Only four directions are supported, {} is not one of them", dir),
            }
            for i in 0..(rope.len()-1) {
                let leading = &rope[i];
                let mut following = rope[i+1];
                catch_up(leading, &mut following);
                rope[i+1] = following;
            }
            visited_by_tail.insert(rope[9]);
        }
    }
    println!("{}", visited_by_tail.len());
}

fn catch_up(head: &Position, tail: &mut Position) {
    if (head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1 {
        return;
    }
    
    tail.x += (head.x - tail.x).signum();
    tail.y += (head.y - tail.y).signum();

    assert!((head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1);
}
