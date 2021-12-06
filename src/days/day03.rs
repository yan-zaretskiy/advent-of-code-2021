use anyhow::{Context, Result};

use crate::utils::partition;

#[inline]
fn has_one_bit_at(value: u16, pos: usize) -> bool {
    value & (1 << pos) != 0
}

fn make_gamma<const N: usize>(values: &[u16]) -> u16 {
    // count 1-bits in each position
    let mut bit_counts = [0; N];
    for v in values {
        for (idx, count) in bit_counts.iter_mut().enumerate() {
            *count += has_one_bit_at(*v, idx) as usize;
        }
    }

    // put 1-bits where they are most common
    let gamma = bit_counts
        .iter()
        .enumerate()
        .filter(|(_, &count)| 2 * count >= values.len())
        .fold(0, |acc, (idx, _)| acc + (1 << idx));

    gamma
}

fn most_common_bit_at_pos(values: &[u16], pos: usize) -> bool {
    let one_count = values
        .iter()
        .fold(0, |acc, v| acc + has_one_bit_at(*v, pos) as usize);

    2 * one_count >= values.len()
}

fn partiton_by_common_bit(values: &mut [u16], pos: usize) -> (&mut [u16], &mut [u16]) {
    if most_common_bit_at_pos(values, pos) {
        partition(values, |value| has_one_bit_at(*value, pos))
    } else {
        partition(values, |value| !has_one_bit_at(*value, pos))
    }
}

fn find_answer<const N: usize>(report: &str) -> Result<(u16, u16, u16, u16)> {
    let mut values = report
        .lines()
        .map(|l| Ok(u16::from_str_radix(l.trim(), 2)?))
        .collect::<Result<Vec<_>>>()
        .with_context(|| "Report diagnostic cannot be parsed as a binary number")?;

    // Part One
    let gamma = make_gamma::<N>(&values);
    let mask = (1 << N) - 1;
    let epsilon = gamma ^ mask;

    // Part 2
    let mut pos = N - 1;
    let (mut left, mut right) = partiton_by_common_bit(&mut values, pos);

    while left.len() > 1 || right.len() > 1 {
        assert!(pos > 0);
        pos -= 1;
        if left.len() > 1 {
            left = partiton_by_common_bit(left, pos).0;
        }
        if right.len() > 1 {
            right = partiton_by_common_bit(right, pos).1;
        }
    }

    Ok((gamma, epsilon, values[0], *values.last().unwrap()))
}

pub fn run() -> Result<(u16, u16, u16, u16)> {
    let input = include_str!("data/day03.txt");
    find_answer::<12>(input)
}

#[test]
fn test_01() {
    let input = "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";

    let (gamma, epsilon, o2, co2) = find_answer::<5>(input).unwrap();
    assert_eq!(gamma, 22);
    assert_eq!(epsilon, 9);
    assert_eq!(o2, 23);
    assert_eq!(co2, 10);
}
