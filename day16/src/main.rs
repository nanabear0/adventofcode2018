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
    let mut before: Vec<usize> = Vec::new();
    let mut input: Vec<usize> = Vec::new();
    let mut after: Vec<usize>;
    let mut empty_spaces = 0;
    let mut phase = 1;
    let mut probs: HashMap<usize, HashSet<String>> = HashMap::new();
    let mut final_mapping: BTreeMap<usize, String> = BTreeMap::new();
    let mut phase2: Vec<usize> = vec![0, 0, 0, 0];
    let mut phase1: usize = 0;
    for l in br.lines() {
        for cap in re.captures_iter(&l.unwrap()) {
            if let Some(b) = cap.get(1) {
                empty_spaces = 0;
                before = b
                    .as_str()
                    .split(", ")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                continue;
            }

            if let Some(b) = cap.get(2) {
                input = b
                    .as_str()
                    .split(' ')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                if phase == 1 {
                    empty_spaces = 0;
                    continue;
                } else if phase == 2 {
                    phase2 = process_phase_2(&phase2, &input, &final_mapping);
                }
            }

            if let Some(b) = cap.get(3) {
                empty_spaces = 0;
                after = b
                    .as_str()
                    .split(", ")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                let res = collect_probs(&before, &input, &after);
                if res.len() >= 3 {
                    phase1 += 1;
                }
                let mut set = probs.entry(input[0]).or_insert_with(|| res.clone()).clone();
                set = set.intersection(&res).cloned().collect();
                probs.remove(&input[0]);
                probs.insert(input[0], set);
                continue;
            }

            empty_spaces += 1;
            if empty_spaces == 3 {
                for _ in 0..16 {
                    let mut c_to_remove: Vec<String> = Vec::new();
                    for (k, c) in probs.iter().filter(|(_, v)| v.len() == 1) {
                        let v = c.iter().next().unwrap();
                        final_mapping.insert(*k, v.clone());
                        c_to_remove.push(v.clone());
                    }
                    for c in c_to_remove {
                        for v in probs.values_mut() {
                            v.remove(&c);
                        }
                    }
                }
                phase = 2;
            }
        }
    }
    println!("part1: {}", phase1);
    println!("part2: {}", phase2[0]);

    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}

fn process_phase_2(
    phase2: &[usize],
    input: &[usize],
    final_mapping: &BTreeMap<usize, String>,
) -> Vec<usize> {
    if final_mapping[&input[0]] == "addr" {
        addr(&phase2, input[1], input[2], input[3])
    } else if final_mapping[&input[0]] == "addi" {
        addi(&phase2, input[1], input[2], input[3])
    } else if final_mapping[&input[0]] == "mulr" {
        mulr(&phase2, input[1], input[2], input[3])
    } else if final_mapping[&input[0]] == "muli" {
        muli(&phase2, input[1], input[2], input[3])
    } else if final_mapping[&input[0]] == "banr" {
        banr(&phase2, input[1], input[2], input[3])
    } else if final_mapping[&input[0]] == "bani" {
        bani(&phase2, input[1], input[2], input[3])
    } else if final_mapping[&input[0]] == "borr" {
        borr(&phase2, input[1], input[2], input[3])
    } else if final_mapping[&input[0]] == "bori" {
        bori(&phase2, input[1], input[2], input[3])
    } else if final_mapping[&input[0]] == "setr" {
        setr(&phase2, input[1], input[2], input[3])
    } else if final_mapping[&input[0]] == "seti" {
        seti(&phase2, input[1], input[2], input[3])
    } else if final_mapping[&input[0]] == "gtir" {
        gtir(&phase2, input[1], input[2], input[3])
    } else if final_mapping[&input[0]] == "gtri" {
        gtri(&phase2, input[1], input[2], input[3])
    } else if final_mapping[&input[0]] == "gtrr" {
        gtrr(&phase2, input[1], input[2], input[3])
    } else if final_mapping[&input[0]] == "eqir" {
        eqir(&phase2, input[1], input[2], input[3])
    } else if final_mapping[&input[0]] == "eqri" {
        eqri(&phase2, input[1], input[2], input[3])
    } else {
        eqrr(&phase2, input[1], input[2], input[3])
    }
}

