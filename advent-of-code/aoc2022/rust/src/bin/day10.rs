use std::fs;

const WIDTH: usize = 40;
const HEIGHT: usize = 6;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = fs::read_to_string("../input/10.txt").unwrap();
    let mut cycle = 0;
    let mut register: i32 = 1;
    let mut res = 0;
    for line in input.lines() {
        if line == "noop" {
            cycle += 1;
            maybe_add(&mut res, cycle, register);
            continue;
        }
        let operand: i32 = line.split(' ').last().unwrap().parse().unwrap();


        cycle += 1;
        maybe_add(&mut res, cycle, register);
        cycle += 1;
        maybe_add(&mut res, cycle, register);

        register += operand
    }
    println!("{}", res);
}

fn part2() {
    let input = fs::read_to_string("../input/10.txt").unwrap();
    let mut cycle = 0;
    let mut register: i32 = 1;
    let mut screen = [['?'; WIDTH]; HEIGHT];
    for line in input.lines() {
        if line == "noop" {
            draw(&mut screen, cycle, register);
            cycle += 1;
            continue;
        }
        let operand: i32 = line.split(' ').last().unwrap().parse().unwrap();


        draw(&mut screen, cycle, register);
        cycle += 1;
        draw(&mut screen, cycle, register);
        cycle += 1;

        register += operand
    }
    show_screen(&screen);
}


fn maybe_add(res: &mut i32, cycle: usize, register: i32) {
    if (cycle as i32 - 20) % 40 == 0 {
        *res += register * cycle as i32;
    }
}

fn draw(screen: &mut [[char; WIDTH]; HEIGHT], cycle: usize, register: i32) {
    if cycle >= WIDTH * HEIGHT {
        return;
    }

    let x = cycle / WIDTH;
    let y = cycle % WIDTH;

    println!("({} {}) x={}", x, y, register);

    screen[x][y] = if shoud_draw(register, y) { '#' } else { '.' }
}

fn shoud_draw(register: i32, y: usize) -> bool {
    (register - y as i32).abs() < 2
}

fn show_screen(screen: &[[char; WIDTH]; HEIGHT]) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            print!("{}", screen[i][j]);
        }
        println!();
    }
}