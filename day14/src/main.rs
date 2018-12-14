use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

fn main() {
    let now = Instant::now();
    let mut recipes: Vec<u8> = [3, 7].to_vec();
    let mut e1: usize = 0;
    let mut e2: usize = 1;
    loop {
        if recipes.len() >= 100000000 + 10 {
            break;
        }
        let nr = recipes[e1] + recipes[e2];
        if nr >= 10 {
            recipes.push(nr / 10);
        }
        recipes.push(nr % 10);
        e1 = (recipes[e1] as usize + e1 + 1) % recipes.len();
        e2 = (recipes[e2] as usize + e2 + 1) % recipes.len();
    }
    let set_of_digits = recipes
        .iter()
        .skip(556061)
        .take(10)
        .map(|x| (*x + 48) as char)
        .collect::<String>();
    println!("part1: {}", set_of_digits);
    let rec_str = recipes
        .iter()
        .map(|x| (*x + 48) as char)
        .collect::<String>();
    println!("part2: {}", rec_str.find("556061").unwrap());
    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}