fn collect_probs(before: &[usize], input: &[usize], after: &[usize]) -> HashSet<String> {
    let mut probs: HashSet<String> = HashSet::new();
    if addr(&before, input[1], input[2], input[3]) == after {
        probs.insert("addr".to_string());
    }
    if addi(&before, input[1], input[2], input[3]) == after {
        probs.insert("addi".to_string());
    }
    if mulr(&before, input[1], input[2], input[3]) == after {
        probs.insert("mulr".to_string());
    }
    if muli(&before, input[1], input[2], input[3]) == after {
        probs.insert("muli".to_string());
    }
    if banr(&before, input[1], input[2], input[3]) == after {
        probs.insert("banr".to_string());
    }
    if bani(&before, input[1], input[2], input[3]) == after {
        probs.insert("bani".to_string());
    }
    if borr(&before, input[1], input[2], input[3]) == after {
        probs.insert("borr".to_string());
    }
    if bori(&before, input[1], input[2], input[3]) == after {
        probs.insert("bori".to_string());
    }
    if setr(&before, input[1], input[2], input[3]) == after {
        probs.insert("setr".to_string());
    }
    if seti(&before, input[1], input[2], input[3]) == after {
        probs.insert("seti".to_string());
    }
    if gtir(&before, input[1], input[2], input[3]) == after {
        probs.insert("gtir".to_string());
    }
    if gtri(&before, input[1], input[2], input[3]) == after {
        probs.insert("gtri".to_string());
    }
    if gtrr(&before, input[1], input[2], input[3]) == after {
        probs.insert("gtrr".to_string());
    }
    if eqir(&before, input[1], input[2], input[3]) == after {
        probs.insert("eqir".to_string());
    }
    if eqri(&before, input[1], input[2], input[3]) == after {
        probs.insert("eqri".to_string());
    }
    if eqrr(&before, input[1], input[2], input[3]) == after {
        probs.insert("eqrr".to_string());
    }
    probs
}

fn addr(vec: &[usize], a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut new_vec = vec.to_owned();
    new_vec[c] = new_vec[a] + new_vec[b];
    new_vec
}

fn addi(vec: &[usize], a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut new_vec = vec.to_owned();
    new_vec[c] = new_vec[a] + b;
    new_vec
}

fn mulr(vec: &[usize], a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut new_vec = vec.to_owned();
    new_vec[c] = new_vec[a] * new_vec[b];
    new_vec
}

fn muli(vec: &[usize], a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut new_vec = vec.to_owned();
    new_vec[c] = new_vec[a] * b;
    new_vec
}

fn banr(vec: &[usize], a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut new_vec = vec.to_owned();
    new_vec[c] = new_vec[a] & new_vec[b];
    new_vec
}

fn bani(vec: &[usize], a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut new_vec = vec.to_owned();
    new_vec[c] = new_vec[a] & b;
    new_vec
}

fn borr(vec: &[usize], a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut new_vec = vec.to_owned();
    new_vec[c] = new_vec[a] | new_vec[b];
    new_vec
}

fn bori(vec: &[usize], a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut new_vec = vec.to_owned();
    new_vec[c] = new_vec[a] | b;
    new_vec
}

fn setr(vec: &[usize], a: usize, _b: usize, c: usize) -> Vec<usize> {
    let mut new_vec = vec.to_owned();
    new_vec[c] = new_vec[a];
    new_vec
}

fn seti(vec: &[usize], a: usize, _b: usize, c: usize) -> Vec<usize> {
    let mut new_vec = vec.to_owned();
    new_vec[c] = a;
    new_vec
}

fn gtir(vec: &[usize], a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut new_vec = vec.to_owned();
    new_vec[c] = (a > new_vec[b]) as usize;
    new_vec
}

fn gtri(vec: &[usize], a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut new_vec = vec.to_owned();
    new_vec[c] = (new_vec[a] > b) as usize;
    new_vec
}

fn gtrr(vec: &[usize], a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut new_vec = vec.to_owned();
    new_vec[c] = (new_vec[a] > new_vec[b]) as usize;
    new_vec
}

fn eqir(vec: &[usize], a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut new_vec = vec.to_owned();
    new_vec[c] = (a == new_vec[b]) as usize;
    new_vec
}

fn eqri(vec: &[usize], a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut new_vec = vec.to_owned();
    new_vec[c] = (new_vec[a] == b) as usize;
    new_vec
}

fn eqrr(vec: &[usize], a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut new_vec = vec.to_owned();
    new_vec[c] = (new_vec[a] == new_vec[b]) as usize;
    new_vec
}
