use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SnafuDigit {
    DoubleMinus,
    Minus,
    Zero,
    One,
    Two,
}

#[derive(Debug, Clone)]
struct SnafuNumber(Vec<SnafuDigit>);

impl SnafuDigit {
    fn from_char(c: char) -> Result<Self> {
        match c {
            '=' => Ok(SnafuDigit::DoubleMinus),
            '-' => Ok(SnafuDigit::Minus),
            '0' => Ok(SnafuDigit::Zero),
            '1' => Ok(SnafuDigit::One),
            '2' => Ok(SnafuDigit::Two),
            _ => Err(anyhow!("Invalid snafu digit {:?}", c)),
        }
    }

    fn to_char(self) -> char {
        match self {
            Self::DoubleMinus => '=',
            Self::Minus => '-',
            Self::Zero => '0',
            Self::One => '1',
            Self::Two => '2',
        }
    }
}

impl SnafuNumber {
    fn new(mut n: isize) -> Self {
        if n < 0 {
            panic!("Negative numbers are currently not handled");
        }

        let mut snafu_digits = Vec::new();
        let mut carry = 0;
        while n > 0 {
            let rem = n % 5;

            if rem == 4 {
                carry += 1;
                snafu_digits.push(SnafuDigit::Minus);
            } else if rem == 3 {
                carry += 1;
                snafu_digits.push(SnafuDigit::DoubleMinus);
            } else if rem == 2 {
                snafu_digits.push(SnafuDigit::Two);
            } else if rem == 1 {
                snafu_digits.push(SnafuDigit::One);
            } else {
                snafu_digits.push(SnafuDigit::Zero);
            }

            n = (n - rem) / 5 + carry;
            carry = 0;
        }
        if snafu_digits.is_empty() {
            snafu_digits.push(SnafuDigit::Zero);
        }
        Self(snafu_digits)
    }

    fn to_isize(&self) -> isize {
        self.0
            .iter()
            .copied()
            .zip(0..)
            .map(|(s, i)| {
                let multiplier = match s {
                    SnafuDigit::DoubleMinus => -2,
                    SnafuDigit::Minus => -1,
                    SnafuDigit::Zero => 0,
                    SnafuDigit::One => 1,
                    SnafuDigit::Two => 2,
                };
                multiplier * 5isize.pow(i)
            })
            .sum()
    }
}

impl FromStr for SnafuNumber {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .rev()
                .map(SnafuDigit::from_char)
                .collect::<Result<Vec<_>>>()?,
        ))
    }
}

impl ToString for SnafuNumber {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .copied()
            .rev()
            .map(SnafuDigit::to_char)
            .collect()
    }
}

fn part_a(snafu_numbers: &[SnafuNumber]) -> String {
    let sum = snafu_numbers.iter().map(SnafuNumber::to_isize).sum();
    SnafuNumber::new(sum).to_string()
}

pub fn main(path: &Path) -> Result<(String, Option<usize>)> {
    let mut snafu_numbers_str = String::new();
    File::open(path)?.read_to_string(&mut snafu_numbers_str)?;
    let snafu_numbers = snafu_numbers_str
        .lines()
        .map(SnafuNumber::from_str)
        .collect::<Result<Vec<_>>>()?;

    Ok((part_a(&snafu_numbers), None))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PAIRS: &'static [(isize, &'static str)] = &[
        (0, "0"),
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (11, "21"),
        (15, "1=0"),
        (20, "1-0"),
        (31, "111"),
        (32, "112"),
        (37, "122"),
        (107, "1-12"),
        (198, "2=0="),
        (201, "2=01"),
        (353, "1=-1="),
        (906, "12111"),
        (1257, "20012"),
        (1747, "1=-0-2"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
    ];

    #[test]
    fn test_snafu_numbers() -> Result<()> {
        for (decimal, snafu_str) in EXAMPLE_PAIRS {
            let snafu_number = SnafuNumber::from_str(snafu_str)?;
            assert_eq!(*decimal, snafu_number.to_isize());
            assert_eq!(snafu_str, &snafu_number.to_string());
        }
        Ok(())
    }
}
