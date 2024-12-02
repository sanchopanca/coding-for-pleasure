use aoc2019::intcode::VM;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut vm = VM::load_program(2);
    vm.set(1, 12);
    vm.set(2, 2);
    vm.execute();
    let value = vm.get(0);
    println!("{value}");
}

fn part2() {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut vm = VM::load_program(2);
            vm.set(1, noun);
            vm.set(2, verb);
            vm.execute();
            let value = vm.get(0);
            if value == 19690720 {
                println!("{}", 100 * noun + verb);
            }
        }
    }
}
