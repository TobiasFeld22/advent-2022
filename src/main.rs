use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt");

    let mut area = HashMap::<(u32, u32), u32>::new();

    let mut row: u32 = 0;
    let mut col: u32 = 0;

    input.lines().for_each(|line| {
        line.chars()
            .map(|char| char.to_digit(10).unwrap())
            .for_each(|d| {
                area.insert((row, col), d);
                col += 1;
            });
        row += 1;
        col = 0;
    });

    let area_size: u32 = *area
        .iter()
        .map(|(k, _v)| k)
        .max_by(|a, b| a.0.cmp(&b.0))
        .map(|(row, _col)| row)
        .unwrap();

    let mut count = 0;
    for row in 0..=area_size {
        for col in 0..=area_size {
            if is_visible(&area, (row, col), area_size) {
                count += 1;
            }
        }
    }
    println!("count {}", count)
}

fn is_visible(area: &HashMap<(u32, u32), u32>, index: (u32, u32), area_size: u32) -> bool {
    let (row, col) = index;

    if row == 0 || row == area_size || col == 0 || col == area_size {
        return true;
    }
    let height = area[&index];
    let mut max_other = 0;
    for c in 0..col {
        let other_height = area[&(row, c)];
        if other_height > max_other {
            max_other = other_height;
        }
    }
    if max_other < height {
        return true;
    }
    max_other = 0;
    for c in (col + 1)..=area_size {
        let other_height = area[&(row, c)];
        if other_height > max_other {
            max_other = other_height;
        }
    }
    if max_other < height {
        return true;
    }
    max_other = 0;
    for r in 0..row {
        let other_height = area[&(r, col)];
        if other_height > max_other {
            max_other = other_height;
        }
    }
    if max_other < height {
        return true;
    }
    max_other = 0;
    for r in (row + 1)..=area_size {
        let other_height = area[&(r, col)];
        if other_height > max_other {
            max_other = other_height;
        }
    }
    if max_other < height {
        return true;
    }
    return false;
}
