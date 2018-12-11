#[macro_use]
extern crate itertools;

use std::time::{Duration, Instant};

fn calc_power(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x as i32 + 10;
    let mut power = (rack_id * (y as i32) + serial) * rack_id;
    power = ((power / 100) % 10) - 5;
    power
}

fn calc_max_for(grid: &Vec<Vec<i32>>, grid_size: usize, size: usize) -> (usize, i32, usize, usize) {
    if let Some((e, y, x)) = iproduct!(0..(grid_size - size), 0..(grid_size - size))
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
    {
        (size, e, y, x)
    } else {
        (size, i32::min_value(), 0, 0)
    }
}

fn main() {
    let now = Instant::now();
    let serial: i32 = 1718; //1718
    let grid_size: usize = 300; //300
    let mut grid: Vec<Vec<i32>> = vec![vec![0; grid_size]; grid_size];
    for (y, row) in grid.iter_mut().enumerate() {
        for (x, element) in row.iter_mut().enumerate() {
            *element = calc_power(x as i32 + 1, y as i32 + 1, serial);
        }
    }
    // //part1
    // println!("{:?}", calc_max_for(&grid, grid_size, 3));
    //part2
    if let Some((size, e, y, x)) = (1..grid_size + 1)
        .map(|size| calc_max_for(&grid, grid_size, size))
        .max_by(|(_, e, _, _), (_, e2, _, _)| e.cmp(e2))
    {
        println!("{}: {},{} => {}", size, x, y, e);
    } else {
        println!("OOPSIE WOOPSIE!! Uwu We made a fucky wucky!! A wittle fucko boingo!");
    }
    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}
