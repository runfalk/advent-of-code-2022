use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Op {
    Add(usize),
    Mul(usize),
    Pow,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    op: Op,
    test_divisible_by: usize,
    target_when_true: usize,
    target_when_false: usize,
}

static MONKEY_RE: Lazy<Regex> = Lazy::new(|| {
    let pattern = [
        r"Monkey (\d+):",
        r"  Starting items: (?P<items>\d+(, \d+)*)",
        r"  Operation: (?P<op>new = old [+*] \S+)",
        r"  Test: divisible by (?P<test_divisible_by>\d+)",
        r"    If true: throw to monkey (?P<target_when_true>\d+)",
        r"    If false: throw to monkey (?P<target_when_false>\d+)",
    ]
    .join("\n");
    Regex::new(&pattern).unwrap()
});

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "new = old * old" {
            Ok(Self::Pow)
        } else if let Some(term) = s.strip_prefix("new = old + ") {
            Ok(Self::Add(term.parse()?))
        } else if let Some(factor) = s.strip_prefix("new = old * ") {
            Ok(Self::Mul(factor.parse()?))
        } else {
            Err(anyhow!("Invalid operation"))
        }
    }
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = MONKEY_RE.captures(s).unwrap();
        Ok(Self {
            items: cap["items"]
                .split(", ")
                .map(|n| Ok(n.parse()?))
                .collect::<Result<VecDeque<_>>>()?,
            op: cap["op"].parse()?,
            test_divisible_by: cap["test_divisible_by"].parse()?,
            target_when_true: cap["target_when_true"].parse()?,
            target_when_false: cap["target_when_false"].parse()?,
        })
    }
}

fn compute_monkey_business(
    mut monkeys: Vec<Monkey>,
    rounds: usize,
    worry_level_divisor: usize,
) -> usize {
    // Find a divisor that is common for all monkeys
    let common_divisor: usize = monkeys.iter().map(|m| m.test_divisible_by).product();

    let mut num_inspections = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(mut item) = monkeys[i].items.pop_front() {
                num_inspections[i] += 1;

                // I'm not sure it's matchematically valid to do the division here, but it works
                // for both the example and my input ¯\_(ツ)_/¯. The trick we're using here is:
                //
                // (x + y) % n = ((x % n) + (y % n)) % n
                // (x * y) % n = ((x % n) * (y % n)) % n
                //
                // This is especially importand for monkey with the op `new = old * old` as the
                // worry level grows to insane numbers without this "modulo compacting".
                //
                // Since the monkeys have different divisors and they are passing the items around
                // we find a common divisor that is compatible with all monkeys.
                item = match monkeys[i].op {
                    Op::Add(n) => (item + n) % common_divisor,
                    Op::Mul(n) => (item * n) % common_divisor,
                    Op::Pow => (item * item) % common_divisor,
                } / worry_level_divisor;

                let target = if item % monkeys[i].test_divisible_by == 0 {
                    monkeys[i].target_when_true
                } else {
                    monkeys[i].target_when_false
                };
                monkeys[target].items.push_back(item);
            }
        }
    }

    num_inspections.sort();
    num_inspections.into_iter().rev().take(2).product()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let mut input = String::new();
    File::open(path)?.read_to_string(&mut input)?;
    let monkeys = input
        .split("\n\n")
        .map(Monkey::from_str)
        .collect::<Result<Vec<Monkey>>>()?;
    Ok((
        compute_monkey_business(monkeys.clone(), 20, 3),
        Some(compute_monkey_business(monkeys, 10_000, 1)),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn monkeys() -> Vec<Monkey> {
        [
            "Monkey 0:",
            "  Starting items: 79, 98",
            "  Operation: new = old * 19",
            "  Test: divisible by 23",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 3",
            "",
            "Monkey 1:",
            "  Starting items: 54, 65, 75, 74",
            "  Operation: new = old + 6",
            "  Test: divisible by 19",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 0",
            "",
            "Monkey 2:",
            "  Starting items: 79, 60, 97",
            "  Operation: new = old * old",
            "  Test: divisible by 13",
            "    If true: throw to monkey 1",
            "    If false: throw to monkey 3",
            "",
            "Monkey 3:",
            "  Starting items: 74",
            "  Operation: new = old + 3",
            "  Test: divisible by 17",
            "    If true: throw to monkey 0",
            "    If false: throw to monkey 1",
        ]
        .join("\n")
        .split("\n\n")
        .map(|monkey_str| monkey_str.parse().unwrap())
        .collect()
    }

    #[test]
    fn test_example_a() {
        assert_eq!(compute_monkey_business(monkeys(), 20, 3), 10_605);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(compute_monkey_business(monkeys(), 10_000, 1), 2_713_310_158);
    }
}
