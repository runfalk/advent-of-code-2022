use anyhow::{anyhow, Result};
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

fn compute_all_x(ops: &[Op]) -> Vec<isize> {
    let mut x = vec![1];
    for op in ops {
        let cx = x.last().copied().unwrap();
        match op {
            Op::Noop => x.push(cx),
            Op::Addx(n) => {
                x.push(cx);
                x.push(cx + n);
            }
        }
    }
    x
}

fn part_a(ops: &[Op]) -> isize {
    let key_cycles = [20, 60, 100, 140, 180, 220];
    let x = compute_all_x(ops);
    key_cycles
        .into_iter()
        .map(|c| (c as isize) * x[c - 1])
        .sum()
}

fn part_b(ops: &[Op]) -> String {
    const WIDTH: usize = 40;
    let mut crt = [false; WIDTH * 6];
    for ((cycle, x), pixel) in (0..WIDTH)
        .cycle()
        .zip(compute_all_x(ops))
        .zip(crt.iter_mut())
    {
        *pixel = (x - 1..=x + 1).contains(&(cycle as isize));
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
