use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug)]
enum Operation {
    And { op1: Operand, op2: Operand },
    Or { op1: Operand, op2: Operand },
    LShift { op1: Operand, op2: Operand },
    RShift { op1: Operand, op2: Operand },
    Not { op: Operand },
    Identity { op: Operand },
}

#[derive(Clone, Debug)]
enum Operand {
    Literal(u16),
    Variable(String),
}

#[derive(Clone, Debug)]
enum LHS {
    Evaluated(u16),
    NonEvaluated(Operation),
}

fn eval(op: &Operand, env: &mut HashMap<String, LHS>) -> u16 {
    match op {
        Operand::Literal(value) => *value,
        Operand::Variable(var) => match env[var].clone() {
            LHS::Evaluated(number) => number,
            LHS::NonEvaluated(operation) => {
                let res = operation.execute(env);
                env.insert((*var).clone(), LHS::Evaluated(res));
                res
            }
        },
    }
}

fn not(op: &Operand, env: &mut HashMap<String, LHS>) -> u16 {
    !eval(op, env)
}

fn identity(op: &Operand, env: &mut HashMap<String, LHS>) -> u16 {
    eval(op, env)
}

fn and(op1: &Operand, op2: &Operand, env: &mut HashMap<String, LHS>) -> u16 {
    eval(op1, env) & eval(op2, env)
}

fn or(op1: &Operand, op2: &Operand, env: &mut HashMap<String, LHS>) -> u16 {
    eval(op1, env) | eval(op2, env)
}

fn lshift(op1: &Operand, op2: &Operand, env: &mut HashMap<String, LHS>) -> u16 {
    eval(op1, env) << eval(op2, env)
}

fn rshift(op1: &Operand, op2: &Operand, env: &mut HashMap<String, LHS>) -> u16 {
    eval(op1, env) >> eval(op2, env)
}

impl Operation {
    fn execute<'a>(&'a self, env: &mut HashMap<String, LHS>) -> u16 {
        match self {
            Self::And { op1, op2 } => and(op1, op2, env),
            Self::Or { op1, op2 } => or(op1, op2, env),
            Self::LShift { op1, op2 } => lshift(op1, op2, env),
            Self::RShift { op1, op2 } => rshift(op1, op2, env),
            Self::Not { op } => not(op, env),
            Self::Identity { op } => identity(op, env),
        }
    }
}

fn read_input() -> HashMap<String, LHS> {
    let input = fs::read_to_string("../input/07.txt").unwrap();
    let mut env = HashMap::new();
    for line in input.lines() {
        let v: Vec<_> = line.split(" -> ").collect();
        let id = v[1].to_owned();
        let operation = v[0].to_owned();

        let lhs = if operation.contains("NOT") {
            let operand = operation.trim_start_matches("NOT ");
            let op = match operand.parse() {
                Err(_) => Operand::Variable(operand.to_string()),
                Ok(number) => Operand::Literal(number),
            };
            LHS::NonEvaluated(Operation::Not { op })
        } else if operation.contains("A") || operation.contains("O") || operation.contains("I") {
            let v: Vec<_> = operation.split(" ").collect();
            let op1 = match v[0].parse() {
                Err(_) => Operand::Variable(v[0].to_string()),
                Ok(number) => Operand::Literal(number),
            };
            let op2 = match v[2].parse() {
                Err(_) => Operand::Variable(v[2].to_string()),
                Ok(number) => Operand::Literal(number),
            };

            match v[1] {
                "AND" => LHS::NonEvaluated(Operation::And { op1, op2 }),
                "OR" => LHS::NonEvaluated(Operation::Or { op1, op2 }),
                "LSHIFT" => LHS::NonEvaluated(Operation::LShift { op1, op2 }),
                "RSHIFT" => LHS::NonEvaluated(Operation::RShift { op1, op2 }),
                _ => unreachable!(),
            }
        } else {
            match operation.parse() {
                Err(_) => LHS::NonEvaluated(Operation::Identity {
                    op: Operand::Variable(operation.to_string()),
                }),
                Ok(number) => LHS::Evaluated(number),
            }
        };
        env.insert(id, lhs);
    }
    env
}

pub fn day07() {
    let mut env = read_input();
    let a = env["a"].clone();
    let result = match a {
        LHS::Evaluated(number) => number,
        LHS::NonEvaluated(operation) => operation.execute(&mut env),
    };

    println!("Part 1: {}", result);

    let mut env = read_input();
    env.insert("b".to_owned(), LHS::Evaluated(result));

    let a = env["a"].clone();
    let result = match a {
        LHS::Evaluated(number) => number,
        LHS::NonEvaluated(operation) => operation.execute(&mut env),
    };

    println!("Part 2: {}", result);
}
