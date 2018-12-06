#[macro_use]
extern crate itertools;
extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
struct Point {
    id: i32,
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Owned(i32),
    HasDist(CellDist),
    HasMultipleDists,
    Empty,
}

#[derive(Debug, Clone, Copy)]
struct CellDist {
    owner: i32,
    distance: i32,
}

fn update_neighbours(
    all: &mut Vec<Cell>,
    owner: &i32,
    x: &i32,
    y: &i32,
    size_x: &i32,
    size_y: &i32,
    distance: &i32,
) {
    if *x != 0 {
        update(&mut all[calc_dist(&(x - 1), &y, &size_x)], owner, distance);
    }
    if *x != size_x - 1 {
        update(&mut all[calc_dist(&(x + 1), &y, &size_x)], owner, distance);
    }
    if *y != 0 {
        update(&mut all[calc_dist(&x, &(y - 1), &size_x)], owner, distance);
    }
    if *y != size_y - 1 {
        update(&mut all[calc_dist(&x, &(y + 1), &size_x)], owner, distance);
    }
}

fn update(c: &mut Cell, owner: &i32, distance: &i32) {
    match c {
        Cell::Owned(x) => {
            //noop
        }
        Cell::HasDist(x) => {
            if x.distance == *distance + 1 && *owner != x.owner {
                *c = Cell::HasMultipleDists;
            }
        }
        Cell::HasMultipleDists => {
            //noop
        }
        Cell::Empty => {
            *c = Cell::HasDist(CellDist {
                owner: *owner,
                distance: *distance + 1,
            })
        }
    }
}

fn calc_dist(x: &i32, y: &i32, size_x: &i32) -> usize {
    (*x * *size_x + *y) as usize
}

fn day61() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let re = Regex::new(r"(\d+), (\d+)").unwrap();
    let size_x: i32 = 500;
    let size_y: i32 = 500;
    let mut all: Vec<Cell> = vec![Cell::Empty; (size_x * size_y) as usize];
    br.lines()
        .enumerate()
        .map(|(e, k)| (e, k.unwrap()))
        .map(|(e, s)| {
            let cap = re.captures_iter(&s).next().unwrap();
            Point {
                id: e as i32,
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
            }
        })
        .for_each(|p: Point| all[(p.x * size_x + p.y) as usize] = Cell::Owned(p.id));
    let mut cur_dist = 0;
    for _ in 1..size_x + size_y {
        for (x, y) in iproduct!(0..size_x, 0..size_y) {
            match all[calc_dist(&x, &y, &size_x)] {
                Cell::Owned(c) => {
                    if cur_dist == 0 {
                        update_neighbours(&mut all, &c, &x, &y, &size_x, &size_y, &0)
                    }
                }
                Cell::HasDist(c) => {
                    if c.distance == cur_dist {
                        update_neighbours(&mut all, &c.owner, &x, &y, &size_x, &size_y, &c.distance)
                    }
                }
                _ => {}
            }
        }
        cur_dist += 1;
    }
    let mut set: HashSet<i32> = HashSet::new();
    for (x, y) in iproduct!(0..size_x, 0..size_y) {
        if x == 0 || x == size_x - 1 || y == 0 || y == size_y - 1 {
            match all[calc_dist(&x, &y, &size_x)] {
                Cell::Owned(c) => {
                    set.insert(c);
                }
                Cell::HasDist(c) => {
                    set.insert(c.owner);
                }
                _ => {}
            }
        }
    }
    let (e, size) = (1..51)
        .map(|i| {
            (
                i,
                all.iter()
                    .filter(|x| match x {
                        Cell::Owned(c) => {
                            if !set.contains(c) {
                                return *c == i;
                            } else {
                                return false;
                            }
                        }
                        Cell::HasDist(c) => {
                            if !set.contains(&c.owner) {
                                return c.owner == i;
                            } else {
                                return false;
                            }
                        }
                        _ => return false,
                    })
                    .count(),
            )
        })
        .max_by(|(_, c), (_, c2)| c.cmp(c2))
        .unwrap();
    println!("{} {}", e, size);
}

fn day62() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let re = Regex::new(r"(\d+), (\d+)").unwrap();
    let size_x: i32 = 500;
    let size_y: i32 = 500;
    let points: Vec<Point> = br
        .lines()
        .enumerate()
        .map(|(e, k)| (e, k.unwrap()))
        .map(|(e, s)| {
            let cap = re.captures_iter(&s).next().unwrap();
            Point {
                id: e as i32,
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
            }
        })
        .collect();

    let count = iproduct!(0..size_x, 0..size_y)
        .filter(|(x, y)| {
            points
                .iter()
                .map(|p| (x - p.x).abs() + (y - p.y).abs())
                .sum::<i32>()
                < 10000
        })
        .count();
    println!("{}", count);
}

fn main() {
    let now = Instant::now();
    day62();
    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}
