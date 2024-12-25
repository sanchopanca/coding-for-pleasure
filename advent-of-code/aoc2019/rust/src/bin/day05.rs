use aoc2019::intcode::VM;

fn main() {
    let mut vm = VM::load_program(5);
    vm.execute();
}
