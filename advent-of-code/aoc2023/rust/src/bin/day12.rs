use aoc_utils::*;
use memoize::memoize;

fn main() {
    let input = read_input_to_lines(12);

    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in input {
        let (condition1, template1) = line.split_once(' ').unwrap();
        let condition2 = [condition1, condition1, condition1, condition1, condition1].join("?");
        let template2 = [template1, template1, template1, template1, template1].join(",");
        let template1 = parse_template(template1);
        let template2 = parse_template(&template2);

        sum1 += solve(condition1.to_owned(), template1.to_owned());
        sum2 += solve(condition2.to_owned(), template2.to_owned());
    }
    println!("{sum1}\n{sum2}");
}

fn parse_template(template: &str) -> Vec<usize> {
    template
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

#[memoize]
fn solve(condition: String, numbers: Vec<usize>) -> u64 {
    if condition.is_empty() {
        return if numbers.is_empty() { 1 } else { 0 };
    }

    if numbers.is_empty() {
        return if condition.contains('#') { 0 } else { 1 };
    }

    let mut sum = 0;
    let next = condition.chars().next().unwrap();

    if next == '.' || next == '?' {
        sum += solve(condition[1..].to_string(), numbers.clone());
    }

    let first_number = numbers[0];

    if condition.len() >= first_number {
        let condition_prefix = &condition[..first_number];
        let is_prefix_valid = condition_prefix.chars().all(|c| c == '#' || c == '?');

        if is_prefix_valid {
            if condition.len() == first_number {
                sum += solve(condition[first_number..].to_string(), numbers[1..].to_vec());
            } else if condition.chars().nth(first_number).unwrap() != '#' {
                sum += solve(
                    condition[first_number + 1..].to_string(),
                    numbers[1..].to_vec(),
                );
            }
        }
    }

    sum
}
