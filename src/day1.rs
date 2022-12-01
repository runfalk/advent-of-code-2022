use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let calories = io::BufReader::new(file)
        .lines()
        .map(|lr| Ok(lr?.parse::<usize>().ok()))
        .collect::<Result<Vec<Option<usize>>>>()?;

    let mut calories_by_elf = vec![0];
    for item_calories in calories.iter().copied() {
        match item_calories {
            Some(c) => *calories_by_elf.last_mut().unwrap() += c,
            None => calories_by_elf.push(0),
        }
    }

    calories_by_elf.sort();

    Ok((
        calories_by_elf.last().copied().unwrap_or(0),
        Some(calories_by_elf.iter().copied().rev().take(3).sum()),
    ))
}
