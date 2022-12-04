mod section;

use std::fs;

use section::Section;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File input");
    let pairs: Vec<(Section, Section)> = input
        .split("\n")
        .map(|line| {
            let mut split = line.split(",");
            return (
                section::Section::parse(split.next().unwrap()),
                section::Section::parse(split.next().unwrap()),
            );
        })
        .collect();
    let mut nr = 0;

    for pair in pairs {
        let first = pair.0;
        let second = pair.1;
        if first.does_contain_other_section(second) || second.does_contain_other_section(first) {
            nr += 1;
        }
    }
    println!("{}", nr);
}
