use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("There to be input!");

    let mut sum_items: Vec<i64> = input
        .split("\n\n")
        .map(|data| {
            data.split("\n")
                .map(|nr| {
                    return i64::from_str_radix(nr, 10).unwrap_or(0);
                })
                .fold(0 as i64, |acc, x| acc + x)
        })
        .collect();

    sum_items.sort_by(|a, b| b.cmp(a));

    println!("{}", sum_items[0]);
}
