use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn decrypt_grove_coordinate_sum(
    encrypted_file: &[isize],
    num_iterations: usize,
    decryption_key: isize,
) -> isize {
    // This produces a shifted version of the example solution, but that doesn't matter since the
    // list is circular and the answer depends on the position of 0
    let indexed_values = encrypted_file
        .iter()
        .copied()
        .map(|v| v * decryption_key)
        .enumerate()
        .collect::<Vec<(usize, isize)>>();
    let mut reordered_values = indexed_values.clone();

    for _ in 0..num_iterations {
        for (original_index, value) in indexed_values.iter().copied() {
            let curr_reordered_index = reordered_values
                .iter()
                .position(|x| x == &(original_index, value))
                .unwrap();
            reordered_values.remove(curr_reordered_index);
            reordered_values.insert(
                (curr_reordered_index as isize + value).rem_euclid(reordered_values.len() as isize)
                    as usize,
                (original_index, value),
            );
        }
    }
    reordered_values
        .into_iter()
        .map(|(_, v)| v)
        .cycle()
        .skip_while(|&v| v != 0)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum()
}

fn part_a(encrypted_file: &[isize]) -> isize {
    decrypt_grove_coordinate_sum(encrypted_file, 1, 1)
}

fn part_b(encrypted_file: &[isize]) -> isize {
    let decryption_key = 811589153;
    decrypt_grove_coordinate_sum(encrypted_file, 10, decryption_key)
}

pub fn main(path: &Path) -> Result<(isize, Option<isize>)> {
    let encrypted_file = io::BufReader::new(File::open(path)?)
        .lines()
        .map(|lr| Ok(lr?.parse()?))
        .collect::<Result<Vec<isize>>>()?;
    if encrypted_file.iter().copied().filter(|&v| v == 0).count() != 1 {
        return Err(anyhow!("Encrypted must contain exactly one 0"));
    }
    Ok((part_a(&encrypted_file), Some(part_b(&encrypted_file))))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static [isize] = &[1, 2, -3, 3, -2, 0, 4];

    #[test]
    fn test_example_a() {
        assert_eq!(part_a(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(part_b(EXAMPLE_INPUT), 1_623_178_306);
    }
}
