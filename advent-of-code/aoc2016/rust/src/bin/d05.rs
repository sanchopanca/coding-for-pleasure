fn main() {
    let input = "uqwqemis";
    let mut i = 0;
    let mut found = 0;
    let mut password1 = String::new();
    let mut password2: [char; 8] = [' '; 8];
    while password2.iter().any(|x| *x == ' ') {
        let hash = md5::compute(format!("{}{}", input, i));
        let hash_str = format!("{:x}", hash);
        if hash_str.starts_with("00000") {
            password1.push(hash_str.chars().nth(5).unwrap());
            found += 1;
            if found == 8 {
                println!("part 1: {}", password1);
            }

            let pair = hash_str.chars().skip(5).take(2).collect::<Vec<_>>();
            if let Some(idx) = pair[0].to_digit(10) {
                if idx <= 7 {
                    let idx = idx as usize;
                    if password2[idx] == ' ' {
                        password2[idx] = pair[1];
                    }
                }
            }
        }
        i += 1;
    }
    println!("part 2: {}", String::from_iter(password2));
}
