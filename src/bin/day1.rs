use std::{fs::File, time::Instant};
use std::io::{self, BufRead};

fn parse(filename: &str)-> Option<Vec<i32>>{
    let file = File::open(filename).ok()?;
    let lines = io::BufReader::new(file).lines();
    let numbers: Vec<i32> = lines.into_iter().map(|l| {
        l.unwrap().parse().unwrap()
    }).collect();
    return Some(numbers)
}

fn main() {
    // Parse input
    let parsing_begin = Instant::now();
    let numbers = parse("./inputs/day1.txt").unwrap();
    let parsing_elapsed = parsing_begin.elapsed();

    // Search matching numbers
    let computing_begin = Instant::now();
    let mut result1 = 0;
    let mut result2 = 0;
    for (i, a) in numbers.iter().enumerate() {
        for (j, b) in numbers[i..].iter().enumerate() {
            if a+b == 2020 {
                result1 = a*b;
            }
            for c in numbers[i+j..].iter() {
                if a+b+c == 2020 {
                    result2 = a*b*c
                }
            }
        }
    }
    let computing_elapsed = computing_begin.elapsed();

    // Print results
    println!("part1: {}", result1);
    println!("part2: {}", result2);

    // Print timings
    println!();
    println!("parsing took {:.2?}", parsing_elapsed);
    println!("computing took {:.2?}", computing_elapsed);
}