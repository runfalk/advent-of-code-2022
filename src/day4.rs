use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::RangeInclusive;
use std::path::Path;

type Pair = (RangeInclusive<usize>, RangeInclusive<usize>);

fn parse_range(s: &str) -> Result<RangeInclusive<usize>> {
    let Some((start, end)) = s.split_once('-') else {
        return Err(anyhow!("Range doesn't contain -"));
    };
    Ok(start.parse()?..=end.parse()?)
}

fn part_a(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|(a, b)| {
            a.contains(b.start()) && a.contains(b.end())
                || b.contains(a.start()) && b.contains(a.end())
        })
        .count()
}

fn part_b(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|(a, b)| {
            a.contains(b.start())
                || a.contains(b.end())
                || b.contains(a.start())
                || b.contains(a.end())
        })
        .count()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let pairs = io::BufReader::new(file)
        .lines()
        .map(|lr| {
            let pair = lr?;
            let Some((a, b)) = pair.split_once(',') else {
            return Err(anyhow!("Pair doesn't contain a comma"));
        };

            Ok((parse_range(a)?, parse_range(b)?))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok((part_a(&pairs), Some(part_b(&pairs))))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static [Pair] = &[
        (2..=4, 6..=8),
        (2..=3, 4..=5),
        (5..=7, 7..=9),
        (2..=8, 3..=7),
        (6..=6, 4..=6),
        (2..=6, 4..=8),
    ];

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(INPUT), 2);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(INPUT), 4);
    }
}
