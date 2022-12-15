use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;
use std::path::Path;

static REPORT_LINE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$")
        .unwrap()
});

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn manhattan_distance(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn try_from_report(s: &str) -> Result<(Self, Self)> {
        let Some(captures) = REPORT_LINE_RE.captures(s) else {
            return Err(anyhow!("Invalid sensor report {:?}", s));
        };
        Ok((
            Self::new(captures[1].parse()?, captures[2].parse()?),
            Self::new(captures[3].parse()?, captures[4].parse()?),
        ))
    }
}

/// Return the range of tiles covered by the given sensor at the given row y
fn coverage_at_y(sensor: &Coord, beacon: &Coord, y: isize) -> Option<RangeInclusive<isize>> {
    let distance = sensor.manhattan_distance(beacon);
    if ((sensor.y - distance)..=(sensor.y + distance)).contains(&y) {
        let spread = distance - (sensor.y - y).abs();
        Some((sensor.x - spread)..=(sensor.x + spread))
    } else {
        None
    }
}

/// Normalize the given vector of potentially overlapping by merging all adjacent and overlapping
/// ranges.
fn normalize_range_set(mut ranges: Vec<RangeInclusive<isize>>) -> Vec<RangeInclusive<isize>> {
    ranges.sort_by_key(|r| Reverse((*r.start(), *r.end())));
    let mut normalized: Vec<RangeInclusive<isize>> = Vec::new();
    while let Some(curr) = ranges.pop() {
        let Some(prev) = normalized.last_mut() else {
            normalized.push(curr);
            continue;
        };
        if curr.start() <= prev.end() {
            let extended_range = (*prev.start())..=((*curr.end()).max(*prev.end()));
            *prev = extended_range;
        } else {
            normalized.push(curr);
        }
    }
    normalized
}

fn part_a(sensors: &[(Coord, Coord)], y: isize) -> usize {
    let overlapping_coverage = sensors
        .iter()
        .filter_map(|(s, b)| coverage_at_y(s, b, y))
        .collect::<Vec<_>>();
    let num_beacons_on_row = sensors
        .iter()
        .filter_map(|(_, b)| (b.y == y).then_some(b.x))
        .collect::<HashSet<_>>()
        .len();
    let num_covered_tiles: usize = normalize_range_set(overlapping_coverage)
        .into_iter()
        .map(Iterator::count)
        .sum();
    num_covered_tiles - num_beacons_on_row
}

fn part_b(sensors: &[(Coord, Coord)], limit: isize) -> Result<isize> {
    for y in 0..=limit {
        // Save each sensors coverage of this line as a range in a vector
        let overlapping_coverage = sensors
            .iter()
            .filter_map(|(s, b)| coverage_at_y(s, b, y))
            .collect::<Vec<_>>();

        // Normalize overlapping ranges. If we have a gap within the given bounding box (limit) we
        // know this is the location for the hidden beacon
        let mut gaps = normalize_range_set(overlapping_coverage)
            .into_iter()
            .skip(1)
            .map(|r| r.start() - 1);
        if let Some(x) = gaps.find(|x| (0..=limit).contains(x)) {
            return Ok(4_000_000 * x + y);
        }
    }
    Err(anyhow!("No solution found"))
}

pub fn main(path: &Path) -> Result<(usize, Option<isize>)> {
    let sensors = io::BufReader::new(File::open(path)?)
        .lines()
        .map(|lr| Coord::try_from_report(&lr?))
        .collect::<Result<Vec<_>>>()?;
    Ok((
        part_a(&sensors, 2_000_000),
        Some(part_b(&sensors, 4_000_000)?),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<(Coord, Coord)> {
        vec![
            (Coord::new(2, 18), Coord::new(-2, 15)),
            (Coord::new(9, 16), Coord::new(10, 16)),
            (Coord::new(13, 2), Coord::new(15, 3)),
            (Coord::new(12, 14), Coord::new(10, 16)),
            (Coord::new(10, 20), Coord::new(10, 16)),
            (Coord::new(14, 17), Coord::new(10, 16)),
            (Coord::new(8, 7), Coord::new(2, 10)),
            (Coord::new(2, 0), Coord::new(2, 10)),
            (Coord::new(0, 11), Coord::new(2, 10)),
            (Coord::new(20, 14), Coord::new(25, 17)),
            (Coord::new(17, 20), Coord::new(21, 22)),
            (Coord::new(16, 7), Coord::new(15, 3)),
            (Coord::new(14, 3), Coord::new(15, 3)),
            (Coord::new(20, 1), Coord::new(15, 3)),
        ]
    }

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(Coord::new(8, 7).manhattan_distance(&Coord::new(2, 10)), 9);
    }

    #[test]
    fn test_normalize_range() {
        assert_eq!(normalize_range_set(vec![]), vec![]);
        assert_eq!(normalize_range_set(vec![1..=3, 0..=4]), vec![0..=4]);
        assert_eq!(normalize_range_set(vec![0..=3, 1..=4]), vec![0..=4]);
        assert_eq!(normalize_range_set(vec![0..=5, 1..=4]), vec![0..=5]);
        assert_eq!(normalize_range_set(vec![0..=3, 5..=9]), vec![0..=3, 5..=9]);
    }

    #[test]
    fn test_example_a() {
        assert_eq!(part_a(&example_input(), 10), 26);
    }

    #[test]
    fn test_example_b() -> Result<()> {
        assert_eq!(part_b(&example_input(), 20)?, 56_000_011);
        Ok(())
    }
}
