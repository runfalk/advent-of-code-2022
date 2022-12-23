use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Direction {
    fn try_move(self, elf: Coord, elves: &HashSet<Coord>) -> Option<Coord> {
        let (delta_left, delta_front, delta_right) = match self {
            Self::North => (Coord::new(-1, -1), Coord::new(0, -1), Coord::new(1, -1)),
            Self::South => (Coord::new(1, 1), Coord::new(0, 1), Coord::new(-1, 1)),
            Self::West => (Coord::new(-1, -1), Coord::new(-1, 0), Coord::new(-1, 1)),
            Self::East => (Coord::new(1, -1), Coord::new(1, 0), Coord::new(1, 1)),
        };
        let left = Coord::new(elf.x + delta_left.x, elf.y + delta_left.y);
        let front = Coord::new(elf.x + delta_front.x, elf.y + delta_front.y);
        let right = Coord::new(elf.x + delta_right.x, elf.y + delta_right.y);
        if !elves.contains(&left) && !elves.contains(&front) && !elves.contains(&right) {
            Some(front)
        } else {
            None
        }
    }
}

/// Cycle through the different directions in the right order
impl Iterator for Direction {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = *self;
        *self = match self {
            Self::North => Self::South,
            Self::South => Self::West,
            Self::West => Self::East,
            Self::East => Self::North,
        };
        Some(curr)
    }
}

fn find_elves(s: &str) -> Result<HashSet<Coord>> {
    let mut map = HashSet::new();
    for (y, line) in s.lines().enumerate() {
        let y: isize = y.try_into()?;
        for (x, c) in line.chars().enumerate() {
            let x: isize = x.try_into()?;
            match c {
                '.' => {}
                '#' => {
                    map.insert(Coord { x, y });
                }
                _ => return Err(anyhow!("Unexpected character {:?} in map", c)),
            }
        }
    }
    Ok(map)
}

fn process_round(elves: HashSet<Coord>, starting_direction: Direction) -> HashSet<Coord> {
    let mut cell_wantedness: HashMap<Coord, usize> = HashMap::new();
    let mut wanted_moves = Vec::new();

    for elf in elves.iter().copied() {
        let mut target = elf;
        let should_move = !Direction::North
            .take(4)
            .all(|d| d.try_move(elf, &elves).is_some());
        if should_move {
            for dir in starting_direction.take(4) {
                if let Some(next_position) = dir.try_move(elf, &elves) {
                    target = next_position;
                    break;
                }
            }
        }

        *cell_wantedness.entry(target).or_default() += 1;
        wanted_moves.push((elf, target));
    }
    wanted_moves
        .into_iter()
        .map(|(curr, wanted)| {
            if cell_wantedness.get(&wanted).copied().unwrap() == 1 {
                wanted
            } else {
                curr
            }
        })
        .collect()
}

fn part_a(mut elves: HashSet<Coord>) -> isize {
    for (starting_direction, _) in Direction::North.zip(0..10) {
        elves = process_round(elves, starting_direction);
    }

    // Find bounding box and calculate the number of empty ground tiles
    let (min_x, max_x) = elves.iter().map(|c| c.x).minmax().into_option().unwrap();
    let (min_y, max_y) = elves.iter().map(|c| c.y).minmax().into_option().unwrap();
    (max_x - min_x + 1) * (max_y - min_y + 1) - (elves.len() as isize)
}

fn part_b(mut elves: HashSet<Coord>) -> usize {
    for (starting_direction, round) in Direction::North.zip(1..) {
        let next_elves = process_round(elves.clone(), starting_direction);
        if elves == next_elves {
            return round;
        }
        elves = next_elves;
    }
    // Unreachable because we'd get a usize overflow before getting here
    unreachable!();
}

pub fn main(path: &Path) -> Result<(isize, Option<usize>)> {
    let mut map_str = String::new();
    File::open(path)?.read_to_string(&mut map_str)?;
    let elves = find_elves(&map_str)?;
    Ok((part_a(elves.clone()), Some(part_b(elves))))
}

#[cfg(test)]
mod tests {
    use super::*;

    const LARGE_EXAMPLE: &'static str = concat!(
        "..............\n",
        "..............\n",
        ".......#......\n",
        ".....###.#....\n",
        "...#...#.#....\n",
        "....#...##....\n",
        "...#.###......\n",
        "...##.#.##....\n",
        "....#..#......\n",
        "..............\n",
        "..............\n",
        "..............\n",
    );

    #[test]
    fn test_example_a() -> Result<()> {
        assert_eq!(part_a(find_elves(LARGE_EXAMPLE)?), 110);
        Ok(())
    }

    #[test]
    fn test_example_b() -> Result<()> {
        assert_eq!(part_b(find_elves(LARGE_EXAMPLE)?), 20);
        Ok(())
    }
}
