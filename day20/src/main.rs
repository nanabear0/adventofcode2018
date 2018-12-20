use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::Chars;
use std::sync::Arc;
use std::time::{Duration, Instant};

fn move_workers_update_grid(
    c: char,
    workers: &mut Vec<(i32, i32, i32)>,
    grid: &mut BTreeMap<(i32, i32), (bool, bool, bool, bool)>,
) {
    for w in workers.iter_mut() {
        match c {
            'N' => {
                grid.get_mut(&(w.1, w.2)).unwrap().0 = true;
                grid.entry((w.1 - 1, w.2))
                    .or_insert((false, false, true, false))
                    .2 = true;
                w.1 -= 1;
            }
            'E' => {
                grid.get_mut(&(w.1, w.2)).unwrap().1 = true;
                grid.entry((w.1, w.2 + 1))
                    .or_insert((false, false, false, true))
                    .3 = true;
                w.2 += 1;
            }
            'S' => {
                grid.get_mut(&(w.1, w.2)).unwrap().2 = true;
                grid.entry((w.1 + 1, w.2))
                    .or_insert((true, false, false, false))
                    .0 = true;
                w.1 += 1;
            }
            'W' => {
                grid.get_mut(&(w.1, w.2)).unwrap().3 = true;
                grid.entry((w.1, w.2 - 1))
                    .or_insert((false, true, false, false))
                    .1 = true;
                w.2 -= 1;
            }
            _ => {}
        }
    }
}
fn generate(iter: &mut Chars, grid: &mut BTreeMap<(i32, i32), (bool, bool, bool, bool)>) {
    let mut workers: Vec<(i32, i32, i32)> = Vec::new();
    let mut worker_level = 0;
    for c in iter {
        match c {
            'N' => move_workers_update_grid(c, &mut workers, grid),
            'E' => move_workers_update_grid(c, &mut workers, grid),
            'S' => move_workers_update_grid(c, &mut workers, grid),
            'W' => move_workers_update_grid(c, &mut workers, grid),
            '(' => {
                worker_level += 1;
            }
            ')' => {}
            '|' => {}
            '$' => {
                break;
            }
            '^' => workers.push((worker_level, 0, 0)),
            _ => {}
        }
    }
}
fn main() {
    let now = Instant::now();
    let mut br = BufReader::new(File::open("input.txt").unwrap());
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut grid: BTreeMap<(i32, i32), (bool, bool, bool, bool)> = BTreeMap::new();
    grid.insert((0, 0), (false, false, false, false));

    generate(&mut line.chars(), &mut grid);
    println!("{:?}", grid);

    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}
