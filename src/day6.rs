use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn find_packet_start(input: &str, marker_size: usize) -> Option<usize> {
    let chars = input.chars().collect::<Vec<_>>();
    for (i, window) in chars.windows(marker_size).enumerate() {
        if window.iter().copied().collect::<HashSet<_>>().len() == marker_size {
            return Some(i + marker_size);
        }
    }
    None
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let mut buf = String::new();
    File::open(path)?.read_to_string(&mut buf)?;
    buf.pop();

    Ok((
        find_packet_start(&buf, 4).ok_or_else(|| anyhow!("Couldn't find start of packet"))?,
        Some(find_packet_start(&buf, 14).ok_or_else(|| anyhow!("Couldn't find start of packet"))?),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_a() -> Result<()> {
        assert_eq!(
            find_packet_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4),
            Some(7)
        );
        assert_eq!(
            find_packet_start("bvwbjplbgvbhsrlpgdmjqwftvncz", 4),
            Some(5)
        );
        assert_eq!(
            find_packet_start("nppdvjthqldpwncqszvftbrmjlhg", 4),
            Some(6)
        );
        assert_eq!(
            find_packet_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            Some(10)
        );
        assert_eq!(
            find_packet_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            Some(11)
        );
        Ok(())
    }

    #[test]
    fn test_example_b() -> Result<()> {
        assert_eq!(
            find_packet_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14),
            Some(19)
        );
        assert_eq!(
            find_packet_start("bvwbjplbgvbhsrlpgdmjqwftvncz", 14),
            Some(23)
        );
        assert_eq!(
            find_packet_start("nppdvjthqldpwncqszvftbrmjlhg", 14),
            Some(23)
        );
        assert_eq!(
            find_packet_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            Some(29)
        );
        assert_eq!(
            find_packet_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            Some(26)
        );
        Ok(())
    }
}
