use std::{
    cmp,
    collections::{HashMap, HashSet},
    fs,
};

fn part1() {
    let input = fs::read_to_string("../input/09.txt").unwrap();
    let mut destinations = HashSet::new();
    let mut distances = HashMap::new();
    for line in input.lines() {
        let parts: Vec<_> = line.split(' ').collect();
        let from = parts[0].to_owned();
        let to = parts[2].to_owned();
        let distance: u32 = parts[4].parse().unwrap();

        destinations.insert(from.clone());
        destinations.insert(to.clone());

        distances.insert((from.clone(), to.clone()), distance);
        distances.insert((to, from), distance);
    }
    let mut res = u32::MAX;
    for start_destination in &destinations {
        let mut rest_of_destinations = destinations.clone();
        rest_of_destinations.remove(start_destination);
        res = cmp::min(
            res,
            f1(
                0,
                start_destination.to_owned(),
                rest_of_destinations,
                &distances,
            ),
        )
    }
    println!("{}", res);
}

fn f1(
    best_score: u32,
    current_destination: String,
    rest_of_destinations: HashSet<String>,
    distances: &HashMap<(String, String), u32>,
) -> u32 {
    if best_score == u32::MAX {
        best_score
    } else if rest_of_destinations.len() == 1 {
        let dest = rest_of_destinations.iter().next().unwrap();
        match distances.get(&(current_destination, dest.to_owned())) {
            Some(distance) => best_score + distance,
            None => u32::MAX,
        }
    } else {
        let mut res = u32::MAX;
        for dest in &rest_of_destinations {
            if let Some(distance) =
                distances.get(&(current_destination.to_owned(), dest.to_owned()))
            {
                let mut subset = rest_of_destinations.clone();
                subset.remove(dest);
                res = cmp::min(
                    res,
                    f1(best_score + distance, dest.to_owned(), subset, distances),
                )
            }
        }
        res
    }
}

fn part2() {
    let input = fs::read_to_string("../input/09.txt").unwrap();
    let mut destinations = HashSet::new();
    let mut distances = HashMap::new();
    for line in input.lines() {
        let parts: Vec<_> = line.split(' ').collect();
        let from = parts[0].to_owned();
        let to = parts[2].to_owned();
        let distance: u32 = parts[4].parse().unwrap();

        destinations.insert(from.clone());
        destinations.insert(to.clone());

        distances.insert((from.clone(), to.clone()), distance);
        distances.insert((to, from), distance);
    }
    let mut res = 0;
    for start_destination in &destinations {
        let mut rest_of_destinations = destinations.clone();
        rest_of_destinations.remove(start_destination);
        res = cmp::max(
            res,
            f2(
                0,
                start_destination.to_owned(),
                rest_of_destinations,
                &distances,
            )
            .unwrap_or(0),
        )
    }
    println!("{}", res);
}

fn f2(
    best_score: u32,
    current_destination: String,
    rest_of_destinations: HashSet<String>,
    distances: &HashMap<(String, String), u32>,
) -> Option<u32> {
    if rest_of_destinations.len() == 1 {
        let dest = rest_of_destinations.iter().next().unwrap();
        distances
            .get(&(current_destination, dest.to_owned()))
            .map(|d| best_score + d)
    } else {
        let mut res = 0;
        for dest in &rest_of_destinations {
            if let Some(distance) =
                distances.get(&(current_destination.to_owned(), dest.to_owned()))
            {
                let mut subset = rest_of_destinations.clone();
                subset.remove(dest);
                res = cmp::max(
                    res,
                    f2(best_score + distance, dest.to_owned(), subset, distances).unwrap_or(0),
                );
            }
        }
        if res == 0 {
            None
        } else {
            Some(res)
        }
    }
}

fn main() {
    part1();
    part2();
}
