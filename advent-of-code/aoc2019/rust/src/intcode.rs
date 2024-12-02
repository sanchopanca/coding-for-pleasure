pub struct VM {
    mem: Vec<usize>,
    ip: usize,
}

mod opcodes {
    pub const ADD: usize = 1;
    pub const MUL: usize = 2;
    pub const HALT: usize = 99;
}

use opcodes::*;

impl VM {
    pub fn load_program(day: u8) -> Self {
        let data = crate::read_input_to_string(day);
        let mem: Vec<usize> = data.split(',').map(|x| x.parse().unwrap()).collect();

        Self { mem, ip: 0 }
    }

    pub fn execute(&mut self) {
        loop {
            let [opcode, p1, p2, p3] = self.mem[self.ip..(self.ip + 4)] else {
                panic!("Cannot read 4 ints at address {}", self.ip);
            };
            match opcode {
                ADD => {
                    self.mem[p3] = self.mem[p1] + self.mem[p2];
                }
                MUL => {
                    self.mem[p3] = self.mem[p1] * self.mem[p2];
                }
                HALT => {
                    break;
                }
                _ => panic!("Unknown opcode {} at address {}", opcode, self.ip),
            }
            self.ip += 4;
        }
    }

    pub fn get(&self, address: usize) -> usize {
        self.mem[address]
    }

    pub fn set(&mut self, address: usize, value: usize) {
        self.mem[address] = value;
    }
}
