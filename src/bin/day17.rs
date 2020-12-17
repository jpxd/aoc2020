use std::{collections::HashSet, fs, mem, time::Instant};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Vec4 {
    x: i8,
    y: i8,
    z: i8,
    w: i8,
}

#[derive(Clone)]
struct Dimension {
    grid: HashSet<Vec4>,
    next: HashSet<Vec4>,
}

fn parse(filename: &str) -> Option<Dimension> {
    let input = fs::read_to_string(filename).ok()?;
    let grid: HashSet<Vec4> = input
        .split("\n")
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_x, c)| *c == '#')
                .map(move |(x, _c)| Vec4 {
                    x: x as i8,
                    y: y as i8,
                    z: 0,
                    w: 0,
                })
        })
        .collect();
    let next = grid.clone();
    Some(Dimension { grid, next })
}

impl Dimension {
    fn count_surrounding(&self, pos: Vec4) -> usize {
        let mut sum = 0;
        for x in pos.x - 1..=pos.x + 1 {
            for y in pos.y - 1..=pos.y + 1 {
                for z in pos.z - 1..=pos.z + 1 {
                    for w in pos.w - 1..=pos.w + 1 {
                        let current = Vec4 { x, y, z, w };
                        if current != pos && self.grid.contains(&current) {
                            sum += 1;
                        }
                    }
                }
            }
        }
        sum
    }
    fn sim_step(&mut self, only_three_dimensions: bool) {
        let rx = self.grid.iter().min_by_key(|p| p.x).unwrap().x - 1
            ..=self.grid.iter().max_by_key(|p| p.x).unwrap().x + 1;
        let ry = self.grid.iter().min_by_key(|p| p.y).unwrap().y - 1
            ..=self.grid.iter().max_by_key(|p| p.y).unwrap().y + 1;
        let rz = self.grid.iter().min_by_key(|p| p.z).unwrap().z - 1
            ..=self.grid.iter().max_by_key(|p| p.z).unwrap().z + 1;
        let mut rw = self.grid.iter().min_by_key(|p| p.w).unwrap().w - 1
            ..=self.grid.iter().max_by_key(|p| p.w).unwrap().w + 1;
        if only_three_dimensions {
            rw = 0..=0;
        }
        for x in rx {
            for y in ry.clone() {
                for z in rz.clone() {
                    for w in rw.clone() {
                        let pos = Vec4 { x, y, z, w };
                        let active = self.grid.contains(&pos);
                        let surrounding = self.count_surrounding(pos);
                        if (active && (surrounding == 2 || surrounding == 3))
                            || (!active && surrounding == 3)
                        {
                            self.next.insert(pos);
                        } else {
                            self.next.remove(&pos);
                        }
                    }
                }
            }
        }
        mem::swap(&mut self.grid, &mut self.next);
    }
}

fn main() {
    // Parse input
    let parsing_begin = Instant::now();
    let initial_dimension = parse("./inputs/day17.txt").unwrap();
    let parsing_elapsed = parsing_begin.elapsed();

    // Compute solution
    let computing_begin = Instant::now();

    let mut dimension1 = initial_dimension.clone();
    for _i in 0..6 {
        dimension1.sim_step(true);
    }
    let res1 = dimension1.grid.len();

    let mut dimension2 = initial_dimension.clone();
    for _i in 0..6 {
        dimension2.sim_step(false);
    }
    let res2 = dimension2.grid.len();

    let computing_elapsed = computing_begin.elapsed();

    // Print results
    println!("part1: {}", res1);
    println!("part2: {}", res2);

    // Print timings
    println!();
    println!("parsing took {:.2?}", parsing_elapsed);
    println!("computing took {:.2?}", computing_elapsed);
}
