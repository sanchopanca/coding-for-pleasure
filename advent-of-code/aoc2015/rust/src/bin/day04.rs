use std::io::Write;
use std::process::{Command, Stdio};
use std::str;

#[allow(dead_code)]
fn md5_subprocess(input: &str) -> String {
    let mut md5sum = Command::new("md5sum")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let stdin = md5sum.stdin.as_mut().unwrap();
    stdin.write_all(input.as_bytes()).unwrap();
    drop(stdin);

    let output = md5sum.wait_with_output().unwrap();
    let mut s = str::from_utf8(&output.stdout).unwrap().to_string();
    s.truncate(32); // md5sum prints filename in the end
    s
}

#[allow(dead_code)]
fn slow_part1() {
    // takes ~5 minutes on my machine
    let key = "bgvyzdsv".to_string();
    let mut i = 0;
    loop {
        let attempt = format!("{key}{i}");
        if md5_subprocess(&attempt).starts_with("00000") {
            println!("{}", i);
            break;
        }
        i += 1;
    }
}

pub fn main() {
    // less than a second
    let key = "bgvyzdsv".to_string();
    let mut i = 0;
    let mut found1 = false;
    loop {
        let attempt = format!("{key}{i}");
        let digest = md5::compute(&attempt);
        let result = format!("{:x}", digest);
        if !found1 && result.starts_with("00000") {
            println!("part1: {}", i);
            found1 = true;
        }
        if result.starts_with("000000") {
            println!("part2: {}", i);
            break;
        }
        i += 1;
    }
}
