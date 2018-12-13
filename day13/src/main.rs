extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

// Don't look I'm hideous.
fn main() {
    let now = Instant::now();
    let br = BufReader::new(File::open("input.txt").unwrap());
    let mut track: Vec<char> = Vec::new();
    let mut cars: HashMap<(i32, i32), (i32, i32, i32)> = HashMap::new();
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
    let mut crash = 0;
    'tick: loop {
        println!("{:?}", cars);
        if cars.len() == 1 {
            break 'tick;
        }
        let mut surviving_cars: HashMap<(i32, i32), (i32, i32, i32)> = HashMap::new();
        for ((cy, cx), (dy, dx, t)) in cars.iter_mut() {
            let mut ny = *cy + *dy;
            let mut nx = *cx + *dx;
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
            } else if nl == 'X' {
                crash += 1;
                continue;
            }
            if let None = surviving_cars.insert((ny, nx), (ndy, ndx, nt)) {
            } else {
                surviving_cars.remove(&(ny, nx));
                // track[(ny * 150 + nx) as usize] = 'X';
            }
        }
        cars = surviving_cars;
    }
    let ((cy, cx), (dy, dx, t)) = cars.iter().next().unwrap();
    println!("{},{}", cx, cy);
    // for c in track.chunks(150 as usize) {
    //     println!("{}", c.iter().collect::<String>());
    // }
    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}
