use std::collections::VecDeque;

fn find_answer(fishes: &[u8], day: u16) -> usize {
    let mut counts: VecDeque<usize> = (0..9)
        .map(|c| fishes.iter().filter(|f| **f == c).count())
        .collect();

    (0..day).for_each(|_| {
        counts.rotate_left(1);
        counts[6] += counts[8];
    });

    counts.iter().sum()
}

pub fn run() -> (usize, usize) {
    let input = include_str!("data/day06.txt");
    let fishes = input
        .trim()
        .split(',')
        .filter_map(|s| s.parse::<u8>().ok())
        .collect::<Vec<_>>();

    (find_answer(&fishes, 80), find_answer(&fishes, 256))
}

#[test]
fn test_01() {
    let fishes = vec![3, 4, 3, 1, 2];

    assert_eq!(find_answer(&fishes, 18), 26);
    assert_eq!(find_answer(&fishes, 80), 5934);
    assert_eq!(find_answer(&fishes, 256), 26984457539);
}
