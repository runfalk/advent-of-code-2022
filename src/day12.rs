use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn iter_neighbors(self) -> impl Iterator<Item = Self> {
        [
            Coord::new(self.x, self.y - 1), // Up
            Coord::new(self.x + 1, self.y), // Right
            Coord::new(self.x, self.y + 1), // Down
            Coord::new(self.x - 1, self.y), // Left
        ]
        .into_iter()
    }
}

fn find_shortest_path_len(
    heightmap: &HashMap<Coord, u8>,
    start: Coord,
    end: Coord,
) -> Option<usize> {
    // Use breadth first search to find the shortest path
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut to_visit = VecDeque::new();
    to_visit.push_back((0, start));

    while let Some((num_moves, curr_pos)) = to_visit.pop_front() {
        if curr_pos == end {
            return Some(num_moves);
        }
        let height = heightmap.get(&curr_pos).unwrap();

        for (neighbor, neighbor_height) in curr_pos
            .iter_neighbors()
            .filter_map(|n| heightmap.get(&n).map(|h| (n, *h)))
        {
            if neighbor_height > height + 1 || !visited.insert(neighbor) {
                continue;
            }
            to_visit.push_back((num_moves + 1, neighbor));
        }
    }
    None
}

fn part_b(heightmap: &HashMap<Coord, u8>, end: Coord) -> Option<usize> {
    heightmap
        .iter()
        .filter_map(|(&c, &h)| (h == 0).then_some(c))
        .filter_map(|start| find_shortest_path_len(heightmap, start, end))
        .min()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let mut heightmap: HashMap<Coord, u8> = HashMap::new();
    let mut start = None;
    let mut end = None;
    for (y, lr) in io::BufReader::new(File::open(path)?).lines().enumerate() {
        for (x, tile) in lr?.chars().enumerate() {
            let coord = Coord::new(x.try_into()?, y.try_into()?);
            match tile {
                'S' => {
                    start = Some(coord);
                    heightmap.insert(coord, 0);
                }
                'E' => {
                    end = Some(coord);
                    heightmap.insert(coord, 25);
                }
                _ if ('a'..='z').contains(&tile) => {
                    heightmap.insert(coord, u8::try_from(tile)? - 97);
                }
                _ => return Err(anyhow!("Invalid heightmap character {:?}", tile)),
            }
        }
    }

    let Some(start) = start else {
        return Err(anyhow!("Found no start position"));
    };
    let Some(end) = end else {
        return Err(anyhow!("Found no end position"));
    };

    Ok((
        find_shortest_path_len(&heightmap, start, end)
            .ok_or_else(|| anyhow!("Found no path for part A"))?,
        Some(part_b(&heightmap, end).ok_or_else(|| anyhow!("Found no paths for part A"))?),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const END: Coord = Coord::new(5, 2);

    fn example_heightmap() -> HashMap<Coord, u8> {
        ["aabqponm", "abcryxxl", "accszzxk", "acctuvwj", "abdefghi"]
            .into_iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, t)| {
                    (
                        Coord::new(x as isize, y as isize),
                        u8::try_from(t).unwrap() - 97,
                    )
                })
            })
            .collect()
    }

    #[test]
    fn test_example_a() {
        assert_eq!(
            find_shortest_path_len(&example_heightmap(), Coord::new(0, 0), END),
            Some(31)
        );
    }

    #[test]
    fn test_example_b() {
        assert_eq!(part_b(&example_heightmap(), END), Some(29));
    }
}
