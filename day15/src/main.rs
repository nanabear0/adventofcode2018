use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Wall,
    Unit(Unit),
    Distance(Distance),
    Target((usize, usize)),
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Unit {
    y: usize,
    x: usize,
    elf_or_goblin: char,
    hp: i32,
    ap: i32,
    id: usize,
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct Distance {
    y: usize,
    x: usize,
    distance: usize,
}

fn main() {
    let now = Instant::now();
    let br = BufReader::new(File::open("input.txt").unwrap());
    let mut map: Vec<Cell> = Vec::new();
    let mut y_size = 0;
    let x_size;
    let mut last_id = 0;
    for (y, line) in br.lines().map(|x| x.unwrap()).enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '#' {
                map.push(Cell::Wall);
            } else if cell == '.' {
                map.push(Cell::Empty);
            } else if cell == 'E' {
                map.push(Cell::Unit(Unit {
                    elf_or_goblin: 'E',
                    hp: 200,
                    x: x,
                    y: y,
                    ap: 3,
                    id: last_id,
                }));
                last_id += 1;
            } else if cell == 'G' {
                map.push(Cell::Unit(Unit {
                    elf_or_goblin: 'G',
                    hp: 200,
                    x: x,
                    y: y,
                    ap: 3,
                    id: last_id,
                }));
                last_id += 1;
            }
        }
        y_size += 1;
    }
    x_size = map.len() / y_size;

    'round: loop {
        // Find active units for this round
        let units: Vec<Unit> = map
            .iter()
            .filter_map(|x| match x {
                Cell::Unit(e) => Some(e.clone()),
                _ => None,
            })
            .collect::<Vec<Unit>>();

        //Resolve action for units
        for unit in units {
            println!("unit : {:?}", unit);
            let mut enemy = Cell::Empty;
            'find_target: loop {
                //find enemies
                let enemies = map
                    .iter()
                    .filter_map(|x| match x {
                        Cell::Unit(u) => {
                            if u.elf_or_goblin != unit.elf_or_goblin {
                                Some(u.clone())
                            } else {
                                None
                            }
                        }
                        _ => None,
                    })
                    .collect::<Vec<Unit>>();
                //fill targets cells for all enemies
                for e in enemies {
                    match map[(e.y + 1) * y_size + (e.x + 0)] {
                        Cell::Empty => {
                            map[(e.y + 1) * y_size + (e.x + 0)] = Cell::Target((e.y + 1, e.x))
                        }
                        Cell::Unit(u) => {
                            if u == unit {
                                enemy = Cell::Unit(e);
                                break 'find_target;
                            }
                        }
                        _ => {}
                    }
                    match map[(e.y - 1) * y_size + (e.x + 0)] {
                        Cell::Empty => {
                            map[(e.y - 1) * y_size + (e.x + 0)] = Cell::Target((e.y - 1, e.x))
                        }
                        Cell::Unit(u) => {
                            if u == unit {
                                enemy = Cell::Unit(e);
                                break 'find_target;
                            }
                        }
                        _ => {}
                    }
                    match map[(e.y + 0) * y_size + (e.x + 1)] {
                        Cell::Empty => {
                            map[(e.y + 0) * y_size + (e.x + 1)] = Cell::Target((e.y, e.x + 1))
                        }
                        Cell::Unit(u) => {
                            if u == unit {
                                enemy = Cell::Unit(e);
                                break 'find_target;
                            }
                        }
                        _ => {}
                    }
                    match map[(e.y + 0) * y_size + (e.x - 1)] {
                        Cell::Empty => {
                            map[(e.y + 0) * y_size + (e.x - 1)] = Cell::Target((e.y, e.x - 1))
                        }
                        Cell::Unit(u) => {
                            if u == unit {
                                enemy = Cell::Unit(e);
                                break 'find_target;
                            }
                        }
                        _ => {}
                    }
                }
                let mut cur_dist = 0;
                let mut move_target = None;
                'find_place_to_move: loop {
                    let distances = map
                        .iter()
                        .filter_map(|x| match x {
                            Cell::Unit(u) => {
                                if u.id == unit.id && cur_dist == 0 {
                                    Some(Distance {
                                        y: u.y,
                                        x: u.x,
                                        distance: 0,
                                    })
                                } else {
                                    None
                                }
                            }
                            Cell::Distance(i) => {
                                if i.distance == cur_dist {
                                    Some(i.clone())
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        })
                        .collect::<Vec<Distance>>();
                    let mut break_iter = true;
                    let mut target_distances: Vec<(usize, usize)> = Vec::new();
                    for d in distances {
                        match map[(d.y + 1) * y_size + (d.x + 0)] {
                            Cell::Empty => {
                                map[(d.y + 1) * y_size + (d.x + 0)] = Cell::Distance(Distance {
                                    y: d.y + 1,
                                    x: d.x + 0,
                                    distance: cur_dist + 1,
                                });
                                break_iter = false;
                            }
                            Cell::Target(t) => {
                                target_distances.push(t);
                            }
                            _ => {}
                        }
                        match map[(d.y - 1) * y_size + (d.x + 0)] {
                            Cell::Empty => {
                                map[(d.y - 1) * y_size + (d.x + 0)] = Cell::Distance(Distance {
                                    y: d.y - 1,
                                    x: d.x + 0,
                                    distance: cur_dist + 1,
                                });
                                break_iter = false;
                            }
                            Cell::Target(t) => {
                                target_distances.push(t);
                            }
                            _ => {}
                        }
                        match map[(d.y + 0) * y_size + (d.x + 1)] {
                            Cell::Empty => {
                                map[(d.y + 0) * y_size + (d.x + 1)] = Cell::Distance(Distance {
                                    y: d.y + 0,
                                    x: d.x + 1,
                                    distance: cur_dist + 1,
                                });
                                break_iter = false;
                            }
                            Cell::Target(t) => {
                                target_distances.push(t);
                            }
                            _ => {}
                        }
                        match map[(d.y + 0) * y_size + (d.x - 1)] {
                            Cell::Empty => {
                                map[(d.y + 0) * y_size + (d.x - 1)] = Cell::Distance(Distance {
                                    y: d.y + 0,
                                    x: d.x - 1,
                                    distance: cur_dist + 1,
                                });
                                break_iter = false;
                            }
                            Cell::Target(t) => {
                                target_distances.push(t);
                            }
                            _ => {}
                        }
                    }
                    if target_distances.len() > 0 {
                        target_distances.sort();
                        move_target = Some(target_distances.iter().next().unwrap().clone());
                        break 'find_place_to_move;
                    }
                    if break_iter == true {
                        break 'find_place_to_move;
                    }
                    cur_dist += 1;
                }
                print_map(&map, y_size);
                match move_target {
                    Some(t) => {
                        println!("will move to: {:?}", t);
                    }
                    None => {
                        println!("cant't move");
                    }
                }
                clean_map(&mut map);
                break 'find_target;
            }
            clean_map(&mut map);
            println!("target?, {:?}", enemy);
            println!();
        }
        break 'round;
    }
    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}

fn clean_map(map: &mut Vec<Cell>) {
    for m in map {
        match m {
            Cell::Target(_) => *m = Cell::Empty,
            Cell::Distance(_) => *m = Cell::Empty,
            _ => {}
        }
    }
}

fn print_map(map: &Vec<Cell>, y: usize) {
    for row in map.chunks(y) {
        for e in row {
            print!(
                "{}",
                match e {
                    Cell::Wall => '#',
                    Cell::Empty => '.',
                    Cell::Distance(i) => (i.distance % 10 + 48) as u8 as char,
                    Cell::Target(_) => '?',
                    Cell::Unit(_) => 'u',
                }
            );
        }
        println!();
    }
}
