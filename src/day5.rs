use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::iter::repeat_with;
use std::path::Path;
use std::str::FromStr;

static PROCEDURE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap());

struct Procedure {
    num_crates: usize,
    from: usize,
    to: usize,
}

impl FromStr for Procedure {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = PROCEDURE_RE.captures(s) else {
            return Err(anyhow!("Invalid movement procedure {:?}", s));
        };
        Ok(Self {
            num_crates: captures[1].parse()?,
            from: captures[2].parse::<usize>()? - 1,
            to: captures[3].parse::<usize>()? - 1,
        })
    }
}

fn parse_stacks(s: &str) -> Result<Vec<Vec<char>>> {
    // NOTE: breaks if stacks are not spaced apart in the same way
    let (num_stacks, lines) = {
        let mut lines = s.lines().collect::<Vec<_>>();
        let Some(last_line) = lines.pop() else {
            return Err(anyhow!("Couldn't find any stacks"));
        };
        (last_line.split_whitespace().count(), lines)
    };

    let mut stacks = repeat_with(Vec::new)
        .take(num_stacks)
        .collect::<Vec<Vec<char>>>();
    for line in lines.into_iter().rev() {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c == ' ' {
                continue;
            }
            stacks[i].push(c);
        }
    }
    Ok(stacks)
}

fn part_a(mut stacks: Vec<Vec<char>>, procedures: &[Procedure]) -> Result<String> {
    for p in procedures {
        for _ in 0..p.num_crates {
            let Some(c) = stacks[p.from].pop() else {
                return Err(anyhow!("Stack {} is empty", p.from));
            };
            stacks[p.to].push(c);
        }
    }
    Ok(stacks
        .into_iter()
        .filter_map(|e| e.last().copied())
        .collect())
}

fn part_b(mut stacks: Vec<Vec<char>>, procedures: &[Procedure]) -> Result<String> {
    for p in procedures {
        let num_crates_kept = stacks[p.from].len() - p.num_crates;
        let moved_crates = stacks[p.from].split_off(num_crates_kept);
        stacks[p.to].extend(moved_crates);
    }
    Ok(stacks
        .into_iter()
        .filter_map(|e| e.last().copied())
        .collect())
}

pub fn main(path: &Path) -> Result<(String, Option<String>)> {
    let mut input = String::new();
    File::open(path)?.read_to_string(&mut input)?;

    let Some((stacks_str, procedures_str)) = input.split_once("\n\n") else {
        return Err(anyhow!("Unable to split input into crate configuration and move procedures"));
    };

    let stacks = parse_stacks(stacks_str)?;
    let procedures = procedures_str
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Procedure>>>()?;

    Ok((
        part_a(stacks.clone(), &procedures)?,
        Some(part_b(stacks, &procedures)?),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_STACKS: Lazy<Vec<Vec<char>>> =
        Lazy::new(|| vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);

    static EXAMPLE_PROCEDURES: Lazy<Vec<Procedure>> = Lazy::new(|| {
        vec![
            "move 1 from 2 to 1".parse().unwrap(),
            "move 3 from 1 to 3".parse().unwrap(),
            "move 2 from 2 to 1".parse().unwrap(),
            "move 1 from 1 to 2".parse().unwrap(),
        ]
    });

    #[test]
    fn test_example_a() -> Result<()> {
        assert_eq!(part_a(EXAMPLE_STACKS.clone(), &EXAMPLE_PROCEDURES)?, "CMZ");
        Ok(())
    }

    #[test]
    fn test_example_b() -> Result<()> {
        assert_eq!(part_b(EXAMPLE_STACKS.clone(), &EXAMPLE_PROCEDURES)?, "MCD");
        Ok(())
    }
}
