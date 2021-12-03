use anyhow::{Context, Result};

#[derive(Debug)]
struct BitCounter<const N: usize> {
    bit_counts: [usize; N],
    num_seen: usize,
}

impl<const N: usize> BitCounter<N> {
    fn new() -> Self {
        Self {
            bit_counts: [0; N],
            num_seen: 0,
        }
    }

    // Don't want to make this generic for all integer widths.
    fn count(&mut self, value: u16) {
        for (idx, count) in self.bit_counts.iter_mut().enumerate() {
            *count += (value & (1 << idx) != 0) as usize;
        }
        self.num_seen += 1;
    }

    fn gamma_epsilon(&self) -> (usize, usize) {
        let mut gamma = 0;
        for (idx, count) in self.bit_counts.iter().enumerate() {
            if *count > self.num_seen / 2 {
                gamma += 1 << idx;
            }
        }

        let mask = (1 << N) - 1;
        (gamma, gamma ^ mask)
    }
}

fn find_answer<const N: usize>(report: &str) -> Result<(usize, usize)> {
    let mut counter = BitCounter::<N>::new();
    for line in report.lines() {
        let value = u16::from_str_radix(line.trim(), 2)
            .with_context(|| "Report diagnostic cannot be parsed as a binary number")?;
        counter.count(value);
    }

    Ok(counter.gamma_epsilon())
}

pub fn run() -> Result<(usize, usize)> {
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

    let (gamma, epsilon) = find_answer::<5>(input).unwrap();
    assert_eq!(gamma, 22);
    assert_eq!(epsilon, 9);
}
