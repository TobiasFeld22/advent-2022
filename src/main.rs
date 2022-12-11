mod instruction;

use std::{cmp, collections::HashSet, fs};

use instruction::{Direction, Instruction};

fn main() {
    let mut visited_locations = HashSet::<(i32, i32)>::new();

    let input = fs::read_to_string("input.txt").expect("file input");
    let mut tail_location: (i32, i32) = (0, 0);
    let mut head_location: (i32, i32) = (0, 0);
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| -> Instruction {
            let mut split = line.split(" ");

            Instruction(
                Direction::from(split.next().unwrap()),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();

    visited_locations.insert((0, 0)); // starting point

    for instruction in instructions {
        for _ in 0..instruction.1 {
            visited_locations.insert(tail_location);
            match instruction.0 {
                Direction::LEFT => head_location.0 -= 1,
                Direction::RIGHT => head_location.0 += 1,
                Direction::UP => head_location.1 -= 1,
                Direction::DOWN => head_location.1 += 1,
            }
            let delta = get_delta(head_location, tail_location);
            if delta.0.abs() == 2 || delta.1.abs() == 2 {
                tail_location.0 += delta.0.signum();
                tail_location.1 += delta.1.signum();
            }
            visited_locations.insert(tail_location);
        }
    }
    println!("{}", visited_locations.len());
}

fn get_delta(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    return ((a.0 - b.0), (a.1 - b.1));
}
