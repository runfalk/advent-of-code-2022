use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_char(c: char) -> Result<Self> {
        match c {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(anyhow!("Invalid action {:?}", c)),
        }
    }

    fn value(self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn beaten_by(self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn beats(self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

fn parse_round(s: &str) -> Result<(char, char)> {
    let mut chars = s.chars();
    let Some(their_move) = chars.next() else {
        return Err(anyhow!("Their move not found"));
    };
    if chars.next() != Some(' ') {
        return Err(anyhow!("Moves must be separated by a single space"));
    }
    let Some(our_move) = chars.next() else {
        return Err(anyhow!("Our move not found"));
    };
    if chars.next().is_some() {
        return Err(anyhow!("Round is longer than 3 characters"));
    }
    Ok((their_move, our_move))
}

fn score_round(their_move: Move, our_move: Move) -> usize {
    our_move.value()
        + if our_move.beats() == their_move {
            6
        } else if our_move == their_move {
            3
        } else {
            0
        }
}

fn part_a(guide: &[(char, char)]) -> Result<usize> {
    let mut score = 0;
    for (them, us) in guide.iter() {
        let their_move = Move::from_char(*them)?;
        let our_move = Move::from_char(*us)?;
        score += score_round(their_move, our_move);
    }
    Ok(score)
}

fn part_b(guide: &[(char, char)]) -> Result<usize> {
    let mut score = 0;
    for (them, outcome) in guide.iter() {
        let their_move = Move::from_char(*them)?;
        let our_move = match outcome {
            'X' => their_move.beats(),
            'Y' => their_move,
            'Z' => their_move.beaten_by(),
            _ => Err(anyhow!("Invalid round result {}", outcome))?,
        };
        score += score_round(their_move, our_move);
    }
    Ok(score)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let guide = io::BufReader::new(file)
        .lines()
        .map(|lr| parse_round(&lr?))
        .collect::<Result<Vec<_>, _>>()?;
    Ok((part_a(&guide)?, Some(part_b(&guide)?)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static [&'static str] = &["A Y", "B X", "C Z"];

    #[test]
    fn test_example_a() -> Result<()> {
        let guide = INPUT
            .iter()
            .map(|r| parse_round(r))
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(part_a(&guide)?, 15);
        Ok(())
    }

    #[test]
    fn test_example_b() -> Result<()> {
        let guide = INPUT
            .iter()
            .map(|r| parse_round(r))
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(part_b(&guide)?, 12);
        Ok(())
    }
}
