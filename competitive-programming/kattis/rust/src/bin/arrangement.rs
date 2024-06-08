use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();

    let mut n = String::new();
    stdin.lock().read_line(&mut n).unwrap();
    let rooms = n.trim().parse::<u32>().unwrap();

    let mut m = String::new();
    stdin.lock().read_line(&mut m).unwrap();
    let teams = m.trim().parse::<u32>().unwrap();

    let teams_per_room = teams / rooms;
    let mut extra_teams = teams % rooms;

    for _ in 0..rooms {
        let x = if extra_teams > 0 {
            extra_teams -= 1;
            teams_per_room + 1
        } else {
            teams_per_room
        };

        println!("{}", "*".repeat(x as usize));
    }
}
