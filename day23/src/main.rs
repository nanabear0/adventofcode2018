#![allow(unused)]

extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

fn in_distance(x: (i32, i32, i32), y: (i32, i32, i32), r: i32) -> bool {
    ((x.0 - y.0).abs() + (x.1 - y.1).abs() + (x.2 - y.2).abs()) <= r
}

fn can_travel_to(
    (px, py, pz): (i32, i32, i32),
    (minx, miny, minz): (i32, i32, i32),
    (maxx, maxy, maxz): (i32, i32, i32),
    r: i32,
) -> i32 {
    let opx1;
    let opx2;
    let opy1;
    let opy2;
    let opz1;
    let opz2;

    if pz > maxz {
        opz1 = pz;
        opz2 = maxz;
    } else if pz < minz {
        opz1 = pz;
        opz2 = minz;
    } else {
        opz1 = 0;
        opz2 = 0;
    }

    if py > maxy {
        opy1 = py;
        opy2 = maxy;
    } else if py < miny {
        opy1 = py;
        opy2 = miny;
    } else {
        opy1 = 0;
        opy2 = 0;
    }

    if px > maxx {
        opx1 = px;
        opx2 = maxx;
    } else if px < minx {
        opx1 = px;
        opx2 = minx;
    } else {
        opx1 = 0;
        opx2 = 0;
    }
    in_distance((opx1, opy1, opz1), (opx2, opy2, opz2), r) as i32
}

fn count_points(
    (minx, miny, minz): (i32, i32, i32),
    (maxx, maxy, maxz): (i32, i32, i32),
    points: &[(i32, i32, i32, i32)],
) -> i32 {
    let mut count = 0;
    for (px, py, pz, r) in points {
        count += can_travel_to((*px, *py, *pz), (minx, miny, minz), (maxx, maxy, maxz), *r);
    }
    count
}

fn search(
    (minx, miny, minz): (i32, i32, i32),
    (maxx, maxy, maxz): (i32, i32, i32),
    points: &[(i32, i32, i32, i32)],
) {
    let mode = 4 * (maxx != minx) as i32 + 2 * (maxy != miny) as i32 + (maxz != minz) as i32;
    let midx = (minx + maxx) / 2;
    let midy = (miny + maxy) / 2;
    let midz = (minz + maxz) / 2;
    let mut ops: Vec<((i32, i32, i32), (i32, i32, i32))> = Vec::new();
    match mode {
        1 => {
            ops.push(((minx, miny, minz), (maxx, maxy, midz)));
            ops.push(((minx, miny, midz + 1), (maxx, maxy, maxz)));
        }
        2 => {
            ops.push(((minx, miny, minz), (maxx, midy, maxz)));
            ops.push(((minx, midy + 1, minz), (maxx, maxy, maxz)));
        }
        3 => {
            ops.push(((minx, miny, minz), (maxx, midy, midz)));
            ops.push(((minx, miny, midz + 1), (maxx, midy, maxz)));
            ops.push(((minx, midy + 1, minz), (maxx, maxy, midz)));
            ops.push(((minx, midy + 1, midz + 1), (maxx, maxy, maxz)));
        }
        4 => {
            ops.push(((minx, miny, minz), (midx, maxy, maxz)));
            ops.push(((midx + 1, miny, minz), (maxx, maxy, maxz)));
        }
        5 => {
            ops.push(((minx, miny, minz), (midx, maxy, midz)));
            ops.push(((minx, miny, midz + 1), (midx, maxy, maxz)));
            ops.push(((midx + 1, miny, minz), (maxx, maxy, midz)));
            ops.push(((midx + 1, miny, midz + 1), (maxx, maxy, maxz)));
        }
        6 => {
            ops.push(((minx, miny, minz), (midx, midy, maxz)));
            ops.push(((minx, midy + 1, minz), (midx, maxy, maxz)));
            ops.push(((midx + 1, miny, minz), (maxx, midy, maxz)));
            ops.push(((midx + 1, midy + 1, minz), (maxx, maxy, maxz)));
        }
        7 => {
            ops.push(((minx, miny, minz), (midx, midy, midz)));
            ops.push(((minx, miny, midz + 1), (midx, midy, maxz)));
            ops.push(((minx, midy + 1, minz), (midx, maxy, midz)));
            ops.push(((minx, midy + 1, midz + 1), (midx, maxy, maxz)));
            ops.push(((midx + 1, miny, minz), (maxx, midy, midz)));
            ops.push(((midx + 1, miny, midz + 1), (maxx, midy, maxz)));
            ops.push(((midx + 1, midy + 1, minz), (maxx, maxy, midz)));
            ops.push(((midx + 1, midy + 1, midz + 1), (maxx, maxy, maxz)));
        }
        _ => {
            println!("part2: {}", maxx.abs() + maxy.abs() + maxz.abs());
            return;
        }
    }

    let where_max = ops
        .iter()
        .map(|op| (*op, count_points(op.0, op.1, points)))
        .max_by(|x, y| x.1.cmp(&y.1))
        .unwrap()
        .0;

    search(where_max.0, where_max.1, points);
}

fn main() {
    let now = Instant::now();
    let br = BufReader::new(File::open("input.txt").unwrap());
    let re = Regex::new(r"pos=<(-{0,1}\d+),(-{0,1}\d+),(-{0,1}\d+)>, r=(\d+)").unwrap();
    let mut minx = i32::max_value();
    let mut maxx = i32::min_value();
    let mut miny = i32::max_value();
    let mut maxy = i32::min_value();
    let mut minz = i32::max_value();
    let mut maxz = i32::min_value();
    let points: Vec<(i32, i32, i32, i32)> = br
        .lines()
        .map(|x| x.unwrap())
        .map(|s| {
            let cap = re.captures_iter(s.as_str()).next().unwrap();
            let x = cap[1].parse::<i32>().unwrap();
            let y = cap[2].parse::<i32>().unwrap();
            let z = cap[3].parse::<i32>().unwrap();
            let r = cap[4].parse::<i32>().unwrap();
            minx = std::cmp::min(minx, x);
            maxx = std::cmp::max(maxx, x);
            miny = std::cmp::min(miny, y);
            maxy = std::cmp::max(maxy, y);
            minz = std::cmp::min(minz, z);
            maxz = std::cmp::max(maxz, z);
            (
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
                cap[3].parse::<i32>().unwrap(),
                cap[4].parse::<i32>().unwrap(),
            )
        })
        .collect();
    let max_p1 = points.iter().max_by(|x, y| x.3.cmp(&y.3)).unwrap();
    let in_range = points
        .iter()
        .filter(|(x, y, z, _)| {
            ((x - max_p1.0).abs() + (y - max_p1.1).abs() + (z - max_p1.2).abs()) <= max_p1.3
        })
        .count();
    println!("part1 : {}", in_range);

    search((minx, miny, minz), (maxx, maxy, maxz), &points);

    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}
