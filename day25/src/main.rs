#![allow(unused)]
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

fn distance(x: (i32, i32, i32, i32), y: (i32, i32, i32, i32)) -> i32 {
    (x.0 - y.0).abs() + (x.1 - y.1).abs() + (x.2 - y.2).abs() + (x.3 - y.3).abs()
}

fn main() {
    let now = Instant::now();
    let br = BufReader::new(File::open("input.txt").unwrap());
    let mut stars_in_the_sky = HashSet::new();
    for l in br.lines() {
        let temp = l
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        stars_in_the_sky.insert((temp[0], temp[1], temp[2], temp[3]));
    }

    let mut constellation_count = 0;
    while !stars_in_the_sky.is_empty() {
        let mut this_constellation: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        let first_star = *stars_in_the_sky.iter().next().unwrap();
        this_constellation.insert(first_star);
        stars_in_the_sky.remove(&first_star);

        loop {
            let tmp_list: Vec<(i32, i32, i32, i32)> = stars_in_the_sky
                .iter()
                .filter(|x| this_constellation.iter().any(|y| distance(**x, *y) <= 3))
                .cloned()
                .collect();
            if tmp_list.is_empty() {
                break;
            }
            for new_star in tmp_list {
                this_constellation.insert(new_star);
                stars_in_the_sky.remove(&new_star);
            }
        }
        constellation_count += 1;
    }
    println!("part1: {}", constellation_count);

    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}
