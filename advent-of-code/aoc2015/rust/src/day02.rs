use std::cmp;
use std::fs;


fn paper(l: i32, w: i32, h: i32) -> i32 {
    let s1 = l * w;
    let s2 = w * h;
    let s3 = h * l;
    2 * s1 + 2 * s2 + 2 * s3 + cmp::min(s1, cmp::min(s2, s3))
}

fn ribbon(l: i32, w: i32, h: i32) -> i32 {
    let p1 = 2 * (l + w);
    let p2 = 2 * (w + h);
    let p3 = 2 * (h + l);
    cmp::min(p1, cmp::min(p2, p3)) + l * w * h
}

pub fn day02() {
    let dims = fs::read_to_string("../input/02.txt").unwrap();
    let mut p = 0;
    let mut r = 0;
    for dim in dims.lines() {
        let v: Vec<_> = dim.split("x").collect();
        let l: i32 = v[0].parse().unwrap();
        let w: i32 = v[1].parse().unwrap();
        let h: i32 = v[2].parse().unwrap();
        p += paper(l, w, h);
        r += ribbon(l, w, h);
    }
    println!("{}", p);
    println!("{}", r);
}