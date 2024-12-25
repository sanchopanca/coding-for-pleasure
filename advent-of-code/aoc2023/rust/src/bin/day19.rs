use std::{
    collections::HashMap,
    ops::{Index, IndexMut, Range},
    str::FromStr,
};

use aoc_utils::*;
use regex::Regex;

fn main() {
    part1();
    part2();
}

struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Index<&str> for Part {
    type Output = u64;
    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "x" => &self.x,
            "m" => &self.m,
            "a" => &self.a,
            "s" => &self.s,
            _ => panic!("invalid index"),
        }
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
        let caps = re.captures(s).unwrap();
        Ok(Part {
            x: caps[1].parse().unwrap(),
            m: caps[2].parse().unwrap(),
            a: caps[3].parse().unwrap(),
            s: caps[4].parse().unwrap(),
        })
    }
}

impl Part {
    fn accepted(&self, workflows: &HashMap<String, Workflow>) -> bool {
        let mut next = "in".to_owned();

        while &next != "A" && &next != "R" {
            let workflow = workflows.get(&next).unwrap();
            next = workflow.execute(self);
        }

        if next == "A" {
            return true;
        }

        false
    }

    fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone)]
struct Condition {
    left: String,
    operator: String,
    right: u64,
    where_to: String,
}

impl Condition {
    fn reverse(&self) -> Self {
        let (operator, right) = if self.operator == ">" {
            ("<".to_owned(), self.right + 1)
        } else {
            (">".to_owned(), self.right - 1)
        };
        Condition {
            left: self.left.clone(),
            operator,
            right,
            where_to: self.where_to.clone(),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    conditions: Vec<Condition>,
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rest) = s.split_once('{').unwrap();
        let (rest, go_to) = rest.rsplit_once(',').unwrap();
        let go_to = go_to.strip_suffix('}').unwrap();
        let mut conditions = Vec::new();

        for cond in rest.split(',') {
            let (rest, where_to) = cond.rsplit_once(':').unwrap();
            if rest.contains('<') {
                let (left, right) = rest.split_once('<').unwrap();
                let left = left.to_owned();
                let right = right.parse::<u64>().unwrap();
                conditions.push(Condition {
                    left: left.to_owned(),
                    operator: "<".to_owned(),
                    right,
                    where_to: where_to.to_owned(),
                });
            } else {
                let (left, right) = rest.split_once('>').unwrap();
                let left = left.to_owned();
                let right = right.parse::<u64>().unwrap();
                conditions.push(Condition {
                    left: left.to_owned(),
                    operator: ">".to_owned(),
                    right,
                    where_to: where_to.to_owned(),
                });
            }
        }

        let mut last = conditions.last().unwrap().reverse();
        last.where_to = go_to.to_owned();
        conditions.push(last);

        let result = Self {
            name: name.to_owned(),
            conditions,
        };

        Ok(result)
    }
}

impl Workflow {
    fn execute(&self, part: &Part) -> String {
        for c in &self.conditions {
            let f: Box<dyn Fn(&Part) -> bool> = match c.operator.as_str() {
                ">" => Box::new(move |p: &Part| p[&c.left] > c.right),
                "<" => Box::new(move |p: &Part| p[&c.left] < c.right),
                _ => panic!("invalid operator"),
            };
            if f(part) {
                return c.where_to.to_owned();
            }
        }
        unreachable!()
    }
}

fn part1() {
    let input = read_input_to_lines(19);

    let empty_at = input.iter().position(|s| s.is_empty()).unwrap();

    let workflows = &input[..empty_at];
    let parts = &input[empty_at + 1..];

    let parts = parts
        .iter()
        .map(|s| s.parse::<Part>().unwrap())
        .collect::<Vec<_>>();

    let workflows = workflows
        .iter()
        .map(|s| s.parse::<Workflow>().unwrap())
        .map(|i| (i.name.clone(), i))
        .collect::<HashMap<_, _>>();

    let mut sum = 0;
    for part in parts {
        if part.accepted(&workflows) {
            sum += part.sum();
        }
    }
    println!("{sum}");
}

#[derive(Debug, Clone)]
struct SpaceFragment {
    x: Range<u64>,
    m: Range<u64>,
    a: Range<u64>,
    s: Range<u64>,
}

impl Index<&str> for SpaceFragment {
    type Output = Range<u64>;
    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "x" => &self.x,
            "m" => &self.m,
            "a" => &self.a,
            "s" => &self.s,
            _ => panic!("invalid index"),
        }
    }
}

impl IndexMut<&str> for SpaceFragment {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match index {
            "x" => &mut self.x,
            "m" => &mut self.m,
            "a" => &mut self.a,
            "s" => &mut self.s,
            _ => panic!("invalid index"),
        }
    }
}

impl SpaceFragment {
    fn new() -> Self {
        Self {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }
    fn size(&self) -> u64 {
        let mut prod = 1;
        for i in ["x", "m", "a", "s"] {
            if self[i].start > self[i].end {
                return 0;
            }
            prod *= self[i].end - self[i].start
        }
        prod
    }

    fn slice_by_gt(&self, index: &str, value: u64) -> (SpaceFragment, SpaceFragment) {
        let mut true_part = self.clone();
        let mut false_part = self.clone();
        true_part[index].start = value + 1;
        false_part[index].end = value + 1;
        assert!(self.size() == true_part.size() + false_part.size());
        (true_part, false_part)
    }

    fn slice_by_lt(&self, index: &str, value: u64) -> (SpaceFragment, SpaceFragment) {
        let mut true_part = self.clone();
        let mut false_part = self.clone();
        true_part[index].end = value;
        false_part[index].start = value;
        assert!(self.size() == true_part.size() + false_part.size());
        (true_part, false_part)
    }
}

fn part2() {
    let input = read_input_to_lines(19);

    let empty_at = input.iter().position(|s| s.is_empty()).unwrap();

    let workflows = &input[..empty_at];

    let workflows = workflows
        .iter()
        .map(|s| s.parse::<Workflow>().unwrap())
        .map(|i| (i.name.clone(), i))
        .collect::<HashMap<_, _>>();

    let sf = SpaceFragment::new();

    let result = solve("in", &sf, &workflows);

    println!("{result}");
}

fn solve(row: &str, sf: &SpaceFragment, workflows: &HashMap<String, Workflow>) -> u64 {
    if sf.size() == 0 || row == "R" {
        return 0;
    }

    if row == "A" {
        return sf.size();
    }

    let mut sf = sf.clone();

    let workflow = &workflows[row];
    let mut sum = 0;
    for condition in &workflow.conditions {
        let (true_part, false_part) = match condition.operator.as_str() {
            ">" => sf.slice_by_gt(&condition.left, condition.right),
            "<" => sf.slice_by_lt(&condition.left, condition.right),
            _ => panic!("invalid operator"),
        };
        sum += solve(&condition.where_to, &true_part, workflows);

        sf = false_part;
    }

    sum
}
