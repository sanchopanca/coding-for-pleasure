use aoc_utils::*;

struct Map {
    source: u64,
    dest: u64,
    length: u64,
}

impl Map {
    fn contains(&self, x: u64) -> bool {
        x >= self.source && x < self.source + self.length
    }

    fn convert(&self, x: u64) -> u64 {
        if !self.contains(x) {
            panic!("out of range");
        }
        let offset = x - self.source;
        self.dest + offset
    }
}

struct MapSet {
    maps: Vec<Map>,
}

impl MapSet {
    fn convert(&self, x: u64) -> u64 {
        for map in &self.maps {
            if map.contains(x) {
                return map.convert(x);
            }
        }
        x
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let (seeds, mapsets) = parse_input();

    let min = seeds
        .iter()
        .map(|x| {
            let mut x = *x;
            for mapset in &mapsets {
                x = mapset.convert(x);
            }
            x
        })
        .min()
        .unwrap();

    println!("{min}");
}

fn part2() {
    let (seeds, mapsets) = parse_input();

    let seed_ranges: Vec<(u64, u64)> = seeds.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();

    let mut min = u64::MAX;
    let l = seed_ranges.len();
    for (i, (start, len)) in seed_ranges.into_iter().enumerate() {
        eprintln!("Progress: {}/{}", i + 1, l);
        let new_min = (start..start + len)
            .map(|x| {
                let mut x = x;
                for mapset in &mapsets {
                    x = mapset.convert(x);
                }
                x
            })
            .min()
            .unwrap();
        min = min.min(new_min);
    }

    println!("{min}");
}

fn parse_input() -> (Vec<u64>, Vec<MapSet>) {
    let input = read_input_to_string(5);

    let (seeds, rest) = input.split_once("\n\n").unwrap();
    let (_, seeds) = seeds.split_once(": ").unwrap();
    let seeds = seeds
        .split_ascii_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut mapsets = Vec::new();
    for split in rest.split("\n\n") {
        let (_, numbers) = split.split_once(":\n").unwrap();
        let mut maps = Vec::new();
        for triplet in numbers.trim().split('\n') {
            let triplet = triplet.split_whitespace().collect::<Vec<_>>();
            let map = Map {
                source: triplet[1].parse::<u64>().unwrap(),
                dest: triplet[0].parse::<u64>().unwrap(),
                length: triplet[2].parse::<u64>().unwrap(),
            };
            maps.push(map);
        }
        mapsets.push(MapSet { maps });
    }
    (seeds, mapsets)
}
