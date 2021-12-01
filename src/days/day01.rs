use anyhow::{Context, Result};

fn find_answer(values: &[u32]) -> (usize, usize) {
    // Part One
    let increase_count = values
        .windows(2)
        .fold(0, |acc, w| acc + (w[1] > w[0]) as usize);

    // Part Two
    let window_increase_count = values
        .windows(4)
        .fold(0, |acc, w| acc + (w[3] > w[0]) as usize);

    (increase_count, window_increase_count)
}

pub fn run() -> Result<(usize, usize)> {
    let path = "data/day01.txt";
    let values = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read input file: {}", path))?
        .lines()
        .map(|v| Ok(v.parse::<u32>()?))
        .collect::<Result<Vec<_>>>()
        .with_context(|| "Parsing input as ints failed somewhere")?;

    Ok(find_answer(&values))
}

#[test]
fn test_01() {
    let values = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(find_answer(values), (7, 5));
}
