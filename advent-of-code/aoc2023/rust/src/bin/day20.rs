use std::collections::{HashMap, VecDeque};

use aoc_utils::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Signal {
    High,
    Low,
}

trait Module: std::fmt::Debug {
    fn recieve_signal(&mut self, from: &str, signal: Signal);
    fn send_signal(&self) -> Option<Signal>;
}

#[derive(Debug)]
struct Broadcaster {
    signal: Signal,
}

impl Broadcaster {
    fn new() -> Self {
        Self {
            signal: Signal::Low,
        }
    }
}

impl Module for Broadcaster {
    fn recieve_signal(&mut self, _from: &str, signal: Signal) {
        self.signal = signal;
    }

    fn send_signal(&self) -> Option<Signal> {
        Some(self.signal)
    }
}

#[derive(Debug)]
struct FlipFlop {
    on: bool,
    will_send: bool,
}

impl FlipFlop {
    fn new() -> Self {
        Self {
            on: false,
            will_send: false,
        }
    }
}

impl Module for FlipFlop {
    fn recieve_signal(&mut self, _from: &str, signal: Signal) {
        if signal == Signal::High {
            self.will_send = false;
            return;
        }
        self.on = !self.on;
        self.will_send = true;
    }

    fn send_signal(&self) -> Option<Signal> {
        if !self.will_send {
            return None;
        }

        if self.on {
            Some(Signal::High)
        } else {
            Some(Signal::Low)
        }
    }
}

#[derive(Debug)]
struct Conjuction {
    inputs: HashMap<String, Signal>,
}

impl Conjuction {
    fn new(inputs: &[String]) -> Self {
        let mut map = HashMap::new();
        for input in inputs {
            map.insert(input.to_owned(), Signal::Low);
        }

        Self { inputs: map }
    }
}

impl Module for Conjuction {
    fn recieve_signal(&mut self, from: &str, signal: Signal) {
        self.inputs.insert(from.to_owned(), signal);
    }

    fn send_signal(&self) -> Option<Signal> {
        if self.inputs.values().all(|&signal| signal == Signal::High) {
            Some(Signal::Low)
        } else {
            Some(Signal::High)
        }
    }
}

#[derive(Debug)]
struct Output {}

impl Module for Output {
    fn recieve_signal(&mut self, _from: &str, _signal: Signal) {}

    fn send_signal(&self) -> Option<Signal> {
        None
    }
}

fn main() {
    let input = read_input_to_lines(aoc(20));

    let input = input
        .into_iter()
        .map(|s| {
            let (name, outputs) = s.split_once(" -> ").unwrap();
            let outputs = outputs
                .split(", ")
                .map(|s| s.to_owned())
                .collect::<Vec<_>>();
            (name.to_owned(), outputs)
        })
        .collect::<Vec<_>>();

    let active_modules = input
        .iter()
        .map(|(module, _)| {
            if module.starts_with('&') || module.starts_with('%') {
                module
                    .strip_prefix(|c| c == '&' || c == '%')
                    .unwrap()
                    .to_owned()
            } else {
                module.to_owned()
            }
        })
        .collect::<Vec<_>>();

    let mut module_to_inputs = HashMap::new();

    for (name, outputs) in &input {
        for output in outputs {
            module_to_inputs
                .entry(output.to_owned())
                .or_insert(vec![])
                .push(strip_prefix(name));
        }
    }

    let mut modules: HashMap<String, (Box<dyn Module>, Vec<String>)> = HashMap::new();

    for (name, outputs) in &input {
        let outputs = outputs.iter().map(|s| s.to_owned()).collect::<Vec<_>>();
        for output in &outputs {
            if !active_modules.contains(output) {
                modules.insert(output.to_owned(), (Box::new(Output {}), vec![]));
            }
        }

        if name.starts_with('%') {
            let name = name.strip_prefix('%').unwrap();
            modules.insert(name.to_owned(), (Box::new(FlipFlop::new()), outputs));
        } else if name.starts_with('&') {
            let name = name.strip_prefix('&').unwrap();
            modules.insert(
                name.to_owned(),
                (Box::new(Conjuction::new(&module_to_inputs[name])), outputs),
            );
        } else {
            modules.insert(name.to_owned(), (Box::new(Broadcaster::new()), outputs));
        }
    }

    let (mut h_sum, mut l_sum) = (0, 0);
    let mut cycles = vec![];
    'pressing_button: for button_press in 1i64.. {
        let mut queue = VecDeque::new();
        queue.push_back(("broadcaster".to_owned(), "button".to_owned(), Signal::Low));
        l_sum += 1;

        while let Some((to, from, signal)) = queue.pop_front() {
            if (to == "jq") && signal == Signal::High {
                cycles.push(button_press);
            }

            if cycles.len() == 4 {
                // i know from the input that there are only 4 of them
                println!("{}", lcm(&cycles));
                break 'pressing_button;
            }

            let (module, outputs) = modules.get_mut(&to).unwrap();
            module.recieve_signal(&from, signal);
            let outputs = outputs.to_owned();
            let signal = module.send_signal();
            match signal {
                Some(Signal::High) => {
                    for output in &outputs {
                        queue.push_back((output.to_owned(), to.to_owned(), Signal::High));
                        h_sum += 1;
                    }
                }
                Some(Signal::Low) => {
                    for output in &outputs {
                        queue.push_back((output.to_owned(), to.to_owned(), Signal::Low));
                        l_sum += 1;
                    }
                }
                None => (),
            }
        }
        if button_press == 1000 {
            println!("{}", l_sum * h_sum);
        }
    }
}

fn strip_prefix(s: &str) -> String {
    match s.strip_prefix(|c| c == '&' || c == '%') {
        Some(s) => s.to_owned(),
        None => s.to_owned(),
    }
}
