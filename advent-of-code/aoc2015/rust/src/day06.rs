use std::cmp;
use std::fs;

struct Grid {
    lights: Vec<Vec<bool>>,
}

struct CoolerGrid {
    lights: Vec<Vec<i16>>,
}

#[derive(Copy, Clone)]
struct Range {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

#[derive(Copy, Clone)]
enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    operation: Operation,
    range: Range,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            lights: vec![vec![false; 1000]; 1000],
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        for i in instruction.range.x1..instruction.range.x2 + 1 {
            for j in instruction.range.y1..instruction.range.y2 + 1 {
                match instruction.operation {
                    Operation::TurnOn => self.lights[i][j] = true,
                    Operation::TurnOff => self.lights[i][j] = false,
                    Operation::Toggle => self.lights[i][j] = !self.lights[i][j],
                }
            }
        }
    }

    pub fn count(&self) -> i32 {
        let mut c = 0;
        for row in &self.lights {
            for light in row {
                if *light {
                    c += 1;
                }
            }
        }
        c
    }
}

impl CoolerGrid {
    pub fn new() -> Self {
        Self {
            lights: vec![vec![0i16; 1000]; 1000],
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        for i in instruction.range.x1..instruction.range.x2 + 1 {
            for j in instruction.range.y1..instruction.range.y2 + 1 {
                match instruction.operation {
                    Operation::TurnOn => self.lights[i][j] += 1,
                    Operation::TurnOff => {
                        self.lights[i][j] -= 1;
                        self.lights[i][j] = cmp::max(0, self.lights[i][j]);
                    }
                    Operation::Toggle => self.lights[i][j] += 2,
                }
            }
        }
    }

    pub fn count(&self) -> i32 {
        let mut c: i32 = 0;
        for row in &self.lights {
            for light in row {
                c += *light as i32;
            }
        }
        c
    }
}

impl Range {
    pub fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Self { x1, y1, x2, y2 }
    }
}

pub fn day06() {
    let mut grid = Grid::new();
    let mut cooler_grid = CoolerGrid::new();
    let instructions = fs::read_to_string("../input/06.txt").unwrap();
    for line in instructions.lines() {
        let mut words: Vec<_> = line.split(" ").collect();
        let operation = if words.len() == 4 {
            Operation::Toggle
        } else {
            if words[1] == "on" {
                Operation::TurnOn
            } else {
                Operation::TurnOff
            }
        };
        let until: Vec<_> = words.pop().unwrap().split(",").collect();
        words.pop();
        let from: Vec<_> = words.pop().unwrap().split(",").collect();
        let range = Range::new(
            from[0].parse::<usize>().unwrap(),
            from[1].parse::<usize>().unwrap(),
            until[0].parse::<usize>().unwrap(),
            until[1].parse::<usize>().unwrap(),
        );
        grid.execute(Instruction { operation, range });
        cooler_grid.execute(Instruction { operation, range });
    }
    println!("{}", grid.count());
    println!("{}", cooler_grid.count());
}
