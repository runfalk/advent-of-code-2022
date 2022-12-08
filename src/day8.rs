use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part_a(trees: &HashMap<(isize, isize), u32>) -> Result<usize> {
    let width = trees.keys().map(|(x, _)| x + 1).max().unwrap_or(0);
    let height = trees.keys().map(|(_, y)| y + 1).max().unwrap_or(0);

    let mut visible = HashSet::new();
    for y in 0..height {
        let mut last_height = None;
        for x in 0..width {
            let Some(tree_height) = trees.get(&(x, y)) else {
                return Err(anyhow!("Tried to read out of bounds tree"));
            };

            if last_height.is_none() {
                last_height = Some(tree_height);
                visible.insert((x, y));
            }

            if let Some(lh) = last_height {
                if lh < tree_height {
                    last_height = Some(tree_height);
                    visible.insert((x, y));
                }
            }
        }
    }
    for y in 0..height {
        let mut last_height = None;
        for x in (0..width).rev() {
            let Some(tree_height) = trees.get(&(x, y)) else {
                return Err(anyhow!("Tried to read out of bounds tree"));
            };

            if last_height.is_none() {
                last_height = Some(tree_height);
                visible.insert((x, y));
            }

            if let Some(lh) = last_height {
                if lh < tree_height {
                    last_height = Some(tree_height);
                    visible.insert((x, y));
                }
            }
        }
    }
    for x in 0..width {
        let mut last_height = None;
        for y in 0..height {
            let Some(tree_height) = trees.get(&(x, y)) else {
                return Err(anyhow!("Tried to read out of bounds tree"));
            };

            if last_height.is_none() {
                last_height = Some(tree_height);
                visible.insert((x, y));
            }

            if let Some(lh) = last_height {
                if lh < tree_height {
                    last_height = Some(tree_height);
                    visible.insert((x, y));
                }
            }
        }
    }
    for x in 0..width {
        let mut last_height = None;
        for y in (0..height).rev() {
            let Some(tree_height) = trees.get(&(x, y)) else {
                return Err(anyhow!("Tried to read out of bounds tree"));
            };

            if last_height.is_none() {
                last_height = Some(tree_height);
                visible.insert((x, y));
            }

            if let Some(lh) = last_height {
                if lh < tree_height {
                    last_height = Some(tree_height);
                    visible.insert((x, y));
                }
            }
        }
    }
    Ok(visible.len())
}

fn score_tree(trees: &HashMap<(isize, isize), u32>, (x, y): (isize, isize)) -> usize {
    let reference_tree = trees.get(&(x, y)).unwrap();
    let mut score = 1;
    for (step_x, step_y) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
        let mut s = 0;
        for nt in (1..).map_while(|i| trees.get(&(x + i * step_x, y + i * step_y))) {
            s += 1;
            if nt >= reference_tree {
                break;
            }
        }
        score *= s;
    }
    score
}

fn part_b(trees: &HashMap<(isize, isize), u32>) -> usize {
    trees
        .keys()
        .copied()
        .map(|c| score_tree(trees, c))
        .max()
        .unwrap_or(0)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let mut trees = HashMap::new();
    for (y, lr) in io::BufReader::new(file).lines().enumerate() {
        for (x, tree_height) in lr?.chars().enumerate() {
            trees.insert(
                (x.try_into()?, y.try_into()?),
                tree_height
                    .to_digit(10)
                    .ok_or_else(|| anyhow!("Invalid character"))?,
            );
        }
    }

    Ok((part_a(&trees)?, Some(part_b(&trees))))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trees() -> HashMap<(isize, isize), u32> {
        ["30373", "25512", "65332", "33549", "35390"]
            .into_iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as isize, y as isize), c.to_digit(10).unwrap()))
            })
            .collect()
    }

    #[test]
    fn test_example_a() {
        assert_eq!(part_a(&trees()).unwrap(), 21);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(part_b(&trees()), 8);
    }
}
