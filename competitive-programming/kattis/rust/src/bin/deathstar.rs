use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();

    let mut n = String::new();
    stdin.lock().read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();

    let mut matrix = Vec::new();
    for _ in 0..n {
        let mut row = String::new();
        stdin.lock().read_line(&mut row).unwrap();
        let row = row
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        matrix.push(row);
    }

    let mut result = vec![0; n];

    for i in 0..n {
        for j in 0..n {
            result[i] |= matrix[i][j];
        }
    }

    println!(
        "{}",
        result
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

#[allow(dead_code)]
fn verify(m: &[Vec<u32>], n: usize, result: &[u32]) {
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            assert_eq!(m[i][j], result[i] & result[j]);
        }
    }
}
