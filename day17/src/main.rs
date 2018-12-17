extern crate regex;
use regex::Regex;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Range;
use std::time::{Duration, Instant};

fn pos(y: usize, x: usize) -> usize {
    y * 2000 + x
}

fn unstabilize(y: usize, x: usize, well: &mut Vec<char>, max_y: usize) {
    well[pos(y, x)] = '|';
    if well[pos(y, x - 1)] == '~' {
        unstabilize(y, x - 1, well, max_y);
    }
    if well[pos(y, x + 1)] == '~' {
        unstabilize(y, x + 1, well, max_y);
    }
}

fn spread(y: usize, x: usize, well: &mut Vec<char>, max_y: usize) -> bool {
    let mut stabilize;
    if well[pos(y, x - 1)] == '.' {
        stabilize = drip(y, x - 1, well, max_y);
    } else {
        stabilize = true;
    }
    if well[pos(y, x + 1)] == '.' {
        stabilize = drip(y, x + 1, well, max_y) && stabilize;
    } else {
        stabilize = stabilize && true;
    }
    if stabilize {
        well[pos(y, x)] = '~';
    } else {
        unstabilize(y, x, well, max_y);
    }
    return stabilize;
}

fn drip(y: usize, x: usize, well: &mut Vec<char>, max_y: usize) -> bool {
    well[pos(y, x)] = '|';
    if y < max_y && well[pos(y + 1, x)] == '.' {
        if drip(y + 1, x, well, max_y) {
            return spread(y, x, well, max_y);
        } else {
            unstabilize(y, x, well, max_y);
            false
        }
    } else if well[pos(y + 1, x)] == '#' {
        return spread(y, x, well, max_y);
    } else if well[pos(y + 1, x)] == '~' {
        return spread(y, x, well, max_y);
    } else {
        return false;
    }
}

fn main() {
    let now = Instant::now();
    let br = BufReader::new(File::open("input.txt").unwrap());
    let regex = Regex::new(r"(.)=(\d+), (.)=(\d+)..(\d+)").unwrap();
    let mut well: Vec<char> = vec!['.'; 2000 * 2000];
    well[500] = '+';
    let mut min_y = usize::max_value();
    let mut max_y = usize::min_value();
    let mut min_x = usize::max_value();
    let mut max_x = usize::min_value();
    for input in br.lines().map(|x| x.unwrap()) {
        let caps = regex.captures(&input).unwrap();
        let id = caps[1].to_string();
        let pos = caps[2].parse::<usize>().unwrap();
        let r1 = caps[4].parse::<usize>().unwrap();
        let r2 = caps[5].parse::<usize>().unwrap();

        if id == "x" {
            max_x = std::cmp::max(max_x, pos);
            min_x = std::cmp::min(min_x, pos);
            for y in r1..=r2 {
                max_y = std::cmp::max(max_y, y);
                min_y = std::cmp::min(min_y, y);
                well[y * 2000 + pos] = '#';
            }
        } else {
            min_y = std::cmp::min(min_y, pos);
            max_y = std::cmp::max(max_y, pos);
            for x in r1..=r2 {
                well[pos * 2000 + x] = '#';
                max_x = std::cmp::max(max_x, x);
                min_x = std::cmp::min(min_x, x);
            }
        }
    }
    drip(1, 500, &mut well, max_y);
    // print_well(&well, &min_y, &max_y, &min_x, &max_x);
    println!(
        "part 1: {}",
        well.iter()
            .skip(min_y * 2000)
            .filter(|x| **x == '~' || **x == '|')
            .count()
    );
    println!(
        "part 1: {}",
        well.iter()
            .skip(min_y * 2000)
            .filter(|x| **x == '~')
            .count()
    );
    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}

#[allow(dead_code)]
fn print_well(well: &Vec<char>, min_y: &usize, max_y: &usize, min_x: &usize, max_x: &usize) {
    let mut file = File::create("foo.txt").unwrap();
    for c in 0..=*max_y {
        for r in *min_x..=(max_x + 1) {
            write!(file, "{}", well[c * 2000 + r]).unwrap();
        }
        writeln!(file).unwrap();
    }
}
