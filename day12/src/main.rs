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
    let mut br = BufReader::new(File::open("input.txt").unwrap());
    let re = Regex::new(r"([\.\#]{5}) => ([\.\#])").unwrap();
    let mut plants = String::new();
    let mut rules: HashMap<String, char> = HashMap::new();
    br.read_line(&mut plants).unwrap();
    plants = plants.trim().to_string();
    br.lines().map(|x| x.unwrap()).for_each(|s| {
        re.captures_iter(&s).for_each(|cap| {
            rules.insert(
                cap[1].to_string(),
                if cap[2].chars().next().unwrap() == '#' {
                    '#'
                } else {
                    '.'
                },
            );
        });
    });
    plants = ".....".to_string() + &plants.trim();
    let mut last_sum: i64 = -1;
    let mut last_dif: i64 = 65;
    let iterations = 200;
    for it in 0..iterations {
        plants = "..".to_string() + &plants.trim() + &".....".to_string();
        let mut tmp: String = String::new();
        for i in 2..plants.len() - 2 {
            tmp.push(
                if let Some(c) = rules
                    .iter()
                    .filter_map(|(pattern, outcome)| {
                        if **pattern == plants[i - 2..i + 3] {
                            Some(outcome)
                        } else {
                            None
                        }
                    })
                    .next()
                {
                    *c
                } else {
                    '.'
                },
            );
        }
        plants = tmp.trim_right_matches('.').to_string();
        let cur_sum: i64 = plants
            .chars()
            .enumerate()
            .filter(|(e, x)| *x == '#')
            .map(|(e, x)| e as i64 - 5)
            .sum::<i64>();
        last_dif = cur_sum - last_sum;
        last_sum = cur_sum;

        if it == 19 {
            println!("part 1: {}", last_sum);
        }
    }
    println!("part 2: {}", (50000000000i64 - iterations) * last_dif + last_sum);
    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}
