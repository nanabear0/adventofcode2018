use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn day11() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    println!(
        "{}",
        br.lines()
            .map(|x| x.unwrap().parse::<i32>().unwrap() as i32)
            .sum::<i32>()
    );
}
fn day12() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let v: Vec<i32> = br.lines().map(|x| x.unwrap().parse().unwrap()).collect();
    let mut set: HashSet<i32> = HashSet::new();
    let mut result = 0;
    for (i, x) in v.iter().cycle().enumerate() {
        result += x;
        if !set.insert(result) {
            println!("{} {}", result, i);
            break;
        }
    }
}
fn main() {
    day12();
}
