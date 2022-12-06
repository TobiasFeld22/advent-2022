use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn main() {
    println!("Hello, world!");
    let input = fs::read_to_string("input.txt").expect("File input");

    let mut queue = VecDeque::<char>::new();
    for (index, char) in input.chars().enumerate() {
        queue.push_back(char);
        if queue.len() > 14 {
            queue.pop_front();
            if is_unique(&queue) == false {
                println!("Unique string at {}: {:?}", index + 1, queue);
                break;
            }
        }
        println!("{:?} | {}", queue, is_unique(&queue));
    }
}

fn is_unique(c: &VecDeque<char>) -> bool {
    let mut set = HashSet::<char>::new();
    if c.len() < 14 {
        return true;
    }
    for i in 0..14 {
        set.insert(*c.get(i).unwrap());
    }
    return set.len() != 14;
}
