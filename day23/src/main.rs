extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

fn main() {
    let now = Instant::now();
    let br = BufReader::new(File::open("input.txt").unwrap());
    let re = Regex::new(r"pos=<(-{0,1}\d+),(-{0,1}\d+),(-{0,1}\d+)>, r=(\d+)").unwrap();
    let points: Vec<(i32, i32, i32, i32)> = br
        .lines()
        .map(|x| x.unwrap())
        .map(|s| {
            let cap = re.captures_iter(s.as_str()).next().unwrap();
            (
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
                cap[3].parse::<i32>().unwrap(),
                cap[4].parse::<i32>().unwrap(),
            )
        })
        .collect();
    let max = points.iter().max_by(|x, y| x.3.cmp(&y.3)).unwrap();
    let in_range = points
        .iter()
        .filter(|(x, y, z, _)| ((x - max.0).abs() + (y - max.1).abs() + (z - max.2).abs()) <= max.3)
        .count();
    println!("part1 : {}", in_range);
    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}
