use std::collections::{HashMap, VecDeque};
use std::fs;

use counter::Counter;
use itertools::Itertools;

struct Monkey {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test_value: u64,
    yes_monkey: usize,
    no_monkey: usize,
}

fn main() {
    part1();
    part2();
}

fn part2() {
    let input = fs::read_to_string("../input/11.txt").unwrap();
    let mut monkeys = Vec::new();
    for description in input.split("\n\n") {
        let lines: Vec<_> = description.lines().collect();
        let items: VecDeque<u64> = lines[1]
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();
        let (operator, operand) = lines[2]
            .split("= old ")
            .last()
            .unwrap()
            .split(' ')
            .collect_tuple()
            .unwrap();
        let operation: Box<dyn Fn(u64) -> u64> = if operand == "old" {
            Box::new(|x| x * x)
        } else {
            let operand: u64 = operand.parse().unwrap();
            if operator == "*" {
                Box::new(move |x| x * operand)
            } else {
                Box::new(move |x| x + operand)
            }
        };

        let test_value: u64 = lines[3].split(' ').last().unwrap().parse().unwrap();
        let yes_monkey: usize = lines[4].split(' ').last().unwrap().parse().unwrap();
        let no_monkey: usize = lines[5].split(' ').last().unwrap().parse().unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            test_value,
            yes_monkey,
            no_monkey,
        });
    }

    let modulo_trick: u64 = monkeys.iter().map(|m| m.test_value).product();

    let mut counter: Counter<usize> = Counter::new();
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            // for (i, monkey) in monkeys.iter_mut().enumerate() {
            let monkey = &mut monkeys[i];
            counter[&i] += monkey.items.len();
            let mut monkeys_tmp_because_borrow_checker_got_me = HashMap::new();
            while !monkey.items.is_empty() {
                let item = monkey.items.pop_front().unwrap();
                let stress = (monkey.operation)(item) % modulo_trick;
                if stress % monkey.test_value == 0 {
                    if !monkeys_tmp_because_borrow_checker_got_me.contains_key(&monkey.yes_monkey) {
                        monkeys_tmp_because_borrow_checker_got_me
                            .insert(monkey.yes_monkey, vec![stress]);
                    } else {
                        monkeys_tmp_because_borrow_checker_got_me
                            .get_mut(&monkey.yes_monkey)
                            .unwrap()
                            .push(stress);
                    }
                } else {
                    if !monkeys_tmp_because_borrow_checker_got_me.contains_key(&monkey.no_monkey) {
                        monkeys_tmp_because_borrow_checker_got_me
                            .insert(monkey.no_monkey, vec![stress]);
                    } else {
                        monkeys_tmp_because_borrow_checker_got_me
                            .get_mut(&monkey.no_monkey)
                            .unwrap()
                            .push(stress);
                    }
                }
            }
            for (idx, items) in monkeys_tmp_because_borrow_checker_got_me {
                monkeys[idx].items.extend(items);
            }
        }
    }
    let busiest_monkeys = counter.k_most_common_ordered(2);
    println!("{}", busiest_monkeys[0].1 * busiest_monkeys[1].1)
}

fn part1() {
    let input = fs::read_to_string("../input/11.txt").unwrap();
    let mut monkeys = Vec::new();
    for description in input.split("\n\n") {
        let lines: Vec<_> = description.lines().collect();
        let items: VecDeque<u64> = lines[1]
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();
        let (operator, operand) = lines[2]
            .split("= old ")
            .last()
            .unwrap()
            .split(' ')
            .collect_tuple()
            .unwrap();
        let operation: Box<dyn Fn(u64) -> u64> = if operand == "old" {
            Box::new(|x| x * x)
        } else {
            let operand: u64 = operand.parse().unwrap();
            if operator == "*" {
                Box::new(move |x| x * operand)
            } else {
                Box::new(move |x| x + operand)
            }
        };

        let test_value: u64 = lines[3].split(' ').last().unwrap().parse().unwrap();
        let yes_monkey: usize = lines[4].split(' ').last().unwrap().parse().unwrap();
        let no_monkey: usize = lines[5].split(' ').last().unwrap().parse().unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            test_value,
            yes_monkey,
            no_monkey,
        });
    }

    let mut counter: Counter<usize> = Counter::new();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            counter[&i] += monkey.items.len();
            let mut monkeys_tmp_because_borrow_checker_got_me = HashMap::new();
            while !monkey.items.is_empty() {
                let item = monkey.items.pop_front().unwrap();
                let mut stress = (monkey.operation)(item);
                stress /= 3;
                if stress % monkey.test_value == 0 {
                    if !monkeys_tmp_because_borrow_checker_got_me.contains_key(&monkey.yes_monkey) {
                        monkeys_tmp_because_borrow_checker_got_me
                            .insert(monkey.yes_monkey, vec![stress]);
                    } else {
                        monkeys_tmp_because_borrow_checker_got_me
                            .get_mut(&monkey.yes_monkey)
                            .unwrap()
                            .push(stress);
                    }
                } else {
                    if !monkeys_tmp_because_borrow_checker_got_me.contains_key(&monkey.no_monkey) {
                        monkeys_tmp_because_borrow_checker_got_me
                            .insert(monkey.no_monkey, vec![stress]);
                    } else {
                        monkeys_tmp_because_borrow_checker_got_me
                            .get_mut(&monkey.no_monkey)
                            .unwrap()
                            .push(stress);
                    }
                }
            }
            for (idx, items) in monkeys_tmp_because_borrow_checker_got_me {
                monkeys[idx].items.extend(items);
            }
        }
    }
    let busiest_monkeys = counter.k_most_common_ordered(2);
    println!("{}", busiest_monkeys[0].1 * busiest_monkeys[1].1)
}
