use std::collections::HashMap;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

trait Cycle {
    fn cycle_cw(&mut self, count: usize);
    fn cycle_ccw(&mut self, count: usize);
}

impl<T> Cycle for VecDeque<T> {
    fn cycle_cw(&mut self, count: usize) {
        for _ in 0..count {
            let tmp = self.pop_back().unwrap();
            self.push_front(tmp);
        }
    }
    fn cycle_ccw(&mut self, count: usize) {
        for _ in 0..count {
            let tmp = self.pop_front().unwrap();
            self.push_back(tmp);
        }
    }
}

fn day91(players: usize, last_marble: usize) {
    let mut marbles: VecDeque<usize> = VecDeque::with_capacity(7152200);
    marbles.push_back(0);
    let mut cur_player = 0 as usize;
    let mut score_card: HashMap<usize, usize> = HashMap::new();
    for i in 1..last_marble + 1 {
        if i % 23 == 0 {
            marbles.cycle_ccw(7);
            *score_card.entry(cur_player).or_insert(0) += marbles.pop_back().unwrap() + i;
        } else {
            marbles.cycle_cw(2);
            marbles.push_back(i);
        }
        cur_player = (cur_player + 1) % players;
    }
    let max_score = score_card.values().max().unwrap();
    println!("{}", max_score);
}

fn main() {
    let now = Instant::now();
    day91(446, 7152200);
    let d: Duration = now.elapsed();
    println!("> {}.{:03} seconds", d.as_secs(), d.subsec_millis());
}
