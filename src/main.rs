pub mod days {
    automod::dir!(pub "src/days");
}

pub mod utils;

fn main() -> anyhow::Result<()> {
    // Day 01
    println!("Day 01\n------\n");
    let (increase_count, window_increase_count) = days::day01::run()?;
    println!("Part 1 answer: {}", increase_count);
    println!("Part 2 answer: {}\n", window_increase_count);

    // Day 02
    println!("Day 02\n------\n");
    let (pos_p1, pos_p2) = days::day02::run()?;
    println!("Part 1 answer: {}", pos_p1.x * pos_p1.y);
    println!("Part 2 answer: {}\n", pos_p2.x * pos_p2.y);

    // Day 03
    println!("Day 03\n------\n");
    let (gamma, epsilon, o2, co2) = days::day03::run()?;
    println!("Part 1 answer: {}", gamma as usize * epsilon as usize);
    println!("Part 2 answer: {}\n", o2 as usize * co2 as usize);

    // Day 04
    println!("Day 04\n------\n");
    let score = days::day04::run();
    println!("Part 1 answer: {}", score.0);
    println!("Part 2 answer: {}\n", score.1);

    // Day 05
    println!("Day 05\n------\n");
    let points = days::day05::run();
    println!("Part 1 answer: {}", points.0);
    println!("Part 2 answer: {}", points.1);

    Ok(())
}
