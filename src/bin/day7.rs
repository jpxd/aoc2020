use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    time::Instant,
};

use regex::Regex;

struct Colors {
    rules: HashMap<String, Vec<(String, i32)>>,
}

impl Colors {
    fn new() -> Colors {
        Colors {
            rules: HashMap::new(),
        }
    }
    fn color_can_contain_color(&self, parent: String, child: String) -> bool {
        let mut stack: Vec<String> = vec![parent];
        while let Some(current) = stack.pop() {
            if current == child {
                return true;
            }
            let next: Vec<String> = self
                .rules
                .get(&current)
                .unwrap()
                .iter()
                .map(|(name, _amount)| name.clone())
                .collect();
            stack.extend(next);
        }
        false
    }
    fn how_many_colors_can_contain_color(&self, child: String) -> usize {
        self.rules
            .keys()
            .filter(|parent| {
                **parent != child && self.color_can_contain_color(parent.to_string(), child.clone())
            })
            .count()
    }
    fn how_many_bags_in_color(&self, parent: String) -> i32 {
        let current = self.rules.get(&parent).unwrap();
        if current.is_empty() {
            return 1;
        }
        let sum_subcolors: i32 = current
            .iter()
            .map(|(name, amount)| *amount * self.how_many_bags_in_color(name.clone()))
            .sum();
        return 1 + sum_subcolors;
    }
}

fn parse(filename: &str) -> Option<Colors> {
    let file = File::open(filename).ok()?;
    let lines = io::BufReader::new(file).lines();
    let mut colors = Colors::new();
    let color_regex = Regex::new(r"^(.+?) bags contain").unwrap();
    let content_regex = Regex::new(r"(\d+) (.+?) bag").unwrap();
    for line in lines {
        let line_str = line.unwrap();
        let color_name = color_regex
            .captures(&line_str)?
            .get(1)?
            .as_str()
            .to_string();
        let rules: Vec<(String, i32)> = content_regex
            .captures_iter(&line_str)
            .filter_map(|captures| {
                let subcolor_amount: i32 = captures.get(1)?.as_str().parse().ok()?;
                let subcolor_name: String = captures.get(2)?.as_str().to_string();
                Some((subcolor_name, subcolor_amount))
            })
            .collect();
        colors.rules.insert(color_name, rules);
    }
    Some(colors)
}

fn main() {
    // Parse input
    let parsing_begin = Instant::now();
    let colors = parse("./inputs/day7.txt").unwrap();
    let parsing_elapsed = parsing_begin.elapsed();

    // Compute solution
    let computing_begin = Instant::now();
    let sum1 = colors.how_many_colors_can_contain_color("shiny gold".into());
    let sum2 = colors.how_many_bags_in_color("shiny gold".into());
    let computing_elapsed = computing_begin.elapsed();

    // Print results
    println!("part1: {}", sum1);
    println!("part2: {}", sum2);

    // Print timings
    println!();
    println!("parsing took {:.2?}", parsing_elapsed);
    println!("computing took {:.2?}", computing_elapsed);
}
