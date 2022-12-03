use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_line(line: &str) -> Result<Vec<usize>> {
    line.chars()
        .map(|c| match c {
            'a'..='z' => Ok(usize::from(u8::try_from(c)?) - 97 + 1),
            'A'..='Z' => Ok(usize::from(u8::try_from(c)?) - 65 + 27),
            _ => Err(anyhow!("Input contains invalid item {}", c)),
        })
        .collect()
}

fn part_a(rucksacks: &[Vec<usize>]) -> Result<usize> {
    let mut sum = 0;
    for r in rucksacks {
        if r.len() % 2 == 1 {
            return Err(anyhow!("Rucksack does not have an even number of elements"));
        }
        let a: HashSet<usize> = r.iter().copied().take(r.len() / 2).collect::<HashSet<_>>();
        let b: HashSet<usize> = r.iter().copied().skip(r.len() / 2).collect::<HashSet<_>>();
        sum += a.intersection(&b).sum::<usize>();
    }
    Ok(sum)
}

fn part_b(rucksacks: &[Vec<usize>]) -> Result<usize> {
    let mut sum = 0;
    for triplets in rucksacks.chunks(3) {
        let a: HashSet<usize> = triplets[0].iter().copied().collect();
        let b: HashSet<usize> = triplets[1].iter().copied().collect();
        let c: HashSet<usize> = triplets[2].iter().copied().collect();
        sum += a
            .intersection(&b.intersection(&c).copied().collect())
            .sum::<usize>();
    }
    Ok(sum)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let rucksacks = io::BufReader::new(file)
        .lines()
        .map(|lr| parse_line(&lr?))
        .collect::<Result<Vec<_>>>()?;

    Ok((part_a(&rucksacks)?, Some(part_b(&rucksacks)?)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static [&'static str] = &[
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];

    #[test]
    fn test_example_a() -> Result<()> {
        let rucksacks = INPUT
            .iter()
            .map(|l| parse_line(l))
            .collect::<Result<Vec<_>>>()?;
        assert_eq!(part_a(&rucksacks)?, 157);
        Ok(())
    }

    #[test]
    fn test_example_b() -> Result<()> {
        let rucksacks = INPUT
            .iter()
            .map(|l| parse_line(l))
            .collect::<Result<Vec<_>>>()?;
        assert_eq!(part_b(&rucksacks)?, 70);
        Ok(())
    }
}
