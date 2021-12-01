use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    println!("Day 01\n------\n");
    let path = "data/day01.txt";
    let values = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read input file: {}", path))?
        .lines()
        .map(|v| Ok(v.parse::<u32>()?))
        .collect::<Result<Vec<_>>>()
        .with_context(|| "Parsing input as ints failed somewhere")?;

    // Part One
    let increase_count = values
        .windows(2)
        .fold(0, |acc, w| acc + (w[1] > w[0]) as usize);
    println!("Depths increase count: {}", increase_count);

    // Part Two
    let window_increase_count = values
        .windows(4)
        .fold(0, |acc, w| acc + (w[3] > w[0]) as usize);
    println!("Depth windows increase count: {}", window_increase_count);

    Ok(())
}
