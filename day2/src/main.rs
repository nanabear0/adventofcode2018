use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn day22() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let v: Vec<String> = br.lines().map(|x| x.unwrap()).collect();
    v.iter().enumerate().for_each(|(e, x)| {
        v.iter()
            .enumerate()
            .skip(e)
            .filter(|(f, _)| e != *f)
            .for_each(|(_, y)| {
                let v: String = x
                    .chars()
                    .zip(y.chars())
                    .filter(|(cx, cy)| cx == cy)
                    .map(|(x, _)| x)
                    .collect();
                if v.len() == x.len() - 1 {
                    println!("{}", v);
                }
            });
    });
}
fn day21() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let l: Vec<String> = br.lines().map(|x| x.unwrap()).collect();
    let mut twos: u32 = 0;
    let mut threes: u32 = 0;
    for line in &l {
        let mut map: HashMap<char, u32> = HashMap::new();
        for c in line.chars() {
            match map.get_mut(&c) {
                Some(k) => {
                    *k += 1;
                }
                None => {
                    map.insert(c, 1);
                }
            }
        }
        match map.values().find(|k| **k == 2) {
            Some(_) => twos += 1,
            _ => {}
        }
        match map.values().find(|k| **k == 3) {
            Some(_) => threes += 1,
            _ => {}
        }
    }
    println!("{}", twos * threes);
}
fn main() {
    day22();
}
