use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<i64> = input
        .trim()
        .split(' ')
        .map(|n| n.parse().unwrap())
        .collect();
    let mut l = numbers[0];
    let mut b = numbers[1];
    let mut res = 0;
    while l <= b {
        l *= 3;
        b *= 2;
        res += 1;
    }
    println!("{res}");
}
