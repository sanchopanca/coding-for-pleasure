use std::fs;

fn main() {
    let input = fs::read_to_string("../input/25.txt").unwrap();
    let s: i64 = input.lines().map(snafu2i).sum();
    println!("{}", i2snafu(s));
}

fn snafu2i(s: &str) -> i64 {
    let mut place = 0;
    const BASE: i64 = 5;
    let mut decimal = 0;
    for digit in s.chars().rev() {
        match digit {
            '0'..='2' => decimal += BASE.pow(place) * digit.to_digit(10).unwrap() as i64,
            '-' => decimal -= BASE.pow(place),
            '=' => decimal -= BASE.pow(place) * 2,
            _ => panic!("wrong digit"),
        }
        place += 1;
    }
    decimal
}

fn i2snafu(mut n: i64) -> String {
    let mut snafu = "".to_owned();
    while n != 0 {
        let r = n % 5;
        match r {
            0..=2 => snafu.push_str(r.to_string().as_str()),
            3 => snafu.push('='),
            4 => snafu.push('-'),
            _ => panic!("unreachable"),
        }
        n = (n + 2) / 5;
    }
    snafu.chars().rev().collect()
}
