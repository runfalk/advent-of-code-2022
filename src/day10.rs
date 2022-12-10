use anyhow::{anyhow, Result};
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Op {
    Noop,
    Addx(isize),
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(term) = s.strip_prefix("addx ") {
            Ok(Self::Addx(term.parse()?))
        } else if s == "noop" {
            Ok(Self::Noop)
        } else {
            Err(anyhow!("Unknown instruction {:?}", s))
        }
    }
}

fn part_a(ops: &[Op]) -> isize {
    let mut x = 1;
    let mut cycle = 0;
    let mut output = 0;
    let mut key_cycles = vec![220, 180, 140, 100, 60, 20];

    for op in ops {
        let (next_x, next_cycle) = match op {
            Op::Noop => (x, cycle + 1),
            Op::Addx(n) => (x + n, cycle + 2),
        };

        let Some(key_cycle) = key_cycles.last().copied() else {
            break
        };
        if next_cycle >= key_cycle {
            output += key_cycle * x;
            key_cycles.pop();
        }

        x = next_x;
        cycle = next_cycle;
    }
    output
}

fn part_b(ops: &[Op]) -> String {
    const WIDTH: usize = 40;
    let mut crt = [false; WIDTH * 6];
    let mut cycle = 0;
    let mut x = 1;

    for op in ops {
        match op {
            Op::Noop => {
                crt[cycle] = (x - 1..=x + 1).contains(&(cycle % WIDTH).try_into().unwrap());
                cycle += 1;
            }
            Op::Addx(n) => {
                crt[cycle] = (x - 1..=x + 1).contains(&(cycle % WIDTH).try_into().unwrap());
                cycle += 1;
                crt[cycle] = (x - 1..=x + 1).contains(&(cycle % WIDTH).try_into().unwrap());
                cycle += 1;
                x += n;
            }
        };
    }

    crt.chunks_exact(WIDTH)
        .map(|line| {
            line.iter()
                .copied()
                .map(|p| if p { '#' } else { ' ' })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn main(path: &Path) -> Result<(isize, Option<String>)> {
    let file = File::open(path)?;
    let ops = io::BufReader::new(file)
        .lines()
        .map(|lr| lr?.parse())
        .collect::<Result<Vec<Op>>>()?;

    Ok((part_a(&ops), Some(part_b(&ops))))
}
