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
) -> bool {
    let sx;
    let sy;
    let sz;

    if pz > maxz {
        sz = maxz;
    } else if pz < minz {
        sz = minz;
    } else {
        sz = pz;
    }

    if py > maxy {
        sy = maxy;
    } else if py < miny {
        sy = miny;
    } else {
        sy = py;
    }

    if px > maxx {
        sx = maxx;
    } else if px < minx {
        sx = minx;
    } else {
        sx = px;
    }

    in_distance((px, py, pz), (sx, sy, sz), r)
}

fn count_points(
    minp: (i32, i32, i32),
    maxp: (i32, i32, i32),
    points: &[(i32, i32, i32, i32)],
) -> usize {
    points
        .iter()
        .filter(|(px, py, pz, r)| {
            can_travel_to((*px, *py, *pz), minp, maxp, *r)
        })
        .count()
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
