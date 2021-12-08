use itertools::Itertools;
use std::collections::HashMap;

type Line = (i16, i16, i16, i16);

fn find_answer(lines: &[Line], use_slanted: bool) -> usize {
    let mut grid: HashMap<(i16, i16), usize> = HashMap::new();

    for (mut x, mut y, x1, y1) in lines.iter().cloned() {
        let (dx, dy) = ((x1 - x).signum(), (y1 - y).signum());
        if dx != 0 && dy != 0 && !use_slanted {
            continue;
        }

        *grid.entry((x, y)).or_default() += 1;
        while (x, y) != (x1, y1) {
            x += dx;
            y += dy;
            *grid.entry((x, y)).or_default() += 1;
        }
    }

    grid.values().filter(|v| **v > 1).count()
}

pub fn run() -> (usize, usize) {
    let input = include_str!("data/day05.txt");
    let lines = input
        .lines()
        .filter_map(|line| {
            line.split(" -> ")
                .flat_map(|point| point.split(","))
                .filter_map(|x| x.parse::<i16>().ok())
                .collect_tuple()
        })
        .collect::<Vec<Line>>();

    (find_answer(&lines, false), find_answer(&lines, true))
}

#[test]
fn test_01() {
    let lines = vec![
        (0, 9, 5, 9),
        (8, 0, 0, 8),
        (9, 4, 3, 4),
        (2, 2, 2, 1),
        (7, 0, 7, 4),
        (6, 4, 2, 0),
        (0, 9, 2, 9),
        (3, 4, 1, 4),
        (0, 0, 8, 8),
        (5, 5, 8, 2),
    ];
    assert_eq!(find_answer(&lines, false), 5);
    assert_eq!(find_answer(&lines, true), 12);
}
