use aoc2023::*;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read_input_to_string(13);
    let blocks = input.split("\n\n").collect::<Vec<_>>();

    let mut fields = Vec::new();
    for block in blocks {
        let field = block
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        fields.push(field);
    }

    let mut sum = 0;
    for field in fields {
        if let Some(row) = find_reflection(&field) {
            sum += 100 * row;
        } else {
            sum += find_reflection(&transpose(&field)).unwrap();
        }
    }

    println!("{sum}");
}

fn part2() {
    let input = read_input_to_string(13);
    let blocks = input.split("\n\n").collect::<Vec<_>>();

    let mut sum = 0;
    for block in blocks {
        let field = block
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let score = alter_field_and_get_score(field);
        sum += score;
    }

    println!("{sum}");
}

fn find_reflection(field: &[Vec<char>]) -> Option<usize> {
    (1..field.len()).find(|&row| reflected_at_row(field, row))
}

fn find_reflections(field: &[Vec<char>]) -> Vec<usize> {
    (1..field.len())
        .filter(|&row| reflected_at_row(field, row))
        .collect::<Vec<_>>()
}

fn reflected_at_row(field: &[Vec<char>], row: usize) -> bool {
    let mut i = 0;
    while row + i < field.len() && row - i > 0 {
        if field[row + i] != field[row - 1 - i] {
            return false;
        }
        i += 1;
    }
    true
}

fn alter_field_and_get_score(mut field: Vec<Vec<char>>) -> usize {
    let old_horizontal_reflection = find_reflection(&field);
    let old_vertical_reflection = find_reflection(&transpose(&field));

    for i in 0..field.len() {
        for j in 0..field[i].len() {
            toggle(&mut field, i, j);

            for r in find_reflections(&field) {
                if let Some(h) = old_horizontal_reflection {
                    if r != h {
                        return 100 * r;
                    }
                } else {
                    return 100 * r;
                }
            }
            for c in find_reflections(&transpose(&field)) {
                if let Some(v) = old_vertical_reflection {
                    if c != v {
                        return c;
                    }
                } else {
                    return c;
                }
            }

            toggle(&mut field, i, j);
        }
    }
    panic!()
}

fn toggle(field: &mut [Vec<char>], row: usize, col: usize) {
    if field[row][col] == '.' {
        field[row][col] = '#';
    } else {
        field[row][col] = '.';
    }
}
