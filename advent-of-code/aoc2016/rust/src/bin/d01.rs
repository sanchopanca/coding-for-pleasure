use std::collections::HashSet;
use std::str::FromStr;
use aoc2016::read_input_to_string;

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Copy, Clone)]
enum Turn {
    Left,
    Right,
}

#[derive(Copy, Clone)]
struct Command {
    turn: Turn,
    distance: u32,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn go(&mut self, direction: Direction, distance: u32) -> Vec<Position> {
        let distance = distance as i32;
        let mut v = Vec::new();
        for _ in 0..distance {
            match direction {
                Direction::North => self.y += 1,
                Direction::South => self.y -= 1,
                Direction::West => self.x -= 1,
                Direction::East => self.x += 1,
            }
            v.push(self.clone());
        }
        v
    }

    pub fn distance_from_origin(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let letter = s.chars().next()
            .ok_or("Expected a string of length at least 1".to_string())?;

        let turn = match letter {
            'L' => Turn::Left,
            'R' => Turn::Right,
            _ => return Err("Expected L or R".to_string()),
        };

        let distance_str = &s[1..];
        let distance: u32 = distance_str.parse()
            .map_err(|_| "Failed to parse distance".to_string())?;

        Ok(Command {
            turn,
            distance,
        })
    }
}


impl Direction {
    pub fn turn_left(&mut self) {
        match self {
            Direction::North => *self = Direction::West,
            Direction::South => *self = Direction::East,
            Direction::West => *self = Direction::South,
            Direction::East => *self = Direction::North,
        }
    }

    pub fn turn_right(&mut self) {
        self.turn_left();
        self.turn_left();
        self.turn_left();
    }
}

fn main() {
    let s = read_input_to_string(1);
    let commands: Vec<Command> = s.split(", ").map(|x| x.parse().unwrap()).collect();
    let mut direction = Direction::North;
    let mut postion = Position { x: 0, y: 0 };
    let mut visited = HashSet::new();
    let mut visited_twice = None;
    visited.insert(postion);
    for command in commands {
        match command.turn {
            Turn::Left => direction.turn_left(),
            Turn::Right => direction.turn_right(),
        }
        let path = postion.go(direction, command.distance);
        for point in path {
            if visited_twice.is_none() && visited.contains(&point) {
                visited_twice = Some(point.distance_from_origin());
            }
            visited.insert(point);
        }
    }

    println!("part 1: {}", postion.distance_from_origin());
    println!("part 2: {}", visited_twice.unwrap());
}