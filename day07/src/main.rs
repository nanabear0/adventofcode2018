extern crate itertools;
extern crate regex;
use regex::Regex;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::rc::Rc;
use std::time::{Duration, Instant};

fn day72(deps: &mut BTreeMap<char, Rc<RefCell<BTreeSet<char>>>>) {
    let mut workers: Vec<(bool, usize, char)> = vec![(true, 0, '*'); 5];
    let mut time: i32 = -1;
    while !deps.is_empty() {
        time += 1;
        let mut completed_jobs: Vec<char> = Vec::new();
        for (x, y, z) in workers.iter_mut() {
            if !*x {
                *y -= 1;
                if *y == 0 {
                    *x = true;
                    completed_jobs.push(*z);
                }
            }
        }
        for j in completed_jobs {
            deps.values().for_each(|n| {
                n.borrow_mut().remove(&j);
            });
        }
        let free_worker_ids: Vec<usize> = workers
            .iter()
            .enumerate()
            .filter(|(_, (x, _, _))| *x)
            .map(|(e, (_, _, _))| (e))
            .collect();
        let new_jobs = deps
            .iter()
            .filter(|(_, v)| v.borrow().len() == 0)
            .map(|(k, _)| *k)
            .collect::<Vec<char>>();
        let mut it = new_jobs.iter();
        for w in free_worker_ids {
            match it.next() {
                Some(d) => {
                    workers[w].2 = *d;
                    workers[w].0 = false;
                    workers[w].1 = (workers[w].2 as usize) - 4;
                    deps.remove(&d);
                }
                _ => {}
            }
        }
    }
    time += workers
        .iter_mut()
        .map(|(x, y, z)| if !*x { *y } else { 0 })
        .max()
        .unwrap() as i32;
    println!("it took {}", time);
}

fn day71(deps: &mut BTreeMap<char, Rc<RefCell<BTreeSet<char>>>>) {
    let mut result: Vec<char> = Vec::new();
    while !deps.is_empty() {
        let c: char = *deps
            .iter()
            .filter(|(_, v)| v.borrow().len() == 0)
            .map(|(k, _)| k)
            .take(1)
            .next()
            .unwrap();
        result.push(c);
        deps.remove(&c);
        deps.values().for_each(|n| {
            n.borrow_mut().remove(&c);
        });
    }
    println!("{}", result.iter().collect::<String>());
}

fn main() {
    let now = Instant::now();
    let br = BufReader::new(File::open("input.txt").unwrap());
    let re = Regex::new(r"Step (.) must be finished before step (.) can begin.").unwrap();
    let mut deps: BTreeMap<char, Rc<RefCell<BTreeSet<char>>>> = BTreeMap::new();
    br.lines()
        .map(|x| x.unwrap())
        .map(|s| {
            let cap = re.captures_iter(s.as_str()).next().unwrap();
            // x,y x->y
            (
                cap[1].chars().next().unwrap(),
                cap[2].chars().next().unwrap(),
            )
        })
        .for_each(|(x, y)| {
            deps.entry(y)
                .or_insert_with(|| Rc::new(RefCell::new(BTreeSet::new())))
                .borrow_mut()
                .insert(x);
        });
    //Not so elegant fix
    deps.entry('C')
        .or_insert_with(|| Rc::new(RefCell::new(BTreeSet::new())));
    deps.entry('I')
        .or_insert_with(|| Rc::new(RefCell::new(BTreeSet::new())));
    deps.entry('Y')
        .or_insert_with(|| Rc::new(RefCell::new(BTreeSet::new())));
    deps.entry('N')
        .or_insert_with(|| Rc::new(RefCell::new(BTreeSet::new())));
    day72(&mut deps);
    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}
