use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();

    let mut n = String::new();
    stdin.lock().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u32>().unwrap();

    let mut trees = String::new();
    stdin.lock().read_line(&mut trees).unwrap();
    let mut trees = trees
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    trees.sort_by(|a, b| b.cmp(a));

    for i in 0..n {
        trees[i as usize] += i + 2;
    }

    let max = trees.iter().max().unwrap();
    println!("{}", max);
}
