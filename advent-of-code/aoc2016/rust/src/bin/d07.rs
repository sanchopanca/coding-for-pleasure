use aoc2016::read_input_to_lines;
use itertools::Itertools;

fn main() {
    let lines = read_input_to_lines(7);
    let mut ssl = 0;
    let mut tls = 0;
    for line in lines {
        let fragments = line.split(|c| c == '[' || c == ']').collect::<Vec<_>>();
        let mut supernet_fragments = vec![];
        let mut hypernet_fragments = vec![];
        for (i, fragment) in fragments.iter().enumerate() {
            if i % 2 == 0 {
                supernet_fragments.push(*fragment);
            } else {
                hypernet_fragments.push(*fragment);
            }
        }
        if supernet_fragments.iter().any(|x| has_abba(x))
            && !hypernet_fragments.iter().any(|x| has_abba(x))
        {
            ssl += 1;
        }

        let abas = supernet_fragments
            .iter()
            .flat_map(|x| find_aba(x))
            .collect::<Vec<_>>();

        'tls_search: for aba in abas {
            let bab = reverse_aba(&aba);
            for hypernet_fragment in &hypernet_fragments {
                if hypernet_fragment.contains(&bab) {
                    tls += 1;
                    break 'tls_search;
                }
            }
        }
    }
    println!("part 1: {}", ssl);
    println!("part 2: {}", tls); // 233 to high
}

fn has_abba(s: &str) -> bool {
    for (a, b, c, d) in s.chars().tuple_windows() {
        if a == d && b == c && a != b {
            return true;
        }
    }
    false
}

fn find_aba(s: &str) -> Vec<String> {
    let mut result = vec![];
    for (a, b, c) in s.chars().tuple_windows() {
        if a == c && a != b {
            result.push(String::from_iter([a, b, c]));
        }
    }
    result
}

fn reverse_aba(s: &str) -> String {
    let (a, b) = s.chars().tuple_windows().next().unwrap();
    String::from_iter([b, a, b])
}
