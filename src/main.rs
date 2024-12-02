use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let string = fs::read_to_string(Path::new("./src/input/day-01.in"))?;

    part1(string.clone())?;
    part2(string.clone())?;
    Ok(())
}

fn part1(input: String) -> Result<()> {
    let values = input.split("\n").map(|s| s.split("   ").map(|s| s.parse::<i32>()));

    let mut left = Vec::new();
    let mut right = Vec::new();
    for mut x in values {
        left.push(x.next().unwrap()?);
        right.push(x.next().unwrap()?);
    }

    left.sort();
    right.sort();

    let mut total = 0;
    for i in 0..left.len() {
        total += left[i].abs_diff(right[i]);
    }

    println!("Part 1: {:?}", total);
    Ok(())
}

fn part2(input: String) -> Result<()> {
    let values = input.split("\n").map(|s| s.split("   ").map(|s| s.parse::<i32>()));

    let mut left = Vec::new();
    let mut right: HashMap<i32, i32> = HashMap::new();
    for mut x in values {
        left.push(x.next().unwrap()?);
        right.entry(x.next().unwrap()?).and_modify(|v| *v += 1).or_insert(1);
    }

    let total: i32 = left.iter().map(|x| x * right.get(x).unwrap_or(&0)).sum();
    println!("Part 2: {:?}", total);

    Ok(())
}