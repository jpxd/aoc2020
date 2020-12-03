use std::{
    fs::File,
    io::{self, BufRead},
    time::Instant,
    vec,
};

struct Map {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Map {
    fn travel_trees(&self, dy: usize, dx: usize) -> i64 {
        let mut x = 0;
        let mut y = 0;
        let mut count = 0;
        loop {
            y += dy;
            x += dx;
            if y >= self.height {
                break;
            }
            if self.map[y][x % self.width] == '#' {
                count += 1;
            }
        }
        count
    }
}

fn parse(filename: &str) -> Option<Map> {
    let file = File::open(filename).ok()?;
    let lines = io::BufReader::new(file).lines();
    let mut map = Map {
        map: vec![],
        width: 0,
        height: 0,
    };
    for line in lines {
        let line_str = line.unwrap();
        let line_vec: Vec<char> = line_str.chars().collect();
        map.width = line_vec.len();
        map.height += 1;
        map.map.push(line_vec);
    }
    return Some(map);
}

fn main() {
    // Parse input
    let parsing_begin = Instant::now();
    let map = parse("./inputs/day3.txt").unwrap();
    let parsing_elapsed = parsing_begin.elapsed();

    // Count the trees
    let computing_begin = Instant::now();
    let trees1 = map.travel_trees(1, 3);
    let trees2 = map.travel_trees(1, 1)
        * map.travel_trees(1, 3)
        * map.travel_trees(1, 5)
        * map.travel_trees(1, 7)
        * map.travel_trees(2, 1);
    let computing_elapsed = computing_begin.elapsed();

    // Print results
    println!("part1: {}", trees1);
    println!("part2: {}", trees2);

    // Print timings
    println!();
    println!("parsing took {:.2?}", parsing_elapsed);
    println!("computing took {:.2?}", computing_elapsed);
}
