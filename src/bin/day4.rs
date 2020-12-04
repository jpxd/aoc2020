use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    time::Instant,
    vec,
};

lazy_static! {
    static ref REQUIRED_FIELDS: Vec<String> = vec![
        "byr".into(),
        "iyr".into(),
        "eyr".into(),
        "hgt".into(),
        "hcl".into(),
        "ecl".into(),
        "pid".into()
    ];
    static ref PID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    static ref HCL_REGEX: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    static ref ECL_REGEX: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
}

struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            fields: HashMap::new(),
        }
    }
    fn valid1(&self) -> bool {
        return REQUIRED_FIELDS
            .iter()
            .filter(|k| self.fields.contains_key(&k.to_string()))
            .count()
            == REQUIRED_FIELDS.len();
    }
    fn valid2(&self) -> Option<()> {
        let byr: i32 = self.fields.get("byr")?.parse().ok()?;
        if !(byr >= 1920 && byr <= 2002) {
            return None;
        }
        let iyr: i32 = self.fields.get("iyr")?.parse().ok()?;
        if !(iyr >= 2010 && iyr <= 2020) {
            return None;
        }
        let eyr: i32 = self.fields.get("eyr")?.parse().ok()?;
        if !(eyr >= 2020 && eyr <= 2030) {
            return None;
        }
        let hgt = self.fields.get("hgt")?;
        let hgt_len = hgt.len();
        let hgt_num: i32 = hgt[..hgt_len - 2].parse().ok()?;
        let hgt_unit = &hgt[hgt_len - 2..hgt_len];
        let hgt_okay = match hgt_unit {
            "cm" => hgt_num >= 150 && hgt_num <= 193,
            "in" => hgt_num >= 59 && hgt_num <= 76,
            _ => false,
        };
        if !hgt_okay {
            return None;
        }
        if !HCL_REGEX.is_match(self.fields.get("hcl")?) {
            return None;
        }
        if !ECL_REGEX.is_match(self.fields.get("ecl")?) {
            return None;
        }
        if !PID_REGEX.is_match(self.fields.get("pid")?) {
            return None;
        }
        Some(())
    }
}

fn parse(filename: &str) -> Option<Vec<Passport>> {
    let file = File::open(filename).ok()?;
    let lines = io::BufReader::new(file).lines();
    let mut passports: Vec<Passport> = vec![];
    let mut current = Passport::new();
    for line in lines {
        let line_str = line.unwrap();
        if line_str.is_empty() {
            passports.push(current);
            current = Passport::new();
            continue;
        }
        line_str.split_whitespace().for_each(|assignment| {
            let tokens: Vec<&str> = assignment.split(':').collect();
            current
                .fields
                .insert(tokens[0].to_string(), tokens[1].to_string());
        });
    }
    passports.push(current);
    return Some(passports);
}

fn main() {
    // Parse input
    let parsing_begin = Instant::now();
    let passports = parse("./inputs/day4.txt").unwrap();
    let parsing_elapsed = parsing_begin.elapsed();

    // Count the trees
    let computing_begin = Instant::now();
    let valid1 = passports.iter().filter(|p| p.valid1()).count();
    let valid2 = passports.iter().filter(|p| p.valid2().is_some()).count();
    let computing_elapsed = computing_begin.elapsed();

    // Print results
    println!("part1: {}", valid1);
    println!("part2: {}", valid2);

    // Print timings
    println!();
    println!("parsing took {:.2?}", parsing_elapsed);
    println!("computing took {:.2?}", computing_elapsed);
}
