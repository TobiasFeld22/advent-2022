use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("file input");
    let mut instructions = Vec::<Instruction>::new();
    let mut cycle_instructions = HashMap::<i32, Instruction>::new();
    let mut cycle = 0;
    input.lines().for_each(|line| {
        let mut split = line.split(" ");
        let instruction = split.next().unwrap();
        if instruction == "noop" {
            cycle += 1;
            cycle_instructions.insert(cycle, Instruction::Noop(cycle));

            return instructions.push(Instruction::Noop(cycle));
        } else if instruction == "addx" {
            let value = split.next().unwrap();
            let digits = value.parse::<i32>().unwrap();
            cycle += 2;
            cycle_instructions.insert(cycle, Instruction::Add(cycle, digits));
            return instructions.push(Instruction::Add(cycle, digits));
        }
    });
    let mut register_x = 1;
    let max = cycle_instructions.keys().max().unwrap();
    let mut sum = 0;
    for i in 0..*max {
        let id = i + 1;

        let entry = cycle_instructions.get(&i);
        match entry {
            Some(instruction) => match instruction {
                Instruction::Add(_, v) => register_x += v,
                Instruction::Noop(_) => (),
            },
            None => (),
        }
        println!("[{}] {}", (i + 1), register_x);
        if id == 20 || id == 60 || id == 100 || id == 140 || id == 180 || id == 220 {
            println!("[Sum_here, {}] {}", id, id * register_x);
            sum += id * register_x;
        }
    }
    println!("[Sum] {}", sum);
}

#[derive(Debug)]
enum Instruction {
    Add(i32, i32),
    Noop(i32),
}
