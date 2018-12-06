extern crate itertools;
extern crate regex;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

fn day42(guard_day_hour_map: HashMap<usize, HashMap<String, Vec<bool>>>) {
    let (cid, ctime, cdur) = guard_day_hour_map
        .iter()
        .map(|(id, x)| {
            let mut hour_totals = vec![0; 60];
            x.values().for_each(|x| {
                x.iter()
                    .enumerate()
                    .filter(|(_, y)| **y)
                    .for_each(|(e, _)| hour_totals[e] += 1);
            });
            let (chosen_time, chosen_duration) = hour_totals
                .iter()
                .enumerate()
                .max_by(|(_, x), (_, x2)| x.cmp(x2))
                .unwrap();
            (
                id + 0 as usize,
                chosen_time + 0 as usize,
                chosen_duration + 0 as usize,
            )
        })
        .max_by(|(_, _, t1), (_, _, t2)| t1.cmp(t2))
        .unwrap();
    println!("{} {} {}", cid, ctime, ctime * cid);
}
fn day41(guard_day_hour_map: HashMap<usize, HashMap<String, Vec<bool>>>) {
    let (id, _) = guard_day_hour_map
        .iter()
        .map(|(id, x)| {
            (
                id,
                x.values()
                    .map(|y| y.iter().filter(|z| **z).count())
                    .sum::<usize>(),
            )
        })
        .max_by(|(_, x), (_, x2)| x.cmp(x2))
        .unwrap();
    let mut day_totals = vec![0; 60];
    guard_day_hour_map.get(id).unwrap().values().for_each(|x| {
        x.iter()
            .enumerate()
            .filter(|(_, y)| **y)
            .for_each(|(e, _)| day_totals[e] += 1);
    });
    let (chosen_day, _) = day_totals
        .iter()
        .enumerate()
        .max_by(|(_, x), (_, x2)| x.cmp(x2))
        .unwrap();
    println!("{} {} {}", id, chosen_day, chosen_day * id);
}

fn main() {
    let now = Instant::now();
    let br = BufReader::new(File::open("input.txt").unwrap());
    let times: Vec<String> = br.lines().map(|x| x.unwrap()).sorted();
    let re = Regex::new(
        r"\[\d{4}-(\d{2}-\d{2}) \d{2}:(\d{2})\] (falls asleep|wakes up|Guard #(\d+) begins shift)",
    )
    .unwrap();
    let mut active_guard: usize = 0;
    let mut sleep_time: usize = 0;
    let mut guard_day_hour_map: HashMap<usize, HashMap<String, Vec<bool>>> = HashMap::new();
    for t in times {
        let cap = re.captures_iter(&t).next().unwrap();
        match cap.get(4) {
            Some(gid) => {
                active_guard = gid.as_str().parse::<usize>().unwrap();
            }
            None => {
                if cap[3].contains("falls asleep") {
                    sleep_time = cap[2].parse::<usize>().unwrap();
                } else {
                    let wake_time = cap[2].parse::<usize>().unwrap();
                    (sleep_time..wake_time).for_each(|x| {
                        guard_day_hour_map
                            .entry(active_guard)
                            .or_insert(HashMap::new())
                            .entry(String::new() + &cap[1])
                            .or_insert(vec![false; 60])[x] = true;
                    });
                }
            }
        }
    }
    day42(guard_day_hour_map);
    let d:Duration = now.elapsed();
    println!(
        "{}{:03} milliseconds",
        d.as_secs(),
        d.subsec_millis()
    );
}
