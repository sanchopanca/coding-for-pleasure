mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
use std::env;

fn main() {
    let day: i32 = env::args().nth(1).unwrap().parse().unwrap();
    if day == 1 {
        day01::day01();
    } else if day == 2 {
        day02::day02();
    } else if day == 3 {
        day03::day03();
    } else if day == 4 {
        day04::day04();
    } else if day == 5 {
        day05::day05();
    }
}
