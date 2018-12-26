#![allow(unused)]

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

// const DEPTH: i32 = 7863;
// const TARGET: (i32, i32) = (14, 760);
const DEPTH: i32 = 510;
const TARGET: (i32, i32) = (10, 10);
// x,y
const DIRECTIONS: &[(i32, i32); 4] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];

fn main() {
    let now = Instant::now();
    let br = BufReader::new(File::open("input.txt").unwrap());
    // (X,Y),(type,geologic_index, erosion_level)
    let mut cave: HashMap<(i32, i32), (i32, i32, i32)> = HashMap::new();
    for x in 0..=TARGET.0 * 7 {
        for y in 0..=TARGET.1 * 7 {
            let gi: i32 = if (x == 0 && y == 0) || (x == TARGET.0 && y == TARGET.1) {
                0
            } else if y == 0 {
                x * 16807
            } else if x == 0 {
                y * 48271
            } else {
                cave.get(&(x - 1, y)).unwrap().2 * cave.get(&(x, y - 1)).unwrap().2
            };
            let el = (gi + DEPTH) % 20183;
            let rt = el % 3;
            cave.insert((x, y), (rt, gi, el));
        }
    }
    let part1: i32 = cave
        .iter()
        .filter_map(|((x, y), z)| {
            if *x <= TARGET.0 && *y <= TARGET.1 {
                Some(z.0)
            } else {
                None
            }
        })
        .sum();
    // (x,y,tool)
    // tool => 0 neither, 1 torch, 2 climbing gear
    // 0 => 1,2
    // 1 => 0,2
    // 2 => 0,1
    let mut job_queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
    // (x,y), (terrain, 0_time, 1_time, 2_time)
    let mut distance_map: HashMap<(i32, i32), (i32, [i32; 3])> = HashMap::new();
    job_queue.push_back((2, 0, 0));
    // Stop if all are filled
    while !job_queue.is_empty() {
        let current_job = job_queue.pop_front().unwrap();
        // Evaluate all links
        for (dir_x, dir_y) in DIRECTIONS {
            let xy = (*dir_x + current_job.0, *dir_y + current_job.1);
            // Get target cell in the cave
            if let Some(cave_cell) = cave.get(&xy) {
                // Find current value or insert new
                let target = distance_map.entry(xy).or_insert_with(|| {
                    (
                        cave_cell.0,
                        [i32::max_value(), i32::max_value(), i32::max_value()],
                    )
                });
                // Update times and generate new jobs for tool transitions
                // This is bullshit, cba
                if current_job.2 == target.0 {
                    target.1[target.0 as usize] = 0;
                }
            }
        }
    }

    println!("{}", part1);
    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}
