use aoc2016::read_input_to_lines;
use std::cmp::{max, min};

struct LeftFinger {
    pub x: i32,
    pub y: i32,
}

impl LeftFinger {
    fn new() -> Self {
        LeftFinger {x: 1, y: 1}
    }

    fn move_finger(&mut self, c: char) {
        match c {
            'U' => self.y = max(self.y - 1, 0),
            'D' => self.y = min(self.y + 1, 2),
            'L' => self.x = max(self.x - 1, 0),
            'R' => self.x = min(self.x + 1, 2),
            _ => panic!("wrong input: {}", c),
        }
    }

    fn press(&self) -> i32 {
        self.y * 3 + self.x + 1
    }
}

struct RightFinger {
    pub x: i32,
    pub y: i32,
}

impl RightFinger {
    fn new() -> Self {
        RightFinger {x: -2, y: 0}
    }

    fn move_finger(&mut self, c: char) {
        match c {
            'U' => if Self::within_keyboard(self.x, self.y - 1) { self.y -= 1 },
            'D' => if Self::within_keyboard(self.x, self.y + 1) { self.y += 1 },
            'L' => if Self::within_keyboard(self.x - 1, self.y) { self.x -= 1 },
            'R' => if Self::within_keyboard(self.x + 1, self.y) { self.x += 1 },
            _ => panic!("wrong input: {}", c),
        }
    }

    fn press(&self) -> char {
        match (self.x, self.y) {
            (0, -2) => '1',
            (-1, -1) => '2',
            (0, -1) => '3',
            (1, -1) => '4',
            (-2, 0) => '5',
            (-1, 0) => '6',
            (0, 0) => '7',
            (1, 0) => '8',
            (2, 0) => '9',
            (-1, 1) => 'A',
            (0, 1) => 'B',
            (1, 1) => 'C',
            (0, 2) => 'D',
            _ => panic!("wrong coords ({} {})", self.x, self.y),
        }
    }

    fn within_keyboard(x: i32, y: i32) -> bool {
        x.abs() + y.abs() <= 2
    }
}


fn main() {
    let commands = read_input_to_lines(2);
    let mut pin1 = String::new();
    let mut pin2 = String::new();
    for command in commands {
        let mut left_finger = LeftFinger::new();
        let mut right_finger = RightFinger::new();
        for c in command.chars() {
            left_finger.move_finger(c);
            right_finger.move_finger(c);

        }
        pin1.push_str(&format!("{}", left_finger.press()));
        pin2.push(right_finger.press());
    }
    println!("part 1: {}", pin1);
    println!("part 2: {}", pin2);
}