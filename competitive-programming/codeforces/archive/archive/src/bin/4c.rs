use std::collections::HashMap;
use std::io;

fn main() {
    let mut passwords = HashMap::new();
    for line in io::stdin().lines().skip(1) {
        let password = line.unwrap();
        if !passwords.contains_key(&password) {
            println!("OK");
            passwords.insert(password, 1);
        } else {
            let suggestion = format!("{}{}", password, passwords[&password]);
            passwords.entry(password.clone()).and_modify(|x| *x += 1);
            println!("{}", suggestion);
            passwords.insert(suggestion, 1);
        }
    }
}
