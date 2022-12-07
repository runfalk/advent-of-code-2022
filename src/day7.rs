use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Default)]
struct DirectoryListing {
    dirs: HashMap<String, DirectoryListing>,
    files: HashMap<String, usize>,
}

impl DirectoryListing {
    fn direct_size(&self) -> usize {
        self.files.values().sum()
    }

    fn total_size(&self) -> usize {
        self.direct_size() + self.dirs.values().map(|d| d.total_size()).sum::<usize>()
    }

    fn cd(&mut self, path: &[String]) -> Option<&mut Self> {
        let mut listing = Some(self);
        for dir_name in path {
            listing = listing?.dirs.get_mut(dir_name);
        }
        listing
    }

    fn add_dir(&mut self, name: &str) {
        self.dirs
            .entry(name.to_owned())
            .or_insert_with(Default::default);
    }

    fn add_file(&mut self, name: &str, size: usize) {
        self.files.entry(name.to_owned()).or_insert(size);
    }
}

fn part_a(dl: &DirectoryListing) -> usize {
    let total_size = dl.total_size();
    dl.dirs.values().map(part_a).sum::<usize>() + if total_size <= 100_000 { total_size } else { 0 }
}

fn part_b(dl: &DirectoryListing) -> usize {
    let capacity = 70_000_000;
    let used = dl.total_size();
    let required_free_space = 30_000_000;
    let needs_freeing = used + required_free_space - capacity;

    let mut stack = vec![dl];
    let mut total_sizes = Vec::new();
    while let Some(d) = stack.pop() {
        stack.extend(d.dirs.values());
        total_sizes.push(d.total_size());
    }

    total_sizes.sort();

    // It's OK to unwrap since capacity is greater than free space and we can always remove all the
    // files
    total_sizes
        .into_iter()
        .find(|s| s >= &needs_freeing)
        .unwrap()
}

fn parse_terminal_output<E>(
    lines: impl Iterator<Item = Result<String, E>>,
) -> Result<DirectoryListing>
where
    E: std::error::Error + Sync + Send + 'static,
{
    let mut root = DirectoryListing::default();
    let mut cwd: Vec<String> = Vec::new();
    let mut read_stdout = false;
    for lr in lines {
        let line = lr?;
        match line.as_str() {
            "$ cd /" => {
                cwd = Vec::new();
                read_stdout = false;
            }
            "$ cd .." => {
                cwd.pop();
                read_stdout = false;
            }
            "$ ls" => {
                read_stdout = true;
            }
            _ if line.starts_with("$ cd ") => {
                cwd.push(line[5..].to_string());
                read_stdout = false;
            }
            _ if read_stdout => {
                let curr_dir = root.cd(&cwd).unwrap();
                if let Some(dir_name) = line.strip_prefix("dir ") {
                    curr_dir.add_dir(dir_name);
                } else if let Some((size_str, name)) = line.split_once(' ') {
                    curr_dir.add_file(name, size_str.parse()?);
                } else {
                    return Err(anyhow!("Invalid stdout for ls ({:?})", line));
                }
            }
            _ => return Err(anyhow!("Unknown input line {:?}", line)),
        }
    }
    Ok(root)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let file = File::open(path)?;
    let root = parse_terminal_output(io::BufReader::new(file).lines())?;

    Ok((part_a(&root), Some(part_b(&root))))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root() -> Result<DirectoryListing> {
        let lines = [
            Ok::<_, io::Error>("$ cd /".to_owned()),
            Ok::<_, io::Error>("$ ls".to_owned()),
            Ok::<_, io::Error>("dir a".to_owned()),
            Ok::<_, io::Error>("14848514 b.txt".to_owned()),
            Ok::<_, io::Error>("8504156 c.dat".to_owned()),
            Ok::<_, io::Error>("dir d".to_owned()),
            Ok::<_, io::Error>("$ cd a".to_owned()),
            Ok::<_, io::Error>("$ ls".to_owned()),
            Ok::<_, io::Error>("dir e".to_owned()),
            Ok::<_, io::Error>("29116 f".to_owned()),
            Ok::<_, io::Error>("2557 g".to_owned()),
            Ok::<_, io::Error>("62596 h.lst".to_owned()),
            Ok::<_, io::Error>("$ cd e".to_owned()),
            Ok::<_, io::Error>("$ ls".to_owned()),
            Ok::<_, io::Error>("584 i".to_owned()),
            Ok::<_, io::Error>("$ cd ..".to_owned()),
            Ok::<_, io::Error>("$ cd ..".to_owned()),
            Ok::<_, io::Error>("$ cd d".to_owned()),
            Ok::<_, io::Error>("$ ls".to_owned()),
            Ok::<_, io::Error>("4060174 j".to_owned()),
            Ok::<_, io::Error>("8033020 d.log".to_owned()),
            Ok::<_, io::Error>("5626152 d.ext".to_owned()),
            Ok::<_, io::Error>("7214296 k".to_owned()),
        ]
        .into_iter();
        parse_terminal_output(lines)
    }

    #[test]
    fn test_example_a() -> Result<()> {
        assert_eq!(part_a(&root()?), 95_437);
        Ok(())
    }

    #[test]
    fn test_example_b() -> Result<()> {
        assert_eq!(part_b(&root()?), 24_933_642);
        Ok(())
    }
}
