mod node;

use std::{collections::HashMap, fs};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CDREGEX: Regex = Regex::new(r"\$ cd (?P<path>/|\w+|\.\.)").unwrap();
    static ref RESULTREGEX: Regex = Regex::new(r"(?P<length>\d+) (?P<file>(\w|\.)+)").unwrap();
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input to be here");

    let lines = input.split("\n");
    let mut path: Vec<String> = Vec::new();
    let mut directory_size: HashMap<String, u32> = HashMap::new();

    for line in lines {
        println!("{:?}", path);

        if line.starts_with("$") {
            if line.starts_with("$ ls") {
                continue;
            }
            parse_cd(line, &mut path, &mut directory_size);
        } else if !line.starts_with("dir") {
            parse_result(line, &mut path, &mut directory_size);
        }
    }
    let totals: u32 = directory_size.values().filter(|&size| *size < 100000).sum();
    println!("{}", totals);
}

fn parse_cd(line: &str, path: &mut Vec<String>, dir_sizes: &mut HashMap<String, u32>) {
    let captures = CDREGEX.captures(line);
    let filename = captures.unwrap().name("path").unwrap().as_str();

    if filename == ".." {
        path.pop();
    } else {
        path.push(filename.to_string());
        if !dir_sizes.contains_key(&path.join("/")) {
            dir_sizes.insert(path.join("/"), 0);
        }
    }
}

fn parse_result(line: &str, path: &mut Vec<String>, dir_sizes: &mut HashMap<String, u32>) {
    println!("{}", line);
    let captures = RESULTREGEX.captures(line).unwrap();
    let amount = captures.name("length").unwrap();
    // let filename = captures.name("file").unwrap();

    let amount = amount.as_str().parse::<u32>().unwrap();

    let mut update_list: Vec<String> = Vec::new();
    for dir in path {
        update_list.push(dir.to_string());
        let update_path = update_list.join("/");
        dir_sizes
            .entry(update_path)
            .and_modify(|size| *size += amount);
    }
}
