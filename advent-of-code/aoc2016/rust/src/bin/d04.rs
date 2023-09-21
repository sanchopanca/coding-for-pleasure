use counter::Counter;
fn main() {
    let lines = aoc2016::read_input_to_lines(4);
    let re = regex::Regex::new(r"^([a-z-]+)-(\d+)\[([a-z]+)]$").unwrap();
    let mut sum = 0;
    for line in lines {
        let cap = re.captures(&line).unwrap();
        let name = cap.get(1).unwrap().as_str().to_owned();
        let sector_id: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
        let checksum = cap.get(3).unwrap().as_str().to_owned();
        if valid_checksum(&name, &checksum) {
            sum += sector_id;
            let name = decipher(name, sector_id);
            if name.contains("north") {
                println!("part 2: {}", sector_id);
            }
        }
    }
    println!("part 1: {sum}");
}

fn valid_checksum(room_name: &str, checksum: &str) -> bool {
    let counter = room_name
        .replace('-', "")
        .chars()
        .collect::<Counter<_>>();
    let correct_checksum = counter
        .k_most_common_ordered(5)
        .iter()
        .map(|x| x.0)
        .collect::<String>();
    checksum == correct_checksum
}

fn decipher(mut s: String, n: i32) -> String {
    for _ in 0..(n%26) {
        s = s.chars()
            .map(|c| {
                match c {
                    'a'..='y' => (1 + c as u8) as char,
                    'z' => 'a',
                    '-' => ' ',
                    ' ' => ' ',
                    _ => panic!("wrong char {}", c),
                }
            })
            .collect();
    }
    s
}