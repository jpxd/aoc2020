use std::{fs, ops::RangeInclusive, time::Instant};
struct Notes {
    rules: Vec<(String, Vec<RangeInclusive<i64>>)>,
    my_ticket: Vec<i64>,
    nearby_tickets: Vec<Vec<i64>>,
}

fn parse(filename: &str) -> Option<Notes> {
    let input = fs::read_to_string(filename).ok()?;
    let sections: Vec<&str> = input.split("\n\n").collect();
    let rules = sections[0]
        .split("\n")
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let name = parts[0].to_string();
            let ranges = parts[1]
                .split(" or ")
                .map(|r| {
                    let numbers: Vec<i64> = r.split("-").map(|n| n.parse().unwrap()).collect();
                    numbers[0]..=numbers[1]
                })
                .collect();
            (name, ranges)
        })
        .collect();
    let my_ticket = sections[1].split("\n").collect::<Vec<&str>>()[1]
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    let nearby_tickets = sections[2].split("\n").collect::<Vec<&str>>()[1..]
        .iter()
        .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();
    Some(Notes {
        rules,
        my_ticket,
        nearby_tickets,
    })
}

impl Notes {
    fn invalid_ticket_numbers(&self, ticket: &Vec<i64>) -> Vec<i64> {
        ticket
            .iter()
            .filter(|value| {
                self.rules
                    .iter()
                    .find(|(_name, ranges)| {
                        ranges.iter().find(|range| range.contains(value)).is_some()
                    })
                    .is_none()
            })
            .map(|value| *value)
            .collect()
    }
    fn ticket_scanning_error_rate(&self) -> i64 {
        self.nearby_tickets
            .iter()
            .map(|ticket| self.invalid_ticket_numbers(ticket))
            .flatten()
            .sum()
    }
    fn departure_code(&self) -> i64 {
        let valid_tickets: Vec<&Vec<i64>> = self
            .nearby_tickets
            .iter()
            .filter(|ticket| self.invalid_ticket_numbers(ticket).is_empty())
            .collect();
        let mut label_candidates: Vec<(usize, Vec<String>)> = (0..self.rules.len())
            .map(|i| {
                (
                    i,
                    self.rules
                        .iter()
                        .filter(|(_name, ranges)| {
                            valid_tickets
                                .iter()
                                .find(|ticket| {
                                    !ranges.iter().any(|range| range.contains(&ticket[i]))
                                })
                                .is_none()
                        })
                        .map(|(name, _rules)| name.to_string())
                        .collect(),
                )
            })
            .collect();
        let mut ordered_labels: Vec<String> = vec![String::new(); self.rules.len()];
        while !label_candidates.is_empty() {
            let (i, candidate) = label_candidates
                .iter_mut()
                .find(|(_i, c)| c.len() == 1)
                .unwrap()
                .clone();
            let label = candidate[0].clone();
            ordered_labels[i] = label.clone();
            label_candidates.retain(|(j, _c)| i != *j);
            label_candidates
                .iter_mut()
                .for_each(|(_i, c)| c.retain(|e| *e != label))
        }
        ordered_labels
            .iter()
            .enumerate()
            .filter(|(_i, name)| name.starts_with("departure"))
            .map(|(i, _name)| self.my_ticket[i])
            .fold(1, |prev, value| prev * value)
    }
}

fn main() {
    // Parse input
    let parsing_begin = Instant::now();
    let notes = parse("./inputs/day16.txt").unwrap();
    let parsing_elapsed = parsing_begin.elapsed();

    // Compute solution
    let computing_begin = Instant::now();
    let res1 = notes.ticket_scanning_error_rate();
    let res2 = notes.departure_code();
    let computing_elapsed = computing_begin.elapsed();

    // Print results
    println!("part1: {}", res1);
    println!("part2: {}", res2);

    // Print timings
    println!();
    println!("parsing took {:.2?}", parsing_elapsed);
    println!("computing took {:.2?}", computing_elapsed);
}
