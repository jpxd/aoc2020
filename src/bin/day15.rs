use std::{fs, time::Instant};
struct Game {
    starting_numbers: Vec<usize>,
}

fn parse(filename: &str) -> Option<Game> {
    let input = fs::read_to_string(filename).ok()?;
    let starting_numbers = input.split(",").map(|n| n.parse().unwrap()).collect();
    Some(Game { starting_numbers })
}

impl Game {
    fn play_until(&self, until: usize) -> usize {
        let mut memory: Vec<usize> = vec![0; until];
        let mut last_spoken_number = 0;
        for i in 1..=self.starting_numbers.len() {
            let number = self.starting_numbers[i - 1];
            memory[number] = i;
            last_spoken_number = number;
        }
        for i in self.starting_numbers.len() + 1..until {
            let previous_number_occurence = memory[last_spoken_number];
            memory[last_spoken_number] = i;
            if previous_number_occurence != 0 {
                last_spoken_number = i - previous_number_occurence;
            } else {
                last_spoken_number = 0;
            }
        }
        last_spoken_number
    }
}

fn main() {
    // Parse input
    let parsing_begin = Instant::now();
    let game = parse("./inputs/day15.txt").unwrap();
    let parsing_elapsed = parsing_begin.elapsed();

    // Compute solution
    let computing_begin = Instant::now();
    let res1 = game.play_until(2020);
    let res2 = game.play_until(30000000);
    let computing_elapsed = computing_begin.elapsed();

    // Print results
    println!("part1: {}", res1);
    println!("part2: {}", res2);

    // Print timings
    println!();
    println!("parsing took {:.2?}", parsing_elapsed);
    println!("computing took {:.2?}", computing_elapsed);
}
