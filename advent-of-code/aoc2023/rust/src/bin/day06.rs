use aoc2023::*;

fn main() {
    part1();
    part2();
}

fn number_of_ways_to_beat_the_record(time: u64, distance: u64) -> u64 {
    let mut ways_to_beat_the_record = 0u64;
    for speed in 1..=time {
        let my_time = distance as f64 / speed as f64;
        if my_time + (speed as f64) < time as f64 {
            ways_to_beat_the_record += 1;
        }
    }
    ways_to_beat_the_record
}

fn part1() {
    let input = read_input_to_lines(6);

    let times = extract_all_numbers::<u64>(&input[0]);
    let distances = extract_all_numbers::<u64>(&input[1]);

    let pairs = times.iter().zip(distances.iter());

    let mut product = 1;
    for (time, distance) in pairs {
        product *= number_of_ways_to_beat_the_record(*time, *distance);
    }
    println!("{product}");
}

fn part2() {
    let input = read_input_to_lines(6);
    let time = skewer_numbers(&extract_all_numbers::<u64>(&input[0]));
    let distance = skewer_numbers(&extract_all_numbers::<u64>(&input[1]));

    println!("{}", number_of_ways_to_beat_the_record(time, distance));
}
