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

impl Coord {
    const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn iter_fall_coords(self) -> impl Iterator<Item = Self> {
        [
            Coord::new(self.x, self.y + 1),     // Down
            Coord::new(self.x - 1, self.y + 1), // Down left
            Coord::new(self.x + 1, self.y + 1), // Down right
        ]
        .into_iter()
    }
}

impl FromStr for Coord {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((x_str, y_str)) = s.split_once(',') else {
            return Err(anyhow!("Malformed coordinate {:?}", s));
        };
        Ok(Self {
            x: x_str.parse()?,
            y: y_str.parse()?,
        })
    }
}

fn part_a(rocks: &HashSet<Coord>) -> Result<usize> {
    let max_y = rocks.iter().copied().map(|r| r.y).max().unwrap_or(0);
    let mut blocked = rocks.clone();

    for num_grains in 0.. {
        let mut grain = Coord::new(500, 0);
        loop {
            let Some(next_grain) = grain.iter_fall_coords().find(|c| !blocked.contains(c)) else {
                blocked.insert(grain);
                break;
            };
            if next_grain.y > max_y {
                return Ok(num_grains);
            }
            grain = next_grain;
        }
    }
    // This should never happen unless the input is malformed
    Err(anyhow!("Sand grain overflow"))
}

fn part_b(rocks: &HashSet<Coord>) -> Result<usize> {
    let max_y = rocks.iter().copied().map(|r| r.y).max().unwrap_or(0) + 2;
    let mut blocked = rocks.clone();

    for num_grains in 0.. {
        let mut grain = Coord::new(500, 0);
        if blocked.contains(&grain) {
            return Ok(num_grains);
        }
        loop {
            let Some(next_grain) = grain.iter_fall_coords().find(|c| !blocked.contains(c) && c.y < max_y) else {
                blocked.insert(grain);
                break;
            };
            grain = next_grain;
        }
    }
    // This should never happen unless the input is malformed
    Err(anyhow!("Sand grain overflow"))
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let mut rocks = HashSet::new();
    for lr in io::BufReader::new(File::open(path)?).lines() {
        let corners = lr?
            .split(" -> ")
            .map(Coord::from_str)
            .collect::<Result<Vec<_>>>()?;
        let mut corners = corners.into_iter();

        let Some(mut source) = corners.next() else {
            return Err(anyhow!("Got a line without any corners"));
        };
        for target in corners {
            if source.x == target.x {
                let step_y = (target.y - source.y).clamp(-1, 1);
                rocks.extend(
                    (0..)
                        .map(|i| Coord::new(source.x, source.y + i * step_y))
                        .take_while(|&c| c != target),
                );
            } else if source.y == target.y {
                let step_x = (target.x - source.x).clamp(-1, 1);
                rocks.extend(
                    (0..)
                        .map(|i| Coord::new(source.x + i * step_x, source.y))
                        .take_while(|&c| c != target),
                );
            } else {
                return Err(anyhow!("Diagonal line from {:?} to {:?}", source, target));
            }
            rocks.insert(target);
            source = target;
        }
    }

    Ok((part_a(&rocks)?, Some(part_b(&rocks)?)))
}
