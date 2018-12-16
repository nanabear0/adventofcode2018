extern crate regex;
use regex::Regex;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

fn main() {
    let now = Instant::now();
    let br = BufReader::new(File::open("input.txt").unwrap());

    let re = Regex::new(r"^Before:\s*\[(.+)\]$|^([\d\s]+)$|^After:\s*\[(.+)\]$|^\s*$").unwrap();
    let mut before: [usize; 4] = [0, 0, 0, 0];
    let mut input: [usize; 4] = [0, 0, 0, 0];
    let mut after: [usize; 4] = [0, 0, 0, 0];
    let mut empty_lines = 0;
    let mut phase = 1;
    let mut probabilities: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut final_mapping: BTreeMap<usize, usize> = BTreeMap::new();

    let mut phase2_answer: [usize; 4] = [0, 0, 0, 0];
    let mut phase1_answer: usize = 0;
    for l in br.lines() {
        for cap in re.captures_iter(&l.unwrap()) {
            if let Some(b) = cap.get(1) {
                empty_lines = 0;
                before.copy_from_slice(
                    &b.as_str()
                        .split(", ")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()[..4],
                );
                continue;
            }

            if let Some(b) = cap.get(2) {
                input.copy_from_slice(
                    &b.as_str()
                        .split(' ')
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()[..4],
                );
                if phase == 1 {
                    empty_lines = 0;
                    continue;
                } else if phase == 2 {
                    phase2_answer = OPS_LIST[final_mapping[&input[0]]](
                        &phase2_answer,
                        input[1],
                        input[2],
                        input[3],
                    )
                }
            }

            if let Some(b) = cap.get(3) {
                empty_lines = 0;
                after.copy_from_slice(
                    &b.as_str()
                        .split(", ")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()[..4],
                );
                let res = collect_probs(&before, &input, &after);
                if res.len() >= 3 {
                    phase1_answer += 1;
                }
                let mut set = probabilities
                    .entry(input[0])
                    .or_insert_with(|| res.clone())
                    .clone();
                set = set.intersection(&res).cloned().collect();
                probabilities.remove(&input[0]);
                probabilities.insert(input[0], set);
                continue;
            }

            empty_lines += 1;
            if empty_lines == 3 {
                final_mapping = figure_out_mapping(&mut probabilities);
                phase = 2;
            }
        }
    }
    println!("part1: {}", phase1_answer);
    println!("part2: {}", phase2_answer[0]);

    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}

fn figure_out_mapping(probs: &mut HashMap<usize, HashSet<usize>>) -> BTreeMap<usize, usize> {
    let mut final_mapping: BTreeMap<usize, usize> = BTreeMap::new();
    for _ in 0..16 {
        let mut c_to_remove: Vec<usize> = Vec::new();
        for (k, c) in probs.iter().filter(|(_, v)| v.len() == 1) {
            let v = c.iter().next().unwrap();
            final_mapping.insert(*k, v.clone());
            c_to_remove.push(*v);
        }
        for c in c_to_remove {
            for v in probs.values_mut() {
                v.remove(&c);
            }
        }
    }
    final_mapping
}

type OPS = fn(&[usize; 4], usize, usize, usize) -> [usize; 4];
const OPS_LIST: [OPS; 16] = [
    addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr,
];

fn collect_probs(before: &[usize; 4], input: &[usize; 4], after: &[usize; 4]) -> HashSet<usize> {
    let mut case_probabilities: HashSet<usize> = HashSet::new();
    for (e, op) in OPS_LIST.iter().enumerate() {
        if op(&before, input[1], input[2], input[3]) == *after {
            case_probabilities.insert(e);
        }
    }
    case_probabilities
}

fn addr(vec: &[usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    let mut result = vec.to_owned();
    result[c] = result[a] + result[b];
    result
}

fn addi(vec: &[usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    let mut result = vec.to_owned();
    result[c] = result[a] + b;
    result
}

fn mulr(vec: &[usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    let mut result = vec.to_owned();
    result[c] = result[a] * result[b];
    result
}

fn muli(vec: &[usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    let mut result = vec.to_owned();
    result[c] = result[a] * b;
    result
}

fn banr(vec: &[usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    let mut result = vec.to_owned();
    result[c] = result[a] & result[b];
    result
}

fn bani(vec: &[usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    let mut result = vec.to_owned();
    result[c] = result[a] & b;
    result
}

fn borr(vec: &[usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    let mut result = vec.to_owned();
    result[c] = result[a] | result[b];
    result
}

fn bori(vec: &[usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    let mut result = vec.to_owned();
    result[c] = result[a] | b;
    result
}

fn setr(vec: &[usize; 4], a: usize, _b: usize, c: usize) -> [usize; 4] {
    let mut result = vec.to_owned();
    result[c] = result[a];
    result
}

fn seti(vec: &[usize; 4], a: usize, _b: usize, c: usize) -> [usize; 4] {
    let mut result = vec.to_owned();
    result[c] = a;
    result
}

fn gtir(vec: &[usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    let mut result = vec.to_owned();
    result[c] = (a > result[b]) as usize;
    result
}

fn gtri(vec: &[usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    let mut result = vec.to_owned();
    result[c] = (result[a] > b) as usize;
    result
}

fn gtrr(vec: &[usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    let mut result = vec.to_owned();
    result[c] = (result[a] > result[b]) as usize;
    result
}

fn eqir(vec: &[usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    let mut result = vec.to_owned();
    result[c] = (a == result[b]) as usize;
    result
}

fn eqri(vec: &[usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    let mut result = vec.to_owned();
    result[c] = (result[a] == b) as usize;
    result
}

fn eqrr(vec: &[usize; 4], a: usize, b: usize, c: usize) -> [usize; 4] {
    let mut result = vec.to_owned();
    result[c] = (result[a] == result[b]) as usize;
    result
}
