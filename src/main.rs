extern crate rayon;
use rayon::prelude::*;

#[derive(Debug)]
struct Generator {
    divisor: u64,
    factor: u64,
    cur_value: u64,
}

impl Generator {
    pub fn new(factor: u64, seed: u64) -> Self {
        Self {
            divisor: 2147483647,
            factor: factor,
            cur_value: seed,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur_value = (self.cur_value * self.factor) % self.divisor;
        Some(self.cur_value)
    }
}

fn main() {
    let a = 16807;
    let b = 48271;

    let a = Generator::new(a, 65);
    let b = Generator::new(b, 8921);

    let count = 40_000_000;

    let values_a: Vec<u16> = a.take(count)
        .map(|x| (x & 0xFFFF) as u16)
        .collect();

    let values_b: Vec<u16> = b.take(count)
        .map(|x| (x & 0xFFFF) as u16)
        .collect();

    let count_match = values_a.par_iter()
        .zip(values_b.par_iter())
        .filter(|&(x, y)| x == y)
        .count();

    println!("{}", count_match);
}
