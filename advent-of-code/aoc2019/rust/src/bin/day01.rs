fn main() {
    part1();
    part2();
}

fn part1() {
    let numbers: Vec<u32> = aoc2019::read_input_to_lines(1)
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();

    let sum: u32 = numbers.into_iter().map(|n| n / 3 - 2).sum();
    println!("{sum}");
}

fn part2() {
    let numbers: Vec<i32> = aoc2019::read_input_to_lines(1)
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();

    let sum: i32 = numbers.into_iter().map(calculate_fuel_req).sum();
    println!("{sum}");
}

fn calculate_fuel_req(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    let req = 0.max(n / 3 - 2);
    req + calculate_fuel_req(req)
}
