use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};
use itertools::Itertools;

#[derive(Debug, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub aim: isize,
}

#[derive(Debug)]
enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl Position {
    fn apply_command_p1(&mut self, command: &Command) {
        match command {
            Command::Down(dist) => self.y += dist,
            Command::Up(dist) => self.y -= dist,
            Command::Forward(dist) => self.x += dist,
        }
    }

    fn apply_command_p2(&mut self, command: &Command) {
        match command {
            Command::Down(dist) => self.aim += *dist as isize,
            Command::Up(dist) => self.aim -= *dist as isize,
            Command::Forward(dist) => {
                self.x += dist;
                self.y += (self.aim * *dist as isize) as usize;
            }
        }
    }
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s
            .trim()
            .split(' ')
            .collect_tuple::<(&str, &str)>()
            .ok_or(Error::msg("Command line must have two words"))?;
        let dist = dist
            .parse::<usize>()
            .with_context(|| format!("Not an int: {}", dist))?;

        match dir {
            "forward" => Ok(Command::Forward(dist)),
            "down" => Ok(Command::Down(dist)),
            "up" => Ok(Command::Up(dist)),
            _ => Err(anyhow!("Illegal direction: {}", dir)),
        }
    }
}

fn find_answer(commands: &str) -> Result<(Position, Position)> {
    let mut pos_p1 = Position::default();
    let mut pos_p2 = Position::default();
    for line in commands.lines() {
        let c = Command::from_str(line)?;
        pos_p1.apply_command_p1(&c);
        pos_p2.apply_command_p2(&c);
    }
    Ok((pos_p1, pos_p2))
}

pub fn run() -> Result<(Position, Position)> {
    let input = include_str!("data/day02.txt");
    find_answer(input)
}

#[test]
fn test_01() {
    let input = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    let (pos_p1, pos_p2) = find_answer(input).unwrap();
    assert_eq!(pos_p1.x, 15);
    assert_eq!(pos_p1.y, 10);

    assert_eq!(pos_p2.x, 15);
    assert_eq!(pos_p2.y, 60);
}
