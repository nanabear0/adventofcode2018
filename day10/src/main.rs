extern crate itertools;
extern crate regex;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

#[allow(dead_code)]
#[derive(Debug)]
struct Star {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Star {
    fn shoot(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }
}

fn part1(stars: &mut Vec<Star>) {
    let mut time = 0;
    loop {
        time += 1;
        for s in stars.iter_mut() {
            s.shoot();
        }
        if let MinMax(miny, maxy) = stars.iter().map(|s| s.y).minmax() {
            if maxy - miny < 10 {
                break;
            }
        }
    }
    if let (MinMax(minx, maxx), MinMax(miny, maxy)) = (
        stars.iter().map(|s| s.x).minmax(),
        stars.iter().map(|s| s.y).minmax(),
    ) {
        let mut p: Vec<Vec<bool>> =
            vec![vec![false; (maxx - minx + 1) as usize]; (maxy - miny + 1) as usize];
        stars.iter().for_each(|s| {
            p[(s.y - miny) as usize][(s.x - minx) as usize] = true;
        });
        println!(
            "{} Will appear after {}",
            p.iter()
                .map(|s| s
                    .iter()
                    .map(|x| if *x { '█' } else { ' ' })
                    .collect::<String>()
                    + "\n")
                .collect::<String>(),
            time
        );
    }
}

fn main() {
    let now = Instant::now();
    let br = BufReader::new(File::open("input.txt").unwrap());
    let mut stars: Vec<Star> = Vec::new();
    let re = Regex::new(
        r"position=<\s*(-{0,1}\d+),\s*(-{0,1}\d+)> velocity=<\s*(-{0,1}\d+),\s*(-{0,1}\d+)>\s*",
    )
    .unwrap();
    br.lines().map(|x| x.unwrap()).for_each(|s| {
        re.captures_iter(&s).for_each(|cap| {
            stars.push(Star {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
                vx: cap[3].parse().unwrap(),
                vy: cap[4].parse().unwrap(),
            });
        })
    });
    part1(&mut stars);
    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}
