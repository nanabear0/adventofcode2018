use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

type OPS = fn(&[usize; 6], usize, usize, usize) -> [usize; 6];

fn p2(r2: usize) {
    let mut r0 = 0;
    for r3 in 1..=r2 {
        for r5 in 1..=(r2 / r3) {
            if r2 == (r3 * r5) {
                r0 = r3 + r0;
            }
        }
    }
    println!("part 2: {}", r0);
}

fn main() {
    let now = Instant::now();
    p1();
    p2(10551403);
    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}

fn p1() {
    let mut br = BufReader::new(File::open("input.txt").unwrap());
    let mut instructions: Vec<(String, usize, usize, usize)> = Vec::new();
    let op_set: HashMap<String, OPS> = fill_op_set();
    let mut reg: [usize; 6] = [0, 0, 0, 0, 0, 0];
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let ip: usize = buf
        .trim()
        .split(' ')
        .map(|x| x.to_string())
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap();
    for l in br.lines().map(|x| x.unwrap()) {
        let ops = l.split(' ').map(|x| x.to_string()).collect::<Vec<String>>();
        instructions.push((
            ops[0].to_string(),
            ops[1].parse::<usize>().unwrap(),
            ops[2].parse::<usize>().unwrap(),
            ops[3].parse::<usize>().unwrap(),
        ));
    }
    while reg[ip] < instructions.len() {
        let i = &instructions[reg[ip]];
        reg = op_set[&i.0](&reg, i.1, i.2, i.3);
        reg[ip] += 1;
    }
    println!("part1: {}", reg[0]);
}
fn fill_op_set() -> HashMap<String, OPS> {
    let mut op_set: HashMap<String, OPS> = HashMap::new();
    op_set.insert("addr".to_string(), addr);
    op_set.insert("addi".to_string(), addi);
    op_set.insert("mulr".to_string(), mulr);
    op_set.insert("muli".to_string(), muli);
    op_set.insert("banr".to_string(), banr);
    op_set.insert("bani".to_string(), bani);
    op_set.insert("borr".to_string(), borr);
    op_set.insert("bori".to_string(), bori);
    op_set.insert("setr".to_string(), setr);
    op_set.insert("seti".to_string(), seti);
    op_set.insert("gtri".to_string(), gtri);
    op_set.insert("gtir".to_string(), gtir);
    op_set.insert("gtrr".to_string(), gtrr);
    op_set.insert("eqir".to_string(), eqir);
    op_set.insert("eqri".to_string(), eqri);
    op_set.insert("eqrr".to_string(), eqrr);

    op_set
}
fn addr(vec: &[usize; 6], a: usize, b: usize, c: usize) -> [usize; 6] {
    let mut result = vec.to_owned();
    result[c] = result[a] + result[b];
    result
}

fn addi(vec: &[usize; 6], a: usize, b: usize, c: usize) -> [usize; 6] {
    let mut result = vec.to_owned();
    result[c] = result[a] + b;
    result
}

fn mulr(vec: &[usize; 6], a: usize, b: usize, c: usize) -> [usize; 6] {
    let mut result = vec.to_owned();
    result[c] = result[a] * result[b];
    result
}

fn muli(vec: &[usize; 6], a: usize, b: usize, c: usize) -> [usize; 6] {
    let mut result = vec.to_owned();
    result[c] = result[a] * b;
    result
}

fn banr(vec: &[usize; 6], a: usize, b: usize, c: usize) -> [usize; 6] {
    let mut result = vec.to_owned();
    result[c] = result[a] & result[b];
    result
}

fn bani(vec: &[usize; 6], a: usize, b: usize, c: usize) -> [usize; 6] {
    let mut result = vec.to_owned();
    result[c] = result[a] & b;
    result
}

fn borr(vec: &[usize; 6], a: usize, b: usize, c: usize) -> [usize; 6] {
    let mut result = vec.to_owned();
    result[c] = result[a] | result[b];
    result
}

fn bori(vec: &[usize; 6], a: usize, b: usize, c: usize) -> [usize; 6] {
    let mut result = vec.to_owned();
    result[c] = result[a] | b;
    result
}

fn setr(vec: &[usize; 6], a: usize, _b: usize, c: usize) -> [usize; 6] {
    let mut result = vec.to_owned();
    result[c] = result[a];
    result
}

fn seti(vec: &[usize; 6], a: usize, _b: usize, c: usize) -> [usize; 6] {
    let mut result = vec.to_owned();
    result[c] = a;
    result
}

fn gtir(vec: &[usize; 6], a: usize, b: usize, c: usize) -> [usize; 6] {
    let mut result = vec.to_owned();
    result[c] = (a > result[b]) as usize;
    result
}

fn gtri(vec: &[usize; 6], a: usize, b: usize, c: usize) -> [usize; 6] {
    let mut result = vec.to_owned();
    result[c] = (result[a] > b) as usize;
    result
}

fn gtrr(vec: &[usize; 6], a: usize, b: usize, c: usize) -> [usize; 6] {
    let mut result = vec.to_owned();
    result[c] = (result[a] > result[b]) as usize;
    result
}

fn eqir(vec: &[usize; 6], a: usize, b: usize, c: usize) -> [usize; 6] {
    let mut result = vec.to_owned();
    result[c] = (a == result[b]) as usize;
    result
}

fn eqri(vec: &[usize; 6], a: usize, b: usize, c: usize) -> [usize; 6] {
    let mut result = vec.to_owned();
    result[c] = (result[a] == b) as usize;
    result
}

fn eqrr(vec: &[usize; 6], a: usize, b: usize, c: usize) -> [usize; 6] {
    let mut result = vec.to_owned();
    result[c] = (result[a] == result[b]) as usize;
    result
}
