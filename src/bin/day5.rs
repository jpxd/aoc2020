use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    time::Instant,
};

struct BoardingPass {
    seat: String,
}

impl BoardingPass {
    fn seat_position(&self) -> (i32, i32) {
        let mut row_low = 0;
        let mut row_high = 127;
        let mut column_low = 0;
        let mut column_high = 7;
        for c in self.seat.chars() {
            match c {
                'F' => row_high = row_low + (row_high - row_low) / 2,
                'B' => row_low = row_high - (row_high - row_low) / 2,
                'R' => column_low = column_high - (column_high - column_low) / 2,
                'L' => column_high = column_low + (column_high - column_low) / 2,
                _ => (),
            }
        }
        (row_low, column_high)
    }
    fn seat_id(&self) -> i32 {
        let (row, column) = self.seat_position();
        row * 8 + column
    }
}

fn parse(filename: &str) -> Option<Vec<BoardingPass>> {
    let file = File::open(filename).ok()?;
    let lines = io::BufReader::new(file).lines();
    let passes: Vec<BoardingPass> = lines.map(|l| BoardingPass { seat: l.unwrap() }).collect();
    return Some(passes);
}

fn main() {
    // Parse input
    let parsing_begin = Instant::now();
    let passes = parse("./inputs/day5.txt").unwrap();
    let parsing_elapsed = parsing_begin.elapsed();

    // Compute solution
    let computing_begin = Instant::now();
    let seat_ids: HashSet<i32> = passes.iter().map(|p| p.seat_id()).collect();
    let max_seat_id = seat_ids.iter().max().unwrap();
    let mut my_seat_id = -1;
    for r in 0..127 {
        for c in 0..8 {
            let id = r * 8 + c;
            if !seat_ids.contains(&id)
                && seat_ids.contains(&(id - 1))
                && seat_ids.contains(&(id + 1))
            {
                my_seat_id = id;
            }
        }
    }
    let computing_elapsed = computing_begin.elapsed();

    // Print results
    println!("part1: {}", max_seat_id);
    println!("part2: {}", my_seat_id);

    // Print timings
    println!();
    println!("parsing took {:.2?}", parsing_elapsed);
    println!("computing took {:.2?}", computing_elapsed);
}
