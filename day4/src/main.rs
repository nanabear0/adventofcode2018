extern crate regex;
#[macro_use]
extern crate itertools;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let br = BufReader::new(File::open("input.txt").unwrap());
    let times: Vec<String> = br.lines().map(|x| x.unwrap()).sorted();
    let re = Regex::new(
        r"\[\d{4}-(\d{2}-\d{2}) \d{2}:(\d{2})\] (falls asleep|wakes up|Guard #(\d+) begins shift)",
    )
    .unwrap();
    let mut active_guard: usize = 0;
    let mut sleep_time: usize = 0;
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
                    let day = &cap[1];
                    println!(
                        "During {} guard {} slept from {} to {}",
                        day, active_guard, sleep_time, wake_time
                    );
                    //TODO work with guard sleep times
                }
            }
        }
    }
}
