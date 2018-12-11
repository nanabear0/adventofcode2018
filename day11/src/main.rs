#[macro_use]
extern crate itertools;
extern crate regex;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

#[test]
fn calc_test() {
    assert_eq!(calc_power(122, 79, 57), -5);
    assert_eq!(calc_power(217, 196, 39), 0);
    assert_eq!(calc_power(101, 153, 71), 4);
}

fn calc_power(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x as i32 + 10;
    let mut power = (rack_id * (y as i32) + serial) * rack_id;
    power = ((power / 100) % 10) - 5;
    power
}

fn main() {
    let now = Instant::now();
    let serial: i32 = 1718; //1718
    let grid_size: usize = 300; //300
                                // [Y,X]
    let mut grid: Vec<Vec<i32>> = vec![vec![0; grid_size]; grid_size];
    for (y, row) in grid.iter_mut().enumerate() {
        for (x, element) in row.iter_mut().enumerate() {
            *element = calc_power(x as i32 + 1, y as i32 + 1, serial);
        }
    }
    for size in 1..301 {
        let (e, y, x) = iproduct!(0..(grid_size - size), 0..(grid_size - size))
            .map(|(y, x)| {
                (
                    iproduct!(y..(y + size), x..(x + size))
                        .map(|(y, x)| grid[y][x])
                        .sum::<i32>(),
                    y + 1,
                    x + 1,
                )
            })
            .max_by(|(e, _, _), (e2, _, _)| e.cmp(e2))
            .unwrap();

        println!("{}: {},{} => {}", size, x, y, e);
    }
    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}
