use gif::{Encoder, Frame, Repeat, SetParameter};
use std::borrow::Cow;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

const MAKE_GIF: bool = true;
const GIF_SCALE: usize = 5;

fn pos((y, x): (i32, i32)) -> usize {
    (y * 50 + x) as usize
}

const POS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn neighbours(orchard: &[char], y: i32, x: i32) -> (usize, usize) {
    let mut trees = 0;
    let mut lumberyards = 0;
    for (e, p) in POS.iter().enumerate() {
        if (e == 0 || e == 1 || e == 2) && y == 0 {
            continue;
        }
        if (e == 5 || e == 6 || e == 7) && y == 49 {
            continue;
        }
        if (e == 0 || e == 3 || e == 5) && x == 0 {
            continue;
        }
        if (e == 2 || e == 4 || e == 7) && x == 49 {
            continue;
        }
        let yx = pos((y + p.0, x + p.1));
        if let Some(val) = orchard.get(yx) {
            match val {
                '#' => lumberyards += 1,
                '|' => trees += 1,
                _ => {}
            }
        }
    }
    (trees, lumberyards)
}

fn pass_time(orchard: &mut Vec<char>) {
    let mut new_orchard = Vec::new();
    for y in 0..50 {
        for x in 0..50 {
            let (nt, nl) = neighbours(&orchard, y, x);
            let old_val = orchard[pos((y, x))];
            match old_val {
                '.' => {
                    if nt >= 3 {
                        new_orchard.push('|');
                    } else {
                        new_orchard.push('.');
                    }
                }
                '|' => {
                    if nl >= 3 {
                        new_orchard.push('#');
                    } else {
                        new_orchard.push('|');
                    }
                }
                '#' => {
                    if nl >= 1 && nt >= 1 {
                        new_orchard.push('#');
                    } else {
                        new_orchard.push('.');
                    }
                }
                _ => {}
            }
        }
    }
    *orchard = new_orchard;
}
fn main() {
    let now = Instant::now();
    let br = BufReader::new(File::open("input.txt").unwrap());
    let mut orchard = Vec::new();

    let mut color_map = Vec::new();
    let mut states: Vec<Vec<u8>> = Vec::new();
    if MAKE_GIF {
        color_map.push(0xFF);
        color_map.push(0xFF);
        color_map.push(0xFF);
        color_map.push(0x34);
        color_map.push(0xB3);
        color_map.push(0x34);
        color_map.push(0x33);
        color_map.push(0x2A);
        color_map.push(0x22);
    }
    for l in br.lines() {
        for c in l.unwrap().chars() {
            orchard.push(c);
        }
    }
    for i in 1..=5_000 {
        pass_time(&mut orchard);
        let trees = orchard.iter().filter(|x| **x == '|').count();
        let ly = orchard.iter().filter(|x| **x == '#').count();
        if (i > 1000 && i < 1100) || i == 10 {
            println!("{},{},{},{}", i, trees, ly, trees * ly);
        }
        if MAKE_GIF {
            let mut state: Vec<u8> = vec![0; 50 * 50 * GIF_SCALE * GIF_SCALE];
            for (y, row) in orchard.chunks(50).enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    match *cell {
                        '.' => {
                            scaling_fill(&mut state, y, x, 0);
                        }
                        '#' => {
                            scaling_fill(&mut state, y, x, 2);
                        }
                        '|' => {
                            scaling_fill(&mut state, y, x, 1);
                        }
                        _ => {}
                    }
                }
            }
            states.push(state);
        }
    }
    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
    if MAKE_GIF {
        let mut image = File::create("orchard.gif").unwrap();
        let mut encoder = Encoder::new(
            &mut image,
            (GIF_SCALE * 50) as u16,
            (GIF_SCALE * 50) as u16,
            &color_map,
        )
        .unwrap();
        encoder.set(Repeat::Infinite).unwrap();
        for state in &states {
            let mut frame = Frame::default();
            frame.width = (GIF_SCALE * 50) as u16;
            frame.height = (GIF_SCALE * 50) as u16;
            frame.buffer = Cow::Borrowed(state as &[u8]);
            encoder.write_frame(&frame).unwrap();
        }
        println!("File created");
    }
}

fn scaling_fill(state: &mut Vec<u8>, y: usize, x: usize, ci: u8) {
    for sy in 0..GIF_SCALE {
        for sx in 0..GIF_SCALE {
            state[(y * GIF_SCALE + sy) * 50 * GIF_SCALE + (x * GIF_SCALE + sx)] = ci;
        }
    }
}

#[allow(dead_code)]
fn print_orchard(orchard: &[char]) {
    for c in orchard.chunks(50) {
        for ch in c {
            print!("{}", ch);
        }
        println!();
    }
    println!();
}
