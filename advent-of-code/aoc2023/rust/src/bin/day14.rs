use aoc_utils::*;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut platform = read_input_to_char_vectors(aoc(14));

    tilt_north(&mut platform);

    println!("{}", calculate_load(&platform));
}

fn part2() {
    let mut platform = read_input_to_char_vectors(aoc(14));
    let mut old_platforms = vec![platform.clone()];
    let mut i = 1;
    let (cycle_start, cycle_len) = 'search: loop {
        tilt_north(&mut platform);
        tilt_west(&mut platform);
        tilt_south(&mut platform);
        tilt_east(&mut platform);

        if let Some((j, _)) = old_platforms
            .iter()
            .enumerate()
            .find(|(_, p)| p == &&platform)
        {
            let cycle_start = j;
            let cycle_len = i - cycle_start;
            break 'search (cycle_start, cycle_len);
        }
        old_platforms.push(platform.clone());
        i += 1;
    };

    let index = (1_000_000_000 - cycle_start) % cycle_len + cycle_start;

    println!("{}", calculate_load(&old_platforms[index]));
}

fn calculate_load(platform: &[Vec<char>]) -> usize {
    let mut sum = 0;
    for (i, row) in platform.iter().enumerate() {
        sum += row.iter().filter(|&&c| c == 'O').count() * (platform.len() - i);
    }

    sum
}

fn tilt_north(platform: &mut Vec<Vec<char>>) {
    for i in 0..platform.len() {
        for j in 0..platform[i].len() {
            if platform[i][j] == 'O' {
                for row in (0..i).rev() {
                    if platform[row][j] == '.' {
                        platform[row][j] = 'O';
                        platform[row + 1][j] = '.';
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_south(platform: &mut Vec<Vec<char>>) {
    for i in (0..platform.len()).rev() {
        for j in 0..platform[i].len() {
            if platform[i][j] == 'O' {
                for row in (i + 1)..platform.len() {
                    if platform[row][j] == '.' {
                        platform[row][j] = 'O';
                        platform[row - 1][j] = '.';
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_west(platform: &mut Vec<Vec<char>>) {
    for j in 0..platform[0].len() {
        for i in 0..platform.len() {
            if platform[i][j] == 'O' {
                for col in (0..j).rev() {
                    if platform[i][col] == '.' {
                        platform[i][col] = 'O';
                        platform[i][col + 1] = '.';
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_east(platform: &mut Vec<Vec<char>>) {
    for j in (0..platform[0].len()).rev() {
        for i in 0..platform.len() {
            if platform[i][j] == 'O' {
                for col in (j + 1)..platform[i].len() {
                    if platform[i][col] == '.' {
                        platform[i][col] = 'O';
                        platform[i][col - 1] = '.';
                    } else {
                        break;
                    }
                }
            }
        }
    }
}
