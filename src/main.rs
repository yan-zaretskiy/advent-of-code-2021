pub mod days {
    automod::dir!(pub "src/days");
}

fn main() -> anyhow::Result<()> {
    // Day 01
    println!("Day 01\n------\n");
    let (increase_count, window_increase_count) = days::day01::run()?;
    println!("Depths increase count: {}", increase_count);
    println!("Depth windows increase count: {}\n", window_increase_count);

    // Day 02
    println!("Day 02\n------\n");
    let (pos_p1, pos_p2) = days::day02::run()?;
    println!("Part 1 answer: {}", pos_p1.x * pos_p1.y);
    println!("Part 2 answer: {}", pos_p2.x * pos_p2.y);
    Ok(())
}
