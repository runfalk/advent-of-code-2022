use anyhow::{anyhow, Result};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coord {
    x: isize,
    y: isize,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// This represenation is kind of jank
struct Blizzard {
    origin: Coord,
    direction: Direction,
    width: isize,
    height: isize,
}

struct Map {
    walls: HashSet<Coord>,
    blizzards: Vec<Blizzard>,
    start: Coord,
    target: Coord,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn iter_moves(self) -> impl Iterator<Item = Self> {
        [
            Self::new(self.x - 1, self.y),
            Self::new(self.x, self.y - 1),
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y + 1),
            self,
        ]
        .into_iter()
    }

    fn manhattan_distance(self, other: Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

impl Blizzard {
    fn position(&self, t: usize) -> Coord {
        let delta = match self.direction {
            Direction::Up => Coord::new(0, -1),
            Direction::Down => Coord::new(0, 1),
            Direction::Left => Coord::new(-1, 0),
            Direction::Right => Coord::new(1, 0),
        };
        Coord::new(
            ((self.origin.x - 1) + delta.x * (t as isize)).rem_euclid(self.width - 2) + 1,
            ((self.origin.y - 1) + delta.y * (t as isize)).rem_euclid(self.height - 2) + 1,
        )
    }
}

impl Map {
    /// Return the earliest possible time we can be at the target
    fn earliest_arrival(&self, starting_minute: usize, start: Coord, target: Coord) -> usize {
        // Use A* to find the quickest route from start to target
        let mut to_explore = BinaryHeap::new();
        to_explore.push(Reverse((
            starting_minute + start.manhattan_distance(target),
            starting_minute,
            start,
        )));
        let mut explored = HashSet::new();

        while let Some(Reverse((_, curr_minute, pos))) = to_explore.pop() {
            if pos == target {
                return curr_minute;
            }

            let next_minute = curr_minute + 1;
            for n in pos.iter_moves().filter(|c| !self.walls.contains(c)) {
                // This could be optimized by only checking for blizzards on the same axis as the
                // position
                let would_hit_blizzard =
                    self.blizzards.iter().any(|b| b.position(next_minute) == n);
                if would_hit_blizzard {
                    continue;
                }
                if explored.insert((next_minute, n)) {
                    to_explore.push(Reverse((
                        next_minute + n.manhattan_distance(target),
                        next_minute,
                        n,
                    )));
                }
            }
        }
        // Since we can wait at the starting postion we'll run out of memory before we get here
        unreachable!();
    }

    fn try_from_str(s: &str) -> Result<Map> {
        let mut start = None;
        let mut target = None;
        let mut width = 0;
        let mut height = 0;
        let mut walls = HashSet::new();

        let mut blizzard_specs = Vec::new();
        for (line, y) in s.lines().zip(0..) {
            for (c, x) in line.chars().zip(0..) {
                let pos = Coord::new(x, y);
                match c {
                    '.' if y == 0 => {
                        start = Some(pos);
                    }
                    '.' => {
                        // The last seen "." will be the target
                        target = Some(pos);
                    }
                    '^' => {
                        blizzard_specs.push((pos, Direction::Up));
                    }
                    'v' => {
                        blizzard_specs.push((pos, Direction::Down));
                    }
                    '<' => {
                        blizzard_specs.push((pos, Direction::Left));
                    }
                    '>' => {
                        blizzard_specs.push((pos, Direction::Right));
                    }
                    '#' => {
                        width = width.max(pos.x + 1);
                        height = height.max(pos.y + 1);
                        walls.insert(pos);
                    }
                    _ => return Err(anyhow!("Unexpected character {:?} in map", c)),
                }
            }
        }

        let Some(start) = start else {
            return Err(anyhow!("Found no entrance"));
        };
        let Some(target) = target else {
            return Err(anyhow!("Found no exit"));
        };

        // Plug the hole behind the entrance and exit
        walls.insert(Coord::new(start.x, start.y - 1));
        walls.insert(Coord::new(target.x, target.y + 1));

        let blizzards = blizzard_specs
            .into_iter()
            .map(|(origin, direction)| Blizzard {
                origin,
                direction,
                width,
                height,
            })
            .collect();

        Ok(Map {
            walls,
            blizzards,
            start,
            target,
        })
    }
}

fn part_a(map: &Map) -> usize {
    map.earliest_arrival(0, map.start, map.target)
}

fn part_b(map: &Map, first_trip: usize) -> usize {
    let back_at_start = map.earliest_arrival(first_trip, map.target, map.start);
    map.earliest_arrival(back_at_start, map.start, map.target)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let mut map_str = String::new();
    File::open(path)?.read_to_string(&mut map_str)?;
    let map = Map::try_from_str(&map_str)?;

    let first_trip = part_a(&map);
    Ok((first_trip, Some(part_b(&map, first_trip))))
}

#[cfg(test)]
mod tests {
    use super::*;

    const LARGE_EXAMPLE: &'static str = concat!(
        "#.######\n",
        "#>>.<^<#\n",
        "#.<..<<#\n",
        "#>v.><>#\n",
        "#<^v^^>#\n",
        "######.#\n",
    );

    #[test]
    fn test_right_blizzard_movement() {
        let right_blizzard = Blizzard {
            origin: Coord::new(1, 1),
            direction: Direction::Right,
            width: 7,
            height: 7,
        };
        assert_eq!(right_blizzard.position(0), right_blizzard.origin);
        assert_eq!(right_blizzard.position(1), Coord::new(2, 1));
        assert_eq!(right_blizzard.position(2), Coord::new(3, 1));
        assert_eq!(right_blizzard.position(3), Coord::new(4, 1));
        assert_eq!(right_blizzard.position(4), Coord::new(5, 1));
        assert_eq!(right_blizzard.position(5), right_blizzard.origin);
    }

    #[test]
    fn test_down_blizzard_movement() {
        let down_blizzard = Blizzard {
            origin: Coord::new(4, 4),
            direction: Direction::Down,
            width: 7,
            height: 7,
        };
        assert_eq!(down_blizzard.position(0), down_blizzard.origin);
        assert_eq!(down_blizzard.position(1), Coord::new(4, 5));
        assert_eq!(down_blizzard.position(2), Coord::new(4, 1));
        assert_eq!(down_blizzard.position(3), Coord::new(4, 2));
        assert_eq!(down_blizzard.position(4), Coord::new(4, 3));
        assert_eq!(down_blizzard.position(5), down_blizzard.origin);
    }

    #[test]
    fn test_example_a() {
        let map = Map::try_from_str(LARGE_EXAMPLE).unwrap();
        assert_eq!(part_a(&map), 18);
    }

    #[test]
    fn test_example_b() {
        let map = Map::try_from_str(LARGE_EXAMPLE).unwrap();
        assert_eq!(part_b(&map, 18), 54);
    }
}
