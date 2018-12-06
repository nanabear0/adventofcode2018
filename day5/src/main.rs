extern crate itertools;
extern crate regex;
use std::cmp;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

fn move_values2(source: &mut VecDeque<u8>, other: &mut VecDeque<u8>, ar: u8) -> bool {
    let mut change: bool = false;
    loop {
        match source.pop_front() {
            Some(x) => {
                if other.is_empty() || (x as i8 - *other.back().unwrap() as i8).abs() != 32 {
                    if x == ar || x == (ar + 32) {
                    } else {
                        other.push_back(x);
                    }
                } else {
                    other.pop_back();
                    change = true;
                }
            }
            None => {
                return change;
            }
        }
    }
}
fn day52() {
    let mut min: usize = 100000000000;
    for ar in 65..90 {
        let br = BufReader::new(File::open("input.txt").unwrap());
        let mut source: VecDeque<u8> = br.bytes().map(|x| x.unwrap() as u8).collect();
        let mut other: VecDeque<u8> = VecDeque::new();
        let mut change: bool = true;
        let mut mode: bool = true;
        while change {
            if mode {
                change = move_values2(&mut source, &mut other, ar);
            } else {
                change = move_values2(&mut other, &mut source, ar);
            }
            mode = !mode;
        }
        min = cmp::min(cmp::max(source.len(), other.len()), min);
    }
    println!("{}", min);
}
fn move_values(source: &mut VecDeque<u8>, other: &mut VecDeque<u8>) -> bool {
    let mut change: bool = false;
    loop {
        match source.pop_front() {
            Some(x) => {
                if other.is_empty() || (x as i8 - *other.back().unwrap() as i8).abs() != 32 {
                    other.push_back(x);
                } else {
                    other.pop_back();
                    change = true;
                }
            }
            None => {
                return change;
            }
        }
    }
}
fn day51() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let mut source: VecDeque<u8> = br.bytes().map(|x| x.unwrap() as u8).collect();
    let mut other: VecDeque<u8> = VecDeque::new();
    let mut change: bool = true;
    let mut mode: bool = true;
    while change {
        if mode {
            change = move_values(&mut source, &mut other);
        } else {
            change = move_values(&mut other, &mut source);
        }
        mode = !mode;
    }
    println!("{}", cmp::max(source.len(), other.len()));
}
fn main() {
    let now = Instant::now();
    day52();
    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}
