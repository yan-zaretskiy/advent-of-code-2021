type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let values = std::fs::read_to_string("data/day01.txt")?
        .lines()
        .map(|v| Ok(v.parse::<i32>()?))
        .collect::<Result<Vec<_>>>()?;

    // Part One
    let increase_count = values
        .windows(2)
        .fold(0, |acc, w| acc + (w[1] > w[0]) as i32);
    println!("Depths increase count: {}", increase_count);

    // Part Two
    let window_increase_count = values
        .windows(4)
        .fold(0, |acc, w| acc + (w[3] > w[0]) as i32);
    println!("Depth windows increase count: {}", window_increase_count);

    Ok(())
}
