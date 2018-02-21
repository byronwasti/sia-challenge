extern crate rayon;
#[macro_use]
extern crate structopt;

use rayon::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(help = "Seed for A Generator", default_value = "65")]
    a_seed: u64,

    #[structopt(help = "Seed for B Generator", default_value = "8921")]
    b_seed: u64,

    #[structopt(short = "c", long = "count", help = "Number of generated values to compare", default_value = "40000000")]
    count: usize,
}

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
    let opt = Opt::from_args();

    // Create generators using default factor + seed from command line options
    let a = 16807;
    let b = 48271;
    let a = Generator::new(a, opt.a_seed);
    let b = Generator::new(b, opt.b_seed);

    // Collect values synchronously
    let values_a: Vec<u16> = a.take(opt.count)
        .map(|x| (x & 0xFFFF) as u16)
        .collect();

    let values_b: Vec<u16> = b.take(opt.count)
        .map(|x| (x & 0xFFFF) as u16)
        .collect();

    // Compare values in parallel
    let count_match = values_a.par_iter()
        .zip(values_b.par_iter())
        .filter(|&(x, y)| x == y)
        .count();

    println!("{}", count_match);
}

