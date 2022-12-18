use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
}

impl Coord {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn iter_neighbors(self) -> impl Iterator<Item = Self> {
        [
            Coord::new(self.x - 1, self.y, self.z),
            Coord::new(self.x + 1, self.y, self.z),
            Coord::new(self.x, self.y - 1, self.z),
            Coord::new(self.x, self.y + 1, self.z),
            Coord::new(self.x, self.y, self.z - 1),
            Coord::new(self.x, self.y, self.z + 1),
        ]
        .into_iter()
    }
}

impl FromStr for Coord {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((x_str, rest)) = s.split_once(',') else {
            return Err(anyhow!("Cube location has no comma ({:?})", s));
        };
        let Some((y_str, z_str)) = rest.split_once(',') else {
            return Err(anyhow!("Cube location has no second comma ({:?})", s));
        };
        Ok(Self {
            x: x_str.parse()?,
            y: y_str.parse()?,
            z: z_str.parse()?,
        })
    }
}

fn part_a(cubes: &HashSet<Coord>) -> usize {
    let mut surface_tiles = 0;
    for cube in cubes.iter() {
        surface_tiles += 6 - cube
            .iter_neighbors()
            .filter(|nc| cubes.contains(nc))
            .count();
    }
    surface_tiles
}

fn part_b(cubes: &HashSet<Coord>) -> usize {
    // Find the bounding box of the set of cubes
    let (min_x, max_x) = cubes
        .iter()
        .copied()
        .map(|Coord { x, .. }| x)
        .minmax()
        .into_option()
        .unwrap_or((0, 0));
    let (min_y, max_y) = cubes
        .iter()
        .copied()
        .map(|Coord { y, .. }| y)
        .minmax()
        .into_option()
        .unwrap_or((0, 0));
    let (min_z, max_z) = cubes
        .iter()
        .copied()
        .map(|Coord { z, .. }| z)
        .minmax()
        .into_option()
        .unwrap_or((0, 0));

    let x_limit = (min_x - 1)..=(max_x + 1);
    let y_limit = (min_y - 1)..=(max_y + 1);
    let z_limit = (min_z - 1)..=(max_z + 1);

    // Perform depth first search to find all spaces with water
    let start = Coord {
        x: min_x,
        y: min_y,
        z: min_z,
    };
    let mut to_visit = vec![start];
    let mut water = [start].into_iter().collect::<HashSet<_>>();
    while let Some(c) = to_visit.pop() {
        for nc in c.iter_neighbors() {
            if !x_limit.contains(&nc.x)
                || !y_limit.contains(&nc.y)
                || !z_limit.contains(&nc.z)
                || cubes.contains(&nc)
                || water.contains(&nc)
            {
                continue;
            }
            water.insert(nc);
            to_visit.push(nc);
        }
    }

    let mut surface_tiles = 0;
    for cube in cubes.iter() {
        surface_tiles += cube
            .iter_neighbors()
            .filter(|nc| water.contains(nc))
            .count();
    }
    surface_tiles
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let cubes = io::BufReader::new(file)
        .lines()
        .map(|lr| lr?.parse())
        .collect::<Result<HashSet<Coord>>>()?;
    Ok((part_a(&cubes), Some(part_b(&cubes))))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small_example() -> HashSet<Coord> {
        [Coord { x: 1, y: 1, z: 1 }, Coord { x: 2, y: 1, z: 1 }]
            .into_iter()
            .collect()
    }

    fn large_example() -> HashSet<Coord> {
        [
            Coord { x: 2, y: 2, z: 2 },
            Coord { x: 1, y: 2, z: 2 },
            Coord { x: 3, y: 2, z: 2 },
            Coord { x: 2, y: 1, z: 2 },
            Coord { x: 2, y: 3, z: 2 },
            Coord { x: 2, y: 2, z: 1 },
            Coord { x: 2, y: 2, z: 3 },
            Coord { x: 2, y: 2, z: 4 },
            Coord { x: 2, y: 2, z: 6 },
            Coord { x: 1, y: 2, z: 5 },
            Coord { x: 3, y: 2, z: 5 },
            Coord { x: 2, y: 1, z: 5 },
            Coord { x: 2, y: 3, z: 5 },
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn test_small_example_a() {
        assert_eq!(part_a(&small_example()), 10);
    }

    #[test]
    fn test_large_example_a() {
        assert_eq!(part_a(&large_example()), 64);
    }

    #[test]
    fn test_large_example_b() {
        assert_eq!(part_b(&large_example()), 58);
    }

    #[test]
    fn test_disjoint_part_a() {
        let cubes = [
            Coord { x: 1, y: 1, z: 1 },
            Coord { x: 2, y: 1, z: 1 },
            Coord { x: 4, y: 1, z: 1 },
        ]
        .into_iter()
        .collect();
        assert_eq!(part_a(&cubes), 16);
    }

    #[test]
    fn test_two_by_two_cube_part_a() {
        let cubes = [
            Coord { x: 1, y: 1, z: 1 },
            Coord { x: 2, y: 1, z: 1 },
            Coord { x: 1, y: 2, z: 1 },
            Coord { x: 2, y: 2, z: 1 },
        ]
        .into_iter()
        .collect();
        assert_eq!(part_a(&cubes), 16);
    }
}
