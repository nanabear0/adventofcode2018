extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[allow(dead_code)]
#[derive(Debug)]
struct Square {
    id: usize,
    offset_x: usize,
    offset_y: usize,
    size_x: usize,
    size_y: usize,
}

fn day32() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let mut map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    let mut lines = 0;
    br.lines().map(|x| x.unwrap()).for_each(|s| {
        lines += 1;
        for cap in re.captures_iter(&s) {
            let id = cap[1].parse::<usize>().unwrap();
            let offset_x = cap[2].parse::<usize>().unwrap();
            let offset_y = cap[3].parse::<usize>().unwrap();
            let size_x = cap[4].parse::<usize>().unwrap();
            let size_y = cap[5].parse::<usize>().unwrap();
            for x in offset_x..(offset_x + size_x) {
                for y in offset_y..(offset_y + size_y) {
                    map.entry((x, y)).or_insert(Vec::new()).push(id);
                }
            }
        }
    });
    let mut set: HashSet<usize> = HashSet::new();
    map.iter()
        .filter(|(_, x)| x.len() >= 2)
        .map(|(_, x)| x)
        .for_each(|v| {
            v.iter().for_each(|z| {
                set.insert(*z);
            });
        });
    let bestOne = (1..lines).find(|x| !set.contains(x)).unwrap();
    println!("{}", bestOne);
}

fn day31() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let re = Regex::new(r"#(\d)+ @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();
    br.lines().map(|x| x.unwrap()).for_each(|s| {
        for cap in re.captures_iter(&s) {
            let offset_x = cap[2].parse::<usize>().unwrap();
            let offset_y = cap[3].parse::<usize>().unwrap();
            let size_x = cap[4].parse::<usize>().unwrap();
            let size_y = cap[5].parse::<usize>().unwrap();
            for x in offset_x..(offset_x + size_x) {
                for y in offset_y..(offset_y + size_y) {
                    map.entry((x, y)).and_modify(|x| *x += 1).or_insert(1);
                }
            }
        }
    });
    let count: usize = map.iter().filter(|(_, x)| **x >= 1).count();
    println!("{}", count);
}

fn main() {
    day32();
}
