use std::{
    fs::File,
    io::{self, BufRead},
    time::Instant,
};
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn add(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
    fn rotate(&mut self, degrees: i32) {
        for _ in (0..degrees + 360).step_by(90) {
            let (old_x, old_y) = (self.x, self.y);
            self.x = -old_y;
            self.y = old_x;
        }
    }
    fn dist(&self) -> i32 {
        i32::abs(self.x) + i32::abs(self.y)
    }
}

struct Ship {
    actions: Vec<(char, i32)>,
    direction: i32,
    position: Vec2,
    waypoint: Vec2,
}

fn parse(filename: &str) -> Option<Ship> {
    let file = File::open(filename).ok()?;
    let lines = io::BufReader::new(file).lines();
    let actions: Vec<(char, i32)> = lines
        .map(|l| {
            let line = l.unwrap();
            let action = line.chars().next().unwrap();
            let parameter: i32 = line[1..].parse().unwrap();
            (action, parameter)
        })
        .collect();
    Some(Ship {
        actions,
        direction: 0,
        position: Vec2 { x: 0, y: 0 },
        waypoint: Vec2 { x: 0, y: 0 },
    })
}

impl Ship {
    fn reset(&mut self, direction: i32, position: Vec2, waypoint: Vec2) {
        self.direction = direction;
        self.position = position;
        self.waypoint = waypoint;
    }

    fn navigate_by_action(&mut self) -> i32 {
        self.actions.clone().iter().for_each(|(mut action, param)| {
            if action == 'F' {
                action = match self.direction {
                    0 => 'N',
                    90 => 'E',
                    180 => 'S',
                    270 => 'W',
                    _ => '?',
                }
            }
            match action {
                'N' => self.position.y -= param,
                'S' => self.position.y += param,
                'E' => self.position.x += param,
                'W' => self.position.x -= param,
                'L' => self.direction = (self.direction - param + 360) % 360,
                'R' => self.direction = (self.direction + param) % 360,
                _ => panic!("invalid action"),
            }
        });
        self.position.dist()
    }

    fn navigate_by_waypoint(&mut self) -> i32 {
        self.actions
            .clone()
            .iter()
            .for_each(|(action, param)| match action {
                'N' => self.waypoint.y -= param,
                'S' => self.waypoint.y += param,
                'E' => self.waypoint.x += param,
                'W' => self.waypoint.x -= param,
                'L' => self.waypoint.rotate(-param),
                'R' => self.waypoint.rotate(*param),
                'F' => self.position.add(Vec2 {
                    x: param * self.waypoint.x,
                    y: param * self.waypoint.y,
                }),
                _ => panic!("invalid action"),
            });
        self.position.dist()
    }
}

fn main() {
    // Parse input
    let parsing_begin = Instant::now();
    let mut ship = parse("./inputs/day12.txt").unwrap();
    let parsing_elapsed = parsing_begin.elapsed();

    // Compute solution
    let computing_begin = Instant::now();
    ship.reset(90, Vec2 { x: 0, y: 0 }, Vec2 { x: 0, y: 0 });
    let res1 = ship.navigate_by_action();
    ship.reset(0, Vec2 { x: 0, y: 0 }, Vec2 { x: 10, y: -1 });
    let res2 = ship.navigate_by_waypoint();
    let computing_elapsed = computing_begin.elapsed();

    // Print results
    println!("part1: {}", res1);
    println!("part2: {}", res2);

    // Print timings
    println!();
    println!("parsing took {:.2?}", parsing_elapsed);
    println!("computing took {:.2?}", computing_elapsed);
}
