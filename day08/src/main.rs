extern crate itertools;
extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct TreeNode {
    metadata: Vec<i32>,
    children: Vec<TreeNode>,
}

fn part2(node: &TreeNode) -> i32 {
    if node.children.len() == 0 {
        node.metadata.iter().sum::<i32>()
    } else {
        node.metadata
            .iter()
            .filter(|m| node.children.len() >= **m as usize)
            .map(|m| part2(node.children.get((m - 1) as usize).unwrap()))
            .sum::<i32>()
    }
}

fn part1(node: &TreeNode) -> i32 {
    node.metadata.iter().sum::<i32>() + node.children.iter().map(|x| part1(x)).sum::<i32>()
}

fn iterate_over_data(data: &mut Vec<i32>) -> TreeNode {
    let children_count: i32 = data.remove(0);
    let metadata_size: i32 = data.remove(0);
    let mut node = TreeNode {
        metadata: Vec::new(),
        children: Vec::new(),
    };
    for _ in 0..children_count {
        node.children.push(iterate_over_data(data));
    }
    for _ in 0..metadata_size {
        node.metadata.push(data.remove(0));
    }
    return node;
}
fn main() {
    let now = Instant::now();
    let br = BufReader::new(File::open("input.txt").unwrap());
    let re = Regex::new(r"\d+").unwrap();
    let mut data: Vec<i32> = Vec::new();
    br.lines().map(|x| x.unwrap()).for_each(|x| {
        re.captures_iter(x.as_str()).for_each(|y| {
            data.push(y[0].parse::<i32>().unwrap());
        });
    });
    let tree: TreeNode = iterate_over_data(&mut data);
    println!("part1 :{}", part1(&tree));
    println!("part2 :{}", part2(&tree));
    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}
