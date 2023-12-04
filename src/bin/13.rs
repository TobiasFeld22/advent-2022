use std::cmp::Ordering;

use itertools::Itertools;
use serde_json::{json, Value};

#[advent_2022::aoc(13)]
fn main(input: &str) -> (usize, usize) {
    let mut packets = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .collect_vec();
    let pairs = packets.chunks(2).map(|c| c.to_vec()).collect_vec();

    let total = pairs
        .iter()
        .map(|p| is_valid(&p[0], &p[1]))
        .enumerate()
        .filter(|(_, p)| p.is_some() && matches!(p.unwrap(), Ordering::Less))
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    let keys = [json!([2]), json!([6])];
    packets.extend(keys.clone());

    packets.sort_by(|a, b| is_valid(a, b).unwrap());

    let p2 = keys
        .iter()
        .map(|k| packets.iter().position(|p| p.eq(k)).unwrap() + 1)
        .reduce(|a, b| a * b)
        .unwrap();

    return (total, p2);
}

fn is_valid(a: &Value, b: &Value) -> Option<Ordering> {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => match a.as_u64().cmp(&b.as_u64()) {
            Ordering::Equal => None,
            order => Some(order),
        },
        (Value::Array(a), Value::Array(b)) => {
            if a.is_empty() || b.is_empty() {
                match a.len().cmp(&b.len()) {
                    Ordering::Equal => None,
                    order => Some(order),
                }
            } else if let Some(v) = is_valid(&a[0], &b[0]) {
                Some(v)
            } else {
                is_valid(&json!(a[1..]), &json!(b[1..]))
            }
        }
        (Value::Number(a), Value::Array(b)) => is_valid(&json!(vec![a]), &json!(b)),
        (Value::Array(a), Value::Number(b)) => is_valid(&json!(a), &json!(vec![b])),
        _ => Some(Ordering::Greater),
    }
}
