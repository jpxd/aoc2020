use itertools::Itertools;
use std::{
    fs::File,
    io::{self, BufRead},
    mem,
    time::Instant,
};

#[derive(Clone)]
struct Seats {
    grid: Vec<Vec<char>>,
    next: Vec<Vec<char>>,
}

fn parse(filename: &str) -> Option<Seats> {
    let file = File::open(filename).ok()?;
    let lines = io::BufReader::new(file).lines();
    let grid: Vec<Vec<char>> = lines.map(|l| l.unwrap().chars().collect_vec()).collect();
    let next = grid.clone();
    Some(Seats { grid, next })
}

impl Seats {
    fn count_surrounding_seats(&self, cy: usize, cx: usize, only_adjacent: bool) -> usize {
        let directions: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        directions
            .iter()
            .filter(|(dy, dx)| {
                let mut y = (cy as i32 + *dy) as usize;
                let mut x = (cx as i32 + *dx) as usize;
                while y < self.grid.len() && x < self.grid[y as usize].len() {
                    match self.grid[y as usize][x as usize] {
                        '#' => return true,
                        'L' => return false,
                        _ => (),
                    }
                    if only_adjacent {
                        return false;
                    }
                    y = ((y as i32) + *dy) as usize;
                    x = ((x as i32) + *dx) as usize;
                }
                false
            })
            .count()
    }
    fn sim_step(&mut self, see_only_adjacent: bool, min_neighbours_to_leave: usize) -> bool {
        let mut changed = false;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                let current = self.grid[y][x];
                if current == '.' {
                    continue;
                }
                let surrounding = self.count_surrounding_seats(y, x, see_only_adjacent);
                if current == 'L' && surrounding == 0 {
                    self.next[y][x] = '#';
                    changed = true;
                } else if current == '#' && surrounding >= min_neighbours_to_leave {
                    self.next[y][x] = 'L';
                    changed = true;
                } else {
                    self.next[y][x] = current;
                }
            }
        }
        mem::swap(&mut self.grid, &mut self.next);
        changed
    }
    fn count_occupied(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|s| s)
            .filter(|c| **c == '#')
            .count()
    }
}

fn main() {
    // Parse input
    let parsing_begin = Instant::now();
    let initial_seats = parse("./inputs/day11.txt").unwrap();
    let parsing_elapsed = parsing_begin.elapsed();

    // Compute solution
    let computing_begin = Instant::now();

    let mut seats1 = initial_seats.clone();
    while seats1.sim_step(true, 4) {}
    let res1 = seats1.count_occupied();

    let mut seats2 = initial_seats.clone();
    while seats2.sim_step(false, 5) {}
    let res2 = seats2.count_occupied();

    let computing_elapsed = computing_begin.elapsed();

    // Print results
    println!("part1: {}", res1);
    println!("part2: {}", res2);

    // Print timings
    println!();
    println!("parsing took {:.2?}", parsing_elapsed);
    println!("computing took {:.2?}", computing_elapsed);
}
