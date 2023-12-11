use aoc2023::*;

fn main() {
    let image = read_input_to_char_vectors(11);
    let empty_lines = find_empty_lines(&image);
    let empty_columns = find_empty_lines(&transpose(&image));
    let galaxies = find_galaxies(&image);

    let mut sum1 = 0;
    let mut sum2 = 0;

    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let (mut x1, mut x2) = (galaxies[i].0, galaxies[j].0);
            let (mut y1, mut y2) = (galaxies[i].1, galaxies[j].1);

            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
            }

            if y1 > y2 {
                std::mem::swap(&mut y1, &mut y2);
            }

            let distance = (x2 - x1) + (y2 - y1);
            for &r in &empty_lines {
                if (x1..x2).contains(&r) {
                    sum1 += 1;
                    sum2 += 999_999;
                }
            }
            for &c in &empty_columns {
                if (y1..y2).contains(&c) {
                    sum1 += 1;
                    sum2 += 999_999;
                }
            }
            sum1 += distance;
            sum2 += distance;
        }
    }
    println!("{sum1}");
    println!("{sum2}");
}

fn find_empty_lines(image: &[Vec<char>]) -> Vec<i64> {
    image
        .iter()
        .enumerate()
        .filter(|(_, line)| line.iter().all(|c| *c == '.'))
        .map(|(i, _)| i as i64)
        .collect()
}

fn find_galaxies(image: &[Vec<char>]) -> Vec<(i64, i64)> {
    let mut result = Vec::new();
    for (i, line) in image.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '#' {
                result.push((i as i64, j as i64));
            }
        }
    }
    result
}
