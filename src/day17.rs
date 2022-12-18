use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Minus,
    Plus,
    L,
    I,
    Cube,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Rock {
    fn cycle() -> impl Iterator<Item = Self> {
        [Self::Minus, Self::Plus, Self::L, Self::I, Self::Cube]
            .into_iter()
            .cycle()
    }

    fn width(self) -> usize {
        match self {
            Self::Minus => 4,
            Self::Plus => 3,
            Self::L => 3,
            Self::I => 1,
            Self::Cube => 2,
        }
    }

    fn height(self) -> usize {
        match self {
            Self::Minus => 1,
            Self::Plus => 3,
            Self::L => 3,
            Self::I => 4,
            Self::Cube => 2,
        }
    }

    fn shift_x(self, direction: Direction, x: usize) -> usize {
        match direction {
            Direction::Left => x.saturating_sub(1),
            Direction::Right => (x + 1).min(7 - self.width()),
        }
    }

    fn shape(self, x: usize, y: usize) -> HashSet<(usize, usize)> {
        match self {
            Self::Minus => (0..=3).map(|dx| (x + dx, y)).collect(),
            Self::Plus => (0..=2)
                .map(|dx| (x + dx, y + 1))
                .chain(vec![(x + 1, y), (x + 1, y + 2)].into_iter())
                .collect(),
            Self::L => (0..=2)
                .map(|dx| (x + dx, y))
                .chain((1..=2).map(|dy| (x + 2, y + dy)))
                .collect(),
            Self::I => (0..=3).map(|dy| (x, y + dy)).collect(),
            Self::Cube => (0..=1)
                .flat_map(|dy| (0..=1).map(move |dx| (x + dx, y + dy)))
                .collect(),
        }
    }

    fn overlaps(self, stationary_rocks: &HashSet<(usize, usize)>, x: usize, y: usize) -> bool {
        !stationary_rocks.is_disjoint(&self.shape(x, y))
    }

    fn is_supported(self, stationary_rocks: &HashSet<(usize, usize)>, x: usize, y: usize) -> bool {
        if y == 0 {
            return true;
        }
        self.overlaps(stationary_rocks, x, y - 1)
    }
}

fn part_a(jet_pattern: &[Direction]) -> usize {
    let mut tower_height = 0;
    let mut stationary_rocks = HashSet::new();
    let mut wind_direction = jet_pattern.iter().cycle().copied();
    for falling_rock in Rock::cycle().take(2022) {
        // Spawn the rock at the corect position
        let mut x = 2;
        let mut y = tower_height + 3;

        // Let the rock fall until it is stationary
        for wind in wind_direction.by_ref() {
            // Try to move the rock according to the wind. The move doesn't happen if the rock
            // would make the rock collide with a stationary rock
            let shifted_x = falling_rock.shift_x(wind, x);
            if !falling_rock.overlaps(&stationary_rocks, shifted_x, y) {
                x = shifted_x;
            }

            // Stop moving the piece if it is resting on a stationary rock
            if falling_rock.is_supported(&stationary_rocks, x, y) {
                tower_height = tower_height.max(y + falling_rock.height());
                stationary_rocks.extend(falling_rock.shape(x, y));
                break;
            }
            y -= 1;
        }
    }
    tower_height
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let mut buf = String::new();
    File::open(path)?.read_to_string(&mut buf)?;
    let jet_pattern = buf
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(anyhow!("Invalid character in jet pattern {:?}", c)),
        })
        .collect::<Result<Vec<Direction>>>()?;
    Ok((part_a(&jet_pattern), None))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_jet_pattern() -> Vec<Direction> {
        ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
            .chars()
            .into_iter()
            .map(|c| match c {
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => unreachable!(),
            })
            .collect()
    }

    #[test]
    fn test_example_a() {
        assert_eq!(part_a(&example_jet_pattern()), 3068);
    }
}
