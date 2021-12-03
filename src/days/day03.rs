use anyhow::{bail, Result};

#[derive(Debug)]
struct BitCounter {
    length: usize,
    bit_counts: Vec<usize>,
    num_seen: usize,
}

impl BitCounter {
    fn new(length: usize) -> Result<Self> {
        if length > 16 {
            bail!("This implementation is limited to 16-bits wide numbers.")
        }

        Ok(Self {
            length,
            bit_counts: vec![0; length],
            num_seen: 0,
        })
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

        let mask = (1 << self.length) - 1;
        (gamma, gamma ^ mask)
    }
}

fn find_answer(report: &str) -> Result<(usize, usize)> {
    let mut lines = report.lines().peekable();
    let length = if let Some(&line) = lines.peek() {
        line.trim().chars().count()
    } else {
        bail!("Empty report!")
    };

    let mut counter = BitCounter::new(length)?;
    for line in lines {
        let line = line.trim();
        if line.chars().count() != length {
            bail!("All lines must have the same length!")
        }
        let value = u16::from_str_radix(line, 2)?;
        counter.count(value);
    }

    Ok(counter.gamma_epsilon())
}

pub fn run() -> Result<(usize, usize)> {
    let input = include_str!("data/day03.txt");
    find_answer(input)
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

    let (gamma, epsilon) = find_answer(input).unwrap();
    assert_eq!(gamma, 22);
    assert_eq!(epsilon, 9);
}
