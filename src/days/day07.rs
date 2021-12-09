fn fixed_cost(crabs: &[i32], pos: i32) -> u32 {
    crabs.iter().map(|c| (*c - pos).abs() as u32).sum()
}

fn find_answer_p1(crabs: &mut [i32]) -> u32 {
    // For part 1, median is the minimizer.
    crabs.sort_unstable();

    if crabs.len() % 2 != 0 {
        let median = crabs[crabs.len() / 2];
        return fixed_cost(crabs, median);
    } else {
        let left = crabs[crabs.len() / 2 - 1];
        let right = crabs[crabs.len() / 2];
        return fixed_cost(crabs, left).min(fixed_cost(crabs, right));
    }
}

fn growing_cost(crabs: &[i32], pos: i32) -> u32 {
    crabs
        .iter()
        .map(|c| {
            let n = (*c - pos).abs() as u32;
            (n * (n + 1)) / 2
        })
        .sum()
}

fn find_answer_p2(crabs: &[i32]) -> u32 {
    // For part 2, the minimizer has to be within a fraction of the average.
    let floor_avg = (crabs.iter().map(|c| *c as u32).sum::<u32>() / crabs.len() as u32) as i32;
    growing_cost(crabs, floor_avg).min(growing_cost(crabs, floor_avg + 1))
}

pub fn run() -> (u32, u32) {
    let input = include_str!("data/day07.txt");
    let mut crabs = input
        .trim()
        .split(',')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<_>>();

    (find_answer_p1(&mut crabs), find_answer_p2(&mut crabs))
}

#[test]
fn test_01() {
    let mut crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    assert_eq!(find_answer_p1(&mut crabs), 37);
    assert_eq!(find_answer_p2(&mut crabs), 168);
}
