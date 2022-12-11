mod instruction;

use std::{cmp, collections::HashSet, fs};

use instruction::{Direction, Instruction};

fn main() {
    let mut visited_locations = HashSet::<(i32, i32)>::new();

    let input = fs::read_to_string("input.txt").expect("file input");
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
    let mut rope = vec![(0, 0); 10];
    for instruction in instructions {
        for _ in 0..instruction.1 {
            match instruction.0 {
                Direction::LEFT => rope[0].0 -= 1,
                Direction::RIGHT => rope[0].0 += 1,
                Direction::UP => rope[0].1 -= 1,
                Direction::DOWN => rope[0].1 += 1,
            }
            for i in 1..rope.len() {
                let delta = get_delta(rope[i - 1], rope[i]);
                if delta.0.abs() == 2 || delta.1.abs() == 2 {
                    rope[i].0 += delta.0.signum();
                    rope[i].1 += delta.1.signum();
                }
                visited_locations.insert(*rope.last().unwrap());
            }
        }
    }
    println!("{}", visited_locations.len());
}

fn get_delta(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    return ((a.0 - b.0), (a.1 - b.1));
}
