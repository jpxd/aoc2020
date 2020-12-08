use std::{
    fs::File,
    io::{self, BufRead},
    time::Instant,
};

#[derive(Clone)]
enum OpCode {
    Acc,
    Jmp,
    Nop,
}

impl OpCode {
    fn from_str(op_name: &str) -> OpCode {
        match op_name {
            "acc" => OpCode::Acc,
            "jmp" => OpCode::Jmp,
            "nop" => OpCode::Nop,
            _ => panic!("bad opcode"),
        }
    }
}

#[derive(Clone)]
struct Instruction {
    op: OpCode,
    arg: i32,
    visited: bool,
}

struct Cpu {
    pc: i32,
    acc: i32,
    instructions: Vec<Instruction>,
}

impl Cpu {
    fn clone(&self) -> Cpu {
        Cpu {
            pc: self.pc,
            acc: self.acc,
            instructions: self.instructions.to_vec(),
        }
    }
    fn run_and_check_if_terminates(&mut self) -> bool {
        while let Some(current) = self.instructions.get_mut(self.pc as usize) {
            if current.visited {
                return false;
            }
            match current.op {
                OpCode::Acc => self.acc += current.arg,
                OpCode::Jmp => self.pc += current.arg - 1,
                OpCode::Nop => (),
            }
            current.visited = true;
            self.pc += 1;
        }
        true
    }
    fn change_instruction_at(&mut self, index: usize) {
        self.instructions[index].op = match self.instructions[index].op {
            OpCode::Acc => OpCode::Acc,
            OpCode::Jmp => OpCode::Nop,
            OpCode::Nop => OpCode::Jmp,
        }
    }
}

fn parse(filename: &str) -> Option<Cpu> {
    let file = File::open(filename).ok()?;
    let lines = io::BufReader::new(file).lines();
    Some(Cpu {
        pc: 0,
        acc: 0,
        instructions: lines
            .filter_map(|l| l.ok())
            .map(|l| {
                let tokens: Vec<&str> = l.split_whitespace().collect();
                Instruction {
                    op: OpCode::from_str(tokens[0]),
                    arg: tokens[1].parse().unwrap(),
                    visited: false,
                }
            })
            .collect(),
    })
}

fn main() {
    // Parse input
    let parsing_begin = Instant::now();
    let initial_cpu = parse("./inputs/day8.txt").unwrap();
    let parsing_elapsed = parsing_begin.elapsed();

    // Compute solution
    let computing_begin = Instant::now();
    let acc1: i32;
    {
        let mut cpu = initial_cpu.clone();
        cpu.run_and_check_if_terminates();
        acc1 = cpu.acc;
    }
    let mut acc2: i32 = -1;
    {
        for i in 0..initial_cpu.instructions.len() {
            let mut cpu = initial_cpu.clone();
            cpu.change_instruction_at(i);
            if cpu.run_and_check_if_terminates() {
                acc2 = cpu.acc;
                break;
            }
        }
    }
    let computing_elapsed = computing_begin.elapsed();

    // Print results
    println!("part1: {}", acc1);
    println!("part2: {}", acc2);

    // Print timings
    println!();
    println!("parsing took {:.2?}", parsing_elapsed);
    println!("computing took {:.2?}", computing_elapsed);
}
