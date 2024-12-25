use aoc_utils::*;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read_input_to_char_vectors(aoc(1));
    let mut sum = 0;
    for line in input {
        let first_digit = line.iter().find(|x| x.is_ascii_digit()).unwrap();
        let last_digit = line.iter().rev().find(|x| x.is_ascii_digit()).unwrap();

        let number = 10 * first_digit.to_digit(10).unwrap() + last_digit.to_digit(10).unwrap();
        sum += number;
    }
    println!("{sum}");
}

fn part2() {
    let input = read_input_to_lines(aoc(1));
    let re1 = regex::Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re2 = regex::Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();

    let mut sum = 0;
    for line in input {
        let reversed = line.chars().rev().collect::<String>();
        let first_match = re1.find(&line).unwrap().as_str();
        let last_match = re2.find(&reversed).unwrap().as_str();

        let number = to_number(first_match) * 10 + to_number(last_match);

        sum += number;
    }
    println!("{sum}");
}

fn to_number(s: &str) -> u32 {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "eno" => 1,
        "owt" => 2,
        "eerht" => 3,
        "ruof" => 4,
        "evif" => 5,
        "xis" => 6,
        "neves" => 7,
        "thgie" => 8,
        "enin" => 9,
        _ => s.parse::<u32>().unwrap(),
    }
}
