use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part_a(item_calories: &[Option<usize>]) -> usize {
    let mut calories_by_elf = vec![0];
    for ic in item_calories.iter().copied() {
        match ic {
            Some(c) => *calories_by_elf.last_mut().unwrap() += c,
            None => calories_by_elf.push(0),
        }
    }
    calories_by_elf.into_iter().max().unwrap()
}

pub fn part_b(item_calories: &[Option<usize>]) -> usize {
    let mut calories_by_elf = vec![0];
    for ic in item_calories.iter().copied() {
        match ic {
            Some(c) => *calories_by_elf.last_mut().unwrap() += c,
            None => calories_by_elf.push(0),
        }
    }
    calories_by_elf.sort();
    calories_by_elf[calories_by_elf.len() - 3..].iter().sum()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let calories = io::BufReader::new(file)
        .lines()
        .map(|lr| Ok(lr?.parse::<usize>().ok()))
        .collect::<Result<Vec<Option<usize>>>>()?;
    Ok((part_a(&calories), Some(part_b(&calories))))
}
