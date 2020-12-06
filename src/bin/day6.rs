use std::{collections::HashSet, fs, time::Instant};

struct Group {
    people: Vec<HashSet<char>>,
}

impl Group {
    fn count_union(&self) -> i32 {
        let mut union: HashSet<char> = HashSet::new();
        union.reserve(32);
        for person in self.people.iter() {
            union.extend(person);
        }
        union.len() as i32
    }
    fn count_intersection(&self) -> i32 {
        let mut intersection: HashSet<char> = self.people.first().unwrap().clone();
        for person in self.people[1..].iter() {
            intersection = intersection.intersection(person).map(|c| *c).collect()
        }
        intersection.len() as i32
    }
}

fn parse(filename: &str) -> Option<Vec<Group>> {
    let input = fs::read_to_string(filename).ok()?;
    let groups: Vec<Group> = input
        .split("\n\n")
        .map(|lines| Group {
            people: (lines.split("\n").map(|ls| ls.chars().collect())).collect(),
        })
        .collect();
    Some(groups)
}

fn main() {
    // Parse input
    let parsing_begin = Instant::now();
    let groups = parse("./inputs/day6.txt").unwrap();
    let parsing_elapsed = parsing_begin.elapsed();

    // Compute solution
    let computing_begin = Instant::now();
    let sum1: i32 = groups.iter().map(|g| g.count_union()).sum();
    let sum2: i32 = groups.iter().map(|g| g.count_intersection()).sum();
    let computing_elapsed = computing_begin.elapsed();

    // Print results
    println!("part1: {}", sum1);
    println!("part2: {}", sum2);

    // Print timings
    println!();
    println!("parsing took {:.2?}", parsing_elapsed);
    println!("computing took {:.2?}", computing_elapsed);
}
