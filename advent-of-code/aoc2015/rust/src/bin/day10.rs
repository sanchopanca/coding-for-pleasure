use itertools::Itertools;

fn main() {
    let mut s = String::from("1113222113");
    for _ in 0..40 {
        s = f(s);
    }
    println!("{}", s.len());

    for _ in 0..10 {
        s = f(s);
    }
    println!("{}", s.len());
}

fn f(s: String) -> String {
    s.chars()
        .group_by(|c| *c)
        .into_iter()
        .map(|(el, group)| format!("{cnt}{el}", el = el, cnt = group.count()))
        .join("")
}
