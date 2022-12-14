use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let mut calories_by_elf = vec![0];
    for line in io::BufReader::new(file).lines() {
        let Some(calories) = line?.parse::<usize>().ok() else {
            calories_by_elf.push(0);
            continue
        };
        *calories_by_elf.last_mut().unwrap() += calories;
    }

    calories_by_elf.sort();

    Ok((
        calories_by_elf.last().copied().unwrap_or(0),
        Some(calories_by_elf.iter().copied().rev().take(3).sum()),
    ))
}
