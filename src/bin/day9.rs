use itertools::Itertools;
use std::{
    fs::File,
    io::{self, BufRead},
    time::Instant,
};

struct XmasCode {
    numbers: Vec<i64>,
}

fn parse(filename: &str) -> Option<XmasCode> {
    let file = File::open(filename).ok()?;
    let lines = io::BufReader::new(file).lines();
    let numbers = lines.map(|l| l.unwrap().parse::<i64>().unwrap()).collect();
    Some(XmasCode { numbers })
}

impl XmasCode {
    fn find_first_invalid_number(&self, window_size: usize) -> i64 {
        let mut i = window_size;
        while self.numbers[i - window_size..i]
            .iter()
            .combinations(2)
            .filter(|c| *c[0] != *c[1] && *c[0] + *c[1] == self.numbers[i])
            .count()
            > 0
        {
            i += 1;
        }
        self.numbers[i]
    }
    fn find_contiguous_sum_set(&self, target_sum: i64) -> i64 {
        let mut head = 0;
        let mut tail = 0;
        let mut sum = self.numbers[0];
        while sum != target_sum || head == tail {
            head += 1;
            sum += self.numbers[head];
            while sum > target_sum {
                sum -= self.numbers[tail];
                tail += 1;
            }
        }
        self.numbers[tail..head].iter().min().unwrap()
            + self.numbers[tail..head].iter().max().unwrap()
    }
}

fn main() {
    // Parse input
    let parsing_begin = Instant::now();
    let xmas_code = parse("./inputs/day9.txt").unwrap();
    let parsing_elapsed = parsing_begin.elapsed();

    // Compute solution
    let computing_begin = Instant::now();
    let res1 = xmas_code.find_first_invalid_number(25);
    let res2 = xmas_code.find_contiguous_sum_set(res1);
    let computing_elapsed = computing_begin.elapsed();

    // Print results
    println!("part1: {}", res1);
    println!("part2: {}", res2);

    // Print timings
    println!();
    println!("parsing took {:.2?}", parsing_elapsed);
    println!("computing took {:.2?}", computing_elapsed);
}
