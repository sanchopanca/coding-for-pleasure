use aoc_utils::{aoc, input, read_input_to_string};

pub struct VM {
    mem: Vec<i64>,
    ip: usize,
}

mod opcodes {
    pub const ADD: i64 = 1;
    pub const MUL: i64 = 2;
    pub const INP: i64 = 3;
    pub const OUT: i64 = 4;
    pub const JNZ: i64 = 5;
    pub const JZ: i64 = 6;
    pub const LT: i64 = 7;
    pub const EQ: i64 = 8;
    pub const HLT: i64 = 99;

    pub enum InstructionLength {
        One,
        Two,
        Three,
        Four,
    }

    pub fn len(opcode: i64) -> InstructionLength {
        use InstructionLength::*;
        match opcode {
            ADD => Four,
            MUL => Four,
            INP => Two,
            OUT => Two,
            JNZ => Three,
            JZ => Three,
            LT => Four,
            EQ => Four,
            HLT => One,
            _ => panic!("Unknown opcode {opcode}"),
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Mode {
        Positional,
        Immediate,
    }
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
        let data = read_input_to_string(aoc(day));
        let mem: Vec<i64> = data.split(',').map(|x| x.parse().unwrap()).collect();

        Self { mem, ip: 0 }
    }

    pub fn execute(&mut self) {
        // Get the value of a parameter based on its mode.
        macro_rules! value {
            ($p:ident, $modes:ident, $i:expr) => {{
                match $modes[$i] {
                    opcodes::Mode::Positional => self.mem[addr!($p[$i])],
                    opcodes::Mode::Immediate => $p[$i],
                }
            }};
        }

        loop {
            let opcode = self.mem[self.ip] % 100;
            let mut modes_segment = self.mem[self.ip] / 100;
            let mut modes = [Mode::Positional; 3];
            for mode in &mut modes {
                *mode = if modes_segment % 10 == 0 {
                    Mode::Positional
                } else {
                    Mode::Immediate
                };
                modes_segment /= 10;
            }

            match len(opcode) {
                InstructionLength::One => {
                    match opcode {
                        HLT => break,
                        _ => unreachable!(),
                    }
                    self.ip += 1
                }
                InstructionLength::Two => {
                    let p = [self.mem[self.ip + 1]];
                    match opcode {
                        INP => {
                            let n: i64 = input("Enter a number");
                            self.mem[addr!(p[0])] = n;
                        }
                        OUT => {
                            let value = value!(p, modes, 0);
                            println!("{value}");
                        }
                        _ => unreachable!(),
                    }
                    self.ip += 2;
                }
                InstructionLength::Three => {
                    let p = self.mem[(self.ip + 1)..(self.ip + 3)].to_vec();
                    match opcode {
                        JNZ => {
                            if (value!(p, modes, 0) != 0) {
                                self.ip = addr!(value!(p, modes, 1));
                                continue;
                            }
                        }
                        JZ => {
                            if (value!(p, modes, 0) == 0) {
                                self.ip = addr!(value!(p, modes, 1));
                                continue;
                            }
                        }
                        _ => unreachable!(),
                    }
                    self.ip += 3
                }
                InstructionLength::Four => {
                    let p = self.mem[(self.ip + 1)..(self.ip + 4)].to_vec();
                    match opcode {
                        ADD => {
                            self.mem[addr!(p[2])] = value!(p, modes, 0) + value!(p, modes, 1);
                        }
                        MUL => {
                            self.mem[addr!(p[2])] = value!(p, modes, 0) * value!(p, modes, 1);
                        }
                        LT => {
                            let val = if value!(p, modes, 0) < value!(p, modes, 1) {
                                1
                            } else {
                                0
                            };
                            self.mem[addr!(p[2])] = val
                        }
                        EQ => {
                            let val = if value!(p, modes, 0) == value!(p, modes, 1) {
                                1
                            } else {
                                0
                            };
                            self.mem[addr!(p[2])] = val
                        }
                        _ => unreachable!(),
                    }
                    self.ip += 4;
                } // No code after here, because of jump instructions that call continue
            }
        }
    }

    pub fn get(&self, address: usize) -> i64 {
        self.mem[address]
    }

    pub fn set(&mut self, address: usize, value: i64) {
        self.mem[address] = value;
    }
}
