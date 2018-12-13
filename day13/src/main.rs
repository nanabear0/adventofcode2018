use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

fn parse_track() -> (BTreeMap<(i32, i32), (i32, i32, i32)>, Vec<char>) {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let mut track: Vec<char> = Vec::new();
    let mut cars: BTreeMap<(i32, i32), (i32, i32, i32)> = BTreeMap::new();
    track.reserve((150 * 150) as usize);
    for (y, line) in br.lines().map(|x| x.unwrap()).enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '>' {
                cars.insert((y as i32, x as i32), (0, 1, 0));
                track.push('-');
            } else if c == 'v' {
                cars.insert((y as i32, x as i32), (1, 0, 0));
                track.push('|');
            } else if c == '<' {
                cars.insert((y as i32, x as i32), (0, -1, 0));
                track.push('-');
            } else if c == '^' {
                cars.insert((y as i32, x as i32), (-1, 0, 0));
                track.push('|');
            } else {
                track.push(c);
            }
        }
    }
    (cars, track)
}
fn new_values_of_car(
    track: &Vec<char>,
    cy: &i32,
    cx: &i32,
    dy: &i32,
    dx: &i32,
    t: &i32,
) -> (i32, i32, i32, i32, i32) {
    let ny = *cy + *dy;
    let nx = *cx + *dx;
    let nl = track[(ny * 150 + nx) as usize];
    let mut ndy = *dy;
    let mut ndx = *dx;
    let mut nt = *t;
    if nl == '/' {
        if ndy == 0 && ndx == 1 {
            ndy = -1;
            ndx = 0;
        } else if ndy == 0 && ndx == -1 {
            ndy = 1;
            ndx = 0;
        } else if ndy == 1 && ndx == 0 {
            ndy = 0;
            ndx = -1;
        } else if ndy == -1 && ndx == 0 {
            ndy = 0;
            ndx = 1;
        }
    } else if nl == '\\' {
        if ndy == 0 && ndx == 1 {
            ndy = 1;
            ndx = 0;
        } else if ndy == 0 && ndx == -1 {
            ndy = -1;
            ndx = 0;
        } else if ndy == 1 && ndx == 0 {
            ndy = 0;
            ndx = 1;
        } else if ndy == -1 && ndx == 0 {
            ndy = 0;
            ndx = -1;
        }
    } else if nl == '+' {
        if nt == 0 {
            //left
            if ndy == 0 && ndx == 1 {
                ndy = -1;
                ndx = 0;
            } else if ndy == 0 && ndx == -1 {
                ndy = 1;
                ndx = 0;
            } else if ndy == 1 && ndx == 0 {
                ndy = 0;
                ndx = 1;
            } else if ndy == -1 && ndx == 0 {
                ndy = 0;
                ndx = -1;
            }
        } else if nt == 1 {
            //straight
        } else if nt == 2 {
            //right
            if ndy == 0 && ndx == 1 {
                ndy = 1;
                ndx = 0;
            } else if ndy == 0 && ndx == -1 {
                ndy = -1;
                ndx = 0;
            } else if ndy == 1 && ndx == 0 {
                ndy = 0;
                ndx = -1;
            } else if ndy == -1 && ndx == 0 {
                ndy = 0;
                ndx = 1;
            }
        }
        nt = (nt + 1) % 3;
    }
    (ny, nx, ndy, ndx, nt)
}
fn main() {
    let now = Instant::now();

    let (mut cars, track) = parse_track();
    'tick: loop {
        if cars.len() <= 1 {
            break 'tick;
        }
        let mut surviving_cars: BTreeMap<(i32, i32), (i32, i32, i32)> = BTreeMap::new();
        let mut crashes: BTreeSet<(i32, i32)> = BTreeSet::new();
        for (e, ((cy, cx), (dy, dx, t))) in cars.iter().enumerate() {
            if crashes.contains(&(*cy, *cx)) {
                continue;
            }
            let (ny, nx, ndy, ndx, nt) = new_values_of_car(&track, cy, cx, dy, dx, t);
            if surviving_cars.contains_key(&(ny, nx)) {
                surviving_cars.remove(&(ny, nx));
                continue;
            }
            if let Some(_) = cars.keys().skip(e + 1).find(|(y, x)| *y == ny && *x == nx) {
                crashes.insert((ny, nx));
                continue;
            }
            surviving_cars.insert((ny, nx), (ndy, ndx, nt));
        }
        cars = surviving_cars;
    }
    let ((cy, cx), _) = cars.iter().next().unwrap();
    println!("{},{}", cx, cy);

    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}
