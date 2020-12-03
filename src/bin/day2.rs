use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
    time::Instant,
};

struct Policy {
    num1: usize,
    num2: usize,
    symbol: char,
}

struct Password {
    policy: Policy,
    password: String,
}

impl Password {
    fn validate1(&self) -> bool {
        let count = self
            .password
            .chars()
            .map(|c| c == self.policy.symbol)
            .count();
        return count >= self.policy.num1 && count <= self.policy.num2;
    }
    fn validate2(&self) -> bool {
        let a = self.password.chars().nth(self.policy.num1 - 1).unwrap() == self.policy.symbol;
        let b = self.password.chars().nth(self.policy.num2 - 1).unwrap() == self.policy.symbol;
        return (a || b) && (a != b);
    }
}

fn parse(filename: &str) -> Option<Vec<Password>> {
    let file = File::open(filename).ok()?;
    let lines = io::BufReader::new(file).lines();
    let regex = Regex::new(r"^(\d+)-(\d+) (\w+): (\w+)$").ok()?;
    let mut passwords: Vec<Password> = vec![];
    for line in lines {
        let line_str = line.unwrap();
        let matches = regex.captures(&line_str)?;
        let password = Password {
            policy: Policy {
                num1: matches.get(1)?.as_str().parse().ok()?,
                num2: matches.get(2)?.as_str().parse().ok()?,
                symbol: matches.get(3)?.as_str().chars().nth(0)?,
            },
            password: matches.get(4)?.as_str().to_string(),
        };
        passwords.push(password);
    }
    return Some(passwords);
}

fn main() {
    // Parse input
    let parsing_begin = Instant::now();
    let passwords = parse("./inputs/day2.txt").unwrap();
    let parsing_elapsed = parsing_begin.elapsed();

    // Count good passwords
    let computing_begin = Instant::now();
    let good1 = passwords.iter().filter(|pwd| pwd.validate1()).count();
    let good2 = passwords.iter().filter(|pwd| pwd.validate2()).count();
    let computing_elapsed = computing_begin.elapsed();

    // Print results
    println!("part1: {}", good1);
    println!("part2: {}", good2);

    // Print timings
    println!();
    println!("parsing took {:.2?}", parsing_elapsed);
    println!("computing took {:.2?}", computing_elapsed);
}
