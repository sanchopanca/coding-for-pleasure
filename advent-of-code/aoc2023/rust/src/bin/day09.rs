use aoc_utils::*;

fn main() {
    let input = read_input_to_lines(99);
    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in input {
        let numbers = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let (first, last) = solve(numbers);
        sum1 += last;
        sum2 += first;
    }
    println!("{sum1}");
    println!("{sum2}");
}

fn solve(numbers: Vec<i32>) -> (i32, i32) {
    let mut d = vec![numbers];
    let next = find_differences(d.last().unwrap());
    d.push(next);
    while !all_zeros(d.last().unwrap()) {
        let next = find_differences(d.last().unwrap());
        d.push(next);
    }

    d.last_mut().unwrap().push(0);
    d.last_mut().unwrap().push(0);

    for i in (0..d.len() - 1).rev() {
        let last = *d[i].last().unwrap();
        let previous_last = *d[i + 1].last().unwrap();
        let v = &mut d[i];
        v.push(last + previous_last);

        let first = *d[i].first().unwrap();
        let previous_first = *d[i + 1].first().unwrap();

        d[i].insert(0, first - previous_first);
    }
    (
        d[0].first().unwrap().to_owned(),
        d[0].last().unwrap().to_owned(),
    )
}

fn find_differences(numbers: &[i32]) -> Vec<i32> {
    let mut differences = Vec::new();
    for i in 1..numbers.len() {
        differences.push(numbers[i] - numbers[i - 1]);
    }
    differences
}

fn all_zeros(numbers: &[i32]) -> bool {
    numbers.iter().all(|&n| n == 0)
}
