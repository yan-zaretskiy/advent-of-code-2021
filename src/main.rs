pub mod days {
    automod::dir!(pub "src/days");
}

fn main() -> anyhow::Result<()> {
    // Day 01
    println!("Day 01\n------\n");
    let (increase_count, window_increase_count) = days::day01::run()?;
    println!("Depths increase count: {}", increase_count);
    println!("Depth windows increase count: {}", window_increase_count);

    Ok(())
}
