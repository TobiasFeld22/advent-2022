use itertools::Itertools;
use std::{error::Error, fs, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin/"))
        .expect("Cannot read inputs")
        .filter_map(|p| p.ok()?.path().file_stem()?.to_str().map(str::to_string))
        .sorted()
        .collect::<Vec<_>>();

    let mut total_time = 0;
    for day in &data {
        let cmd = Command::new("cargo")
            .args(["run", "--release", "--bin", day])
            .output()?;
        let output = String::from_utf8(cmd.stdout)?;
        println!("Day {}:\n{}", day, output);
        total_time += extract_microseconds(&output)?;
    }
    println!("Total time: {}ms", total_time / 1000);
    Ok(())
}

fn extract_microseconds(output: &str) -> Result<usize, Box<dyn Error>> {
    let out = output.lines().last().unwrap();
    let time = if out.ends_with("ms") {
        out["Time: ".len()..out.len() - 2].parse::<usize>()? * 1000
    } else {
        out["Time: ".len()..out.len() - 3].parse::<usize>()?
    };
    Ok(time)
}
