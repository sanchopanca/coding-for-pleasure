pub struct VM {
    mem: Vec<i64>,
    ip: usize,
}

mod opcodes {
    pub const ADD: i64 = 1;
    pub const MUL: i64 = 2;
    pub const HALT: i64 = 99;
}

macro_rules! addr {
    ($n:expr) => {{
        if $n < 0 {
            panic!("Negative address {}", $n);
        }
        $n as usize
    }};
}

use opcodes::*;

impl VM {
    pub fn load_program(day: u8) -> Self {
        let data = crate::read_input_to_string(day);
        let mem: Vec<i64> = data.split(',').map(|x| x.parse().unwrap()).collect();

        Self { mem, ip: 0 }
    }

    pub fn execute(&mut self) {
        loop {
            let [opcode, p1, p2, p3] = self.mem[self.ip..(self.ip + 4)] else {
                panic!("Cannot read 4 ints at address {}", self.ip);
            };
            match opcode {
                ADD => {
                    self.mem[addr!(p3)] = self.mem[addr!(p1)] + self.mem[addr!(p2)];
                }
                MUL => {
                    self.mem[addr!(p3)] = self.mem[addr!(p1)] * self.mem[addr!(p2)];
                }
                HALT => {
                    break;
                }
                _ => panic!("Unknown opcode {} at address {}", opcode, self.ip),
            }
            self.ip += 4;
        }
    }

    pub fn get(&self, address: usize) -> i64 {
        self.mem[address]
    }

    pub fn set(&mut self, address: usize, value: i64) {
        self.mem[address] = value;
    }
}
