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

fn day71(deps: &mut BTreeMap<char, Rc<RefCell<BTreeSet<char>>>>) {
    let mut result: Vec<char> = Vec::new();
    while deps.len() > 0 {
        let c: char = *deps
            .iter()
            .filter(|(_, v)| v.borrow().len() == 0)
            .map(|(k, _)| k)
            .take(1)
            .next()
            .unwrap();
        result.push(c);
        deps.remove(&c);
        for n in deps.values() {
            n.borrow_mut().remove(&c);
        }
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
    day71(&mut deps);
    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}
