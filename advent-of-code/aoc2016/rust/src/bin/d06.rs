use aoc2016::{read_input_to_char_vectors, transpose};
use counter::Counter;

fn main() {
    let rows = read_input_to_char_vectors(6);
    let columns = transpose(&rows);
    let mut s1 = String::new();
    let mut s2 = String::new();
    for column in columns {
        let counter = column.iter().collect::<Counter<_>>();
        let c1 = counter.k_most_common_ordered(1).first().unwrap().0;
        let c2 = counter.most_common_ordered().last().unwrap().0;
        s1.push(*c1);
        s2.push(*c2);
    }
    println!("part 1: {}", s1);
    println!("part 2: {}", s2);
}
