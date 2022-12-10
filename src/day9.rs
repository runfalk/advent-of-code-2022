use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Up(isize),
    Right(isize),
    Down(isize),
    Left(isize),
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn iter_moves(self, m: Move) -> impl Iterator<Item = Self> {
        let (delta, count) = match m {
            Move::Up(c) => (Coord::new(0, -1), c),
            Move::Right(c) => (Coord::new(1, 0), c),
            Move::Down(c) => (Coord::new(0, 1), c),
            Move::Left(c) => (Coord::new(-1, 0), c),
        };
        (1..=count).map(move |step| Coord::new(self.x + step * delta.x, self.y + step * delta.y))
    }
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some(("U", c)) => Ok(Self::Up(c.parse()?)),
            Some(("R", c)) => Ok(Self::Right(c.parse()?)),
            Some(("D", c)) => Ok(Self::Down(c.parse()?)),
            Some(("L", c)) => Ok(Self::Left(c.parse()?)),
            _ => Err(anyhow!("Invalid move instruction ({})", s)),
        }
    }
}

fn num_tail_visits<const N: usize>(moves: &[Move]) -> usize {
    let mut tail_visited = HashSet::new();
    let mut rope = [Coord::default(); N];

    for move_instruction in moves.iter().copied() {
        for m in rope[0].iter_moves(move_instruction) {
            rope[0] = m;

            for i in 1..rope.len() {
                let prev_knot = rope[i - 1];
                let mut knot = rope[i];

                if (knot.x - prev_knot.x).abs() > 1 || (knot.y - prev_knot.y).abs() > 1 {
                    knot.x = if (knot.x - prev_knot.x).abs() > 1 {
                        knot.x.clamp(prev_knot.x - 1, prev_knot.x + 1)
                    } else {
                        prev_knot.x
                    };
                    knot.y = if (knot.y - prev_knot.y).abs() > 1 {
                        knot.y.clamp(prev_knot.y - 1, prev_knot.y + 1)
                    } else {
                        prev_knot.y
                    };
                }
                rope[i] = knot;
            }
            tail_visited.insert(*rope.last().unwrap());
        }
    }
    tail_visited.len()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let moves = io::BufReader::new(file)
        .lines()
        .map(|lr| lr?.parse())
        .collect::<Result<Vec<Move>>>()?;

    Ok((
        num_tail_visits::<2>(&moves),
        Some(num_tail_visits::<10>(&moves)),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small_example() -> Vec<Move> {
        ["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"]
            .into_iter()
            .map(FromStr::from_str)
            .collect::<Result<Vec<_>>>()
            .unwrap()
    }

    #[test]
    fn test_example_a() {
        assert_eq!(num_tail_visits::<2>(&small_example()), 13);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(num_tail_visits::<10>(&small_example()), 1);
    }

    #[test]
    fn test_example_b_large() {
        let large_example = ["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"]
            .into_iter()
            .map(FromStr::from_str)
            .collect::<Result<Vec<Move>>>()
            .unwrap();
        assert_eq!(num_tail_visits::<10>(&large_example), 36);
    }
}
