use itertools::Itertools;
use std::{
    fs::File,
    io::{self, BufRead},
    time::Instant,
    vec,
};

struct Adapters {
    sorted: Vec<i64>,
    cache: Vec<i64>,
}

fn parse(filename: &str) -> Option<Adapters> {
    let file = File::open(filename).ok()?;
    let lines = io::BufReader::new(file).lines();
    let mut sorted: Vec<i64> = lines
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .sorted()
        .collect();
    sorted.insert(0, 0);
    sorted.push(sorted.last().unwrap() + 3);
    let cache = vec![0; sorted.len()];
    Some(Adapters { cache, sorted })
}

impl Adapters {
    fn difference_distribution(&self) -> i64 {
        let mut jolts = self.sorted[0];
        let mut differences = [0, 0, 0, 0];
        for adapter in self.sorted.iter() {
            if *adapter < jolts {
                continue;
            }
            let diff = adapter - jolts;
            if diff > 3 {
                panic!("Missing adapter!")
            }
            differences[diff as usize] += 1;
            jolts = *adapter;
        }
        differences[1] * differences[3]
    }
    fn count_arrangements(&mut self, current: usize) -> i64 {
        if current == self.sorted.len() - 1 {
            return 1;
        }
        if self.cache[current] != 0 {
            return self.cache[current];
        }
        let mut count = 0;
        let mut next = current + 1;
        while next < self.sorted.len() && (self.sorted[next] - self.sorted[current]) <= 3 {
            count += self.count_arrangements(next);
            next += 1;
        }
        self.cache[current] = count;
        count
    }
}

fn main() {
    // Parse input
    let parsing_begin = Instant::now();
    let mut adapters = parse("./inputs/day10.txt").unwrap();
    let parsing_elapsed = parsing_begin.elapsed();

    // Compute solution
    let computing_begin = Instant::now();
    let res1 = adapters.difference_distribution();
    let res2 = adapters.count_arrangements(0);
    let computing_elapsed = computing_begin.elapsed();

    // Print results
    println!("part1: {}", res1);
    println!("part2: {}", res2);

    // Print timings
    println!();
    println!("parsing took {:.2?}", parsing_elapsed);
    println!("computing took {:.2?}", computing_elapsed);
}
