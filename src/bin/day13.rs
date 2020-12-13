use std::{
    fs::File,
    io::{self, BufRead},
    time::Instant,
};
struct Shuttles {
    earliest_depart: i64,
    bus_ids: Vec<Option<i64>>,
}

fn parse(filename: &str) -> Option<Shuttles> {
    let file = File::open(filename).ok()?;
    let mut lines = io::BufReader::new(file).lines();
    let earliest_depart: i64 = lines.next()?.ok()?.parse().ok()?;
    let bus_ids: Vec<Option<i64>> = lines
        .next()?
        .ok()?
        .split(',')
        .map(|n| n.parse().ok())
        .collect();
    Some(Shuttles {
        earliest_depart,
        bus_ids,
    })
}

impl Shuttles {
    fn my_next_bus(&self) -> i64 {
        let (bus_id, depart) = self
            .bus_ids
            .iter()
            .filter_map(|id| *id)
            .map(|id| (id, self.earliest_depart - self.earliest_depart % id + id))
            .min_by_key(|(_id, depart)| *depart)
            .unwrap();
        bus_id * (depart - self.earliest_depart)
    }
    fn find_contest_schedule(&self) -> i64 {
        let mut time = 0;
        let mut interval = self.bus_ids[0].unwrap();
        for (delta, bus) in self.bus_ids.iter().enumerate() {
            if bus.is_none() {
                continue;
            }
            while (time + (delta as i64)) % bus.unwrap() != 0 {
                time += interval
            }
            if delta == self.bus_ids.len() - 1 {
                break;
            }
            let first = time;
            time += interval;
            while (time + (delta as i64)) % bus.unwrap() != 0 {
                time += interval
            }
            interval = time - first;
        }
        time
    }
}

fn main() {
    // Parse input
    let parsing_begin = Instant::now();
    let shuttles = parse("./inputs/day13.txt").unwrap();
    let parsing_elapsed = parsing_begin.elapsed();

    // Compute solution
    let computing_begin = Instant::now();
    let res1 = shuttles.my_next_bus();
    let res2 = shuttles.find_contest_schedule();
    let computing_elapsed = computing_begin.elapsed();

    // Print results
    println!("part1: {}", res1);
    println!("part2: {}", res2);

    // Print timings
    println!();
    println!("parsing took {:.2?}", parsing_elapsed);
    println!("computing took {:.2?}", computing_elapsed);
}
