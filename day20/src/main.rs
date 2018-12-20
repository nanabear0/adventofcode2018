use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::Chars;
use std::sync::Arc;
use std::time::{Duration, Instant};

enum Edge {
    Wall,
    Unknown,
    Door(Arc<Box<Vertex>>),
}

struct Vertex {
    n: Edge,
    e: Edge,
    s: Edge,
    w: Edge,
}
fn generate(iter: &mut Chars, v: &mut Vertex) {
    let mut paths: Vec<String> = Vec::new();
    let mut cur_path: String = String::new();
    loop {
        let c = iter.next().unwrap();
        match c {
            'N' => match v.n {
                Edge::Door(d) => {
                    v = &mut d;
                }
                Edge::Unknown => {
                    v.n = Edge::Door(Arc::new(Box::new(Vertex {
                        n: Edge::Unknown,
                        e: Edge::Unknown,
                        s: Edge::Unknown,
                        w: Edge::Unknown,
                    })));
                }
                _ => {}
            },
            'E' => match v.e {
                Edge::Door(d) => {
                    v = &mut d;
                }
                Edge::Unknown => {
                    v.e = Edge::Door(Arc::new(Box::new(Vertex {
                        n: Edge::Unknown,
                        e: Edge::Unknown,
                        s: Edge::Unknown,
                        w: Edge::Unknown,
                    })));
                }
                _ => {}
            },
            'S' => match v.s {
                Edge::Door(d) => {
                    v = &mut d;
                }
                Edge::Unknown => {
                    v.s = Edge::Door(Arc::new(Box::new(Vertex {
                        n: Edge::Unknown,
                        e: Edge::Unknown,
                        s: Edge::Unknown,
                        w: Edge::Unknown,
                    })));
                }
                _ => {}
            },
            'W' => match v.n {
                Edge::Door(d) => {
                    v = &mut d;
                }
                Edge::Unknown => {
                    v.n = Edge::Door(Arc::new(Box::new(Vertex {
                        n: Edge::Unknown,
                        e: Edge::Unknown,
                        s: Edge::Unknown,
                        w: Edge::Unknown,
                    })));
                }
                _ => {}
            },
            '(' => {
                generate(iter, v);
            }
            ')' => {
                return;
            }
            '|' => {
                generate(iter, v);
            }
            '$' => {
                return;
            }
            _ => {}
        }
    }
}
fn main() {
    let now = Instant::now();
    let mut br = BufReader::new(File::open("input.txt").unwrap());
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    generate(
        &mut line.chars(),
        &mut Vertex {
            n: Edge::Unknown,
            e: Edge::Unknown,
            s: Edge::Unknown,
            w: Edge::Unknown,
        }
    );
    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}
