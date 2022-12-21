use std::collections::HashMap;
use std::fs;

use itertools::Itertools;

#[derive(Clone)]
enum RHS {
    Literal(i64),
    Op(Operation),
}

#[derive(Clone)]
struct Operation {
    op1: String,
    op2: String,
    operator: String,
}

impl Operation {
    fn evaluate(&self, env: &mut HashMap<String, RHS>) -> i64 {
        let op1 = env[&self.op1].clone();
        let op2 = env[&self.op2].clone();
        let op1 = match op1 {
            RHS::Literal(n) => n,
            RHS::Op(op) => op.evaluate(env),
        };
        let op2 = match op2 {
            RHS::Literal(n) => n,
            RHS::Op(op) => op.evaluate(env),
        };
        match self.operator.as_str() {
            "+" => op1 + op2,
            "-" => op1 - op2,
            "*" => op1 * op2,
            "/" => op1 / op2,
            _ => panic!("unsupported operator"),
        }
    }
}

fn main() {
    let input = fs::read_to_string("../input/21.txt").unwrap();
    let mut env = HashMap::new();
    for line in input.lines() {
        let (lhs, rhs) = line.split(": ").collect_tuple().unwrap();
        let rhs = if rhs.contains(' ') {
            let (op1, operator, op2) = rhs.split(' ').collect_tuple().unwrap();
            RHS::Op(Operation {
                op1: op1.to_owned(),
                op2: op2.to_owned(),
                operator: operator.to_owned(),
            })
        } else {
            RHS::Literal(rhs.parse().unwrap())
        };
        env.insert(lhs.to_owned(), rhs);
    }

    let root = env["root"].clone();
    let result = match root {
        RHS::Literal(n) => n,
        RHS::Op(op) => op.evaluate(&mut env),
    };

    println!("{}", result);
}
