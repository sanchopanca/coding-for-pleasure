use aoc_utils::*;

fn main() {
    part1();
}

fn part1() {
    let input = read_input_to_lines(aoc(24));

    let mut hailstones = Vec::new();

    for line in input {
        let (position, velocity) = line.split_once(" @ ").unwrap();
        let position = position
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i64>>();
        let velocity = velocity
            .split(", ")
            // .inspect(|x| println!("{x}"))
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i64>>();

        hailstones.push((
            (position[0], position[1], position[2]),
            (velocity[0], velocity[1], velocity[2]),
        ));
    }

    let (min, max) = (200000000000000f64, 400000000000000f64);
    // let (min, max) = (7f64, 27f64);
    let mut sum = 0;
    for (i, &((x1, y1, _), (dx1, dy1, _))) in hailstones.iter().enumerate() {
        for &((x2, y2, _), (dx2, dy2, _)) in hailstones.iter().skip(i + 1) {
            let k1 = dy1 as f64 / dx1 as f64;
            let m1 = y1 as f64 - k1 * x1 as f64;

            let k2 = dy2 as f64 / dx2 as f64;
            let m2 = y2 as f64 - k2 * x2 as f64;

            let x = (m2 - m1) / (k1 - k2);
            let y = k1 * x + m1;

            if min <= x
                && x <= max
                && min <= y
                && y <= max
                && in_the_future_2d((x1, y1), (dx1, dy1), (x, y))
                && in_the_future_2d((x2, y2), (dx2, dy2), (x, y))
            {
                sum += 1;
            }
        }
    }

    println!("{sum}");
}

fn in_the_future_2d((x0, y0): (i64, i64), (dx, dy): (i64, i64), (x, y): (f64, f64)) -> bool {
    (x - x0 as f64) * (dx as f64) > 0. && (y - y0 as f64) * (dy as f64) > 0.
}
