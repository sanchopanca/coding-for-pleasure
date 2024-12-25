use aoc_utils::*;

#[derive(Debug, Clone, Copy)]
struct Number {
    row: usize,
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, Copy)]
struct Gear {
    row: usize,
    col: usize,
}

impl Gear {
    fn is_adjacent(&self, number: Number) -> bool {
        // same row
        if self.row == number.row {
            if number.start != 0 && self.col == number.start - 1 {
                return true;
            }
            if number.end + 1 == self.col {
                return true;
            }
        }

        let start = number.start.max(1) - 1;
        let end = number.end + 1;
        // row above
        if number.row != 0 && self.row == number.row - 1 && (start..=end).contains(&self.col) {
            return true;
        }

        // row below
        if self.row == number.row + 1 && (start..=end).contains(&self.col) {
            return true;
        }
        false
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let schematic = read_input_to_char_vectors(3);
    let numbers = find_numbers(&schematic);

    let mut sum = 0;
    for number in numbers {
        if is_adjacent_to_something(&schematic, number) {
            sum += get_value(&schematic, number);
        }
    }
    println!("{sum}");
}

fn part2() {
    let schematic = read_input_to_char_vectors(3);
    let numbers = find_numbers(&schematic);
    let gears = find_gears(&schematic);
    let mut sum = 0;
    for gear in gears {
        let mut gear_numbers = Vec::new();
        for number in numbers.iter() {
            if gear.is_adjacent(*number) {
                gear_numbers.push(*number);
            }
        }
        if gear_numbers.len() == 2 {
            sum += get_value(&schematic, gear_numbers[0]) * get_value(&schematic, gear_numbers[1]);
            continue;
        }
    }
    println!("{sum}");
}

fn find_numbers(schematic: &[Vec<char>]) -> Vec<Number> {
    let mut numbers = Vec::new();

    for (row, line) in schematic.iter().enumerate() {
        let mut consuming = false;
        let mut start: usize = 0;
        for (col, c) in line.iter().enumerate() {
            if c.is_ascii_digit() && !consuming {
                consuming = true;
                start = col;
            } else if !c.is_ascii_digit() && consuming {
                consuming = false;
                numbers.push(Number {
                    row,
                    start,
                    end: col - 1,
                });
            }
        }
        if consuming {
            numbers.push(Number {
                row,
                start,
                end: line.len() - 1,
            });
        }
    }
    numbers
}

fn get_value(schematic: &[Vec<char>], number: Number) -> u32 {
    let value = schematic[number.row][number.start..=number.end]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap();
    value
}

fn is_adjacent_to_something(schematic: &[Vec<char>], number: Number) -> bool {
    let row_len = schematic[number.row].len();
    // same row
    if number.start != 0 && schematic[number.row][number.start - 1] != '.' {
        return true;
    }
    if number.end + 1 < schematic[number.row].len() && schematic[number.row][number.end + 1] != '.'
    {
        return true;
    }

    let start = number.start.max(1) - 1;
    let end = (row_len - 1).min(number.end + 1);
    // row above
    if number.row > 0 {
        let has_something = schematic[number.row - 1][start..=end]
            .iter()
            .any(|c| *c != '.');
        if has_something {
            return true;
        }
    }

    // row below
    if number.row + 1 < schematic.len() {
        let has_something = schematic[number.row + 1][start..=end]
            .iter()
            .any(|c| *c != '.');
        if has_something {
            return true;
        }
    }
    false
}

fn find_gears(schematic: &[Vec<char>]) -> Vec<Gear> {
    let mut gears = Vec::new();
    for (row, line) in schematic.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == '*' {
                gears.push(Gear { row, col });
            }
        }
    }
    gears
}
