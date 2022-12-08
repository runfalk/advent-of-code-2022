use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Perform type erasure by boxing the given iterator
fn box_iter<'a, I: Iterator<Item = T> + 'a, T>(it: I) -> Box<dyn Iterator<Item = T> + 'a> {
    Box::new(it)
}

fn part_a(trees: &HashMap<(isize, isize), u32>) -> Result<usize> {
    let width = trees.keys().map(|(x, _)| x + 1).max().unwrap_or(0);
    let height = trees.keys().map(|(_, y)| y + 1).max().unwrap_or(0);

    // Generate line scans for all directions, for all edge cells
    let from_left = (0..height).map(|y| box_iter((0..width).map(move |x| (x, y))));
    let from_right = (0..height).map(|y| box_iter((0..width).rev().map(move |x| (x, y))));
    let from_top = (0..width).map(|x| box_iter((0..height).map(move |y| (x, y))));
    let from_bottom = (0..width).map(|x| box_iter((0..height).rev().map(move |y| (x, y))));

    let mut visible = HashSet::new();
    for mut line_scan in from_left
        .chain(from_top)
        .chain(from_right)
        .chain(from_bottom)
    {
        let Some((edge_x, edge_y)) = line_scan.next() else {
            // This would only happen if there are no trees
            continue
        };
        let Some(mut tallest_tree) = trees.get(&(edge_x, edge_y)) else {
            return Err(anyhow!("Edge tree is not in set of trees"));
        };
        visible.insert((edge_x, edge_y));

        for (x, y) in line_scan {
            let Some(tree_height) = trees.get(&(x, y)) else {
                return Err(anyhow!("Tried to access an out of bounds tree"));
            };

            if tree_height > tallest_tree {
                tallest_tree = tree_height;
                visible.insert((x, y));
            }
        }
    }
    Ok(visible.len())
}

fn part_b(trees: &HashMap<(isize, isize), u32>) -> usize {
    trees
        .iter()
        .map(|(&(x, y), &reference_tree)| {
            // Explore all four directions, compute partial score and multiply them together
            let mut score = 1;
            for (step_x, step_y) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                let mut partial_score = 0;
                for neighbor in (1..).map_while(|i| trees.get(&(x + i * step_x, y + i * step_y))) {
                    partial_score += 1;
                    if *neighbor >= reference_tree {
                        break;
                    }
                }
                score *= partial_score;
            }
            score
        })
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
    fn test_example_a() -> Result<()> {
        assert_eq!(part_a(&trees()).unwrap(), 21);
        Ok(())
    }

    #[test]
    fn test_example_b() {
        assert_eq!(part_b(&trees()), 8);
    }

    #[test]
    fn test_no_trees() -> Result<()> {
        assert_eq!(part_a(&HashMap::new())?, 0);
        assert_eq!(part_b(&HashMap::new()), 0);
        Ok(())
    }
}
