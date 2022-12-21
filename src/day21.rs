use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static MONKEY_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([a-z]{4}): (?:(\d+)|([a-z]{4}) ([-+*/]) ([a-z]{4}))$").unwrap());

#[derive(Debug, Clone)]
enum ExprRef {
    Scalar(isize),
    BinOp {
        op: BinOp,
        left: String,
        right: String,
    },
}

#[derive(Debug, Clone, Copy)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
enum Monkey {
    Scalar {
        name: String,
        value: isize,
    },
    BinOp {
        name: String,
        op: BinOp,
        left: Box<Self>,
        right: Box<Self>,
    },
}

impl BinOp {
    fn apply(self, left: &Monkey, right: &Monkey) -> isize {
        match self {
            BinOp::Add => left.eval() + right.eval(),
            BinOp::Sub => left.eval() - right.eval(),
            BinOp::Mul => left.eval() * right.eval(),
            BinOp::Div => left.eval() / right.eval(),
        }
    }
}

impl Monkey {
    fn depends_on(&self, monkey: &str) -> bool {
        match self {
            Monkey::Scalar { name, .. } => name == monkey,
            Monkey::BinOp {
                name, left, right, ..
            } => name == monkey || left.depends_on(monkey) || right.depends_on(monkey),
        }
    }

    fn eval(&self) -> isize {
        match self {
            Self::Scalar { value, .. } => *value,
            Self::BinOp {
                op, left, right, ..
            } => op.apply(left, right),
        }
    }
}

fn parse_monkey(s: &str) -> Result<(String, ExprRef)> {
    let Some(captures) = MONKEY_RE.captures(s) else {
        return Err(anyhow!("Invalid monkey {:?}", s));
    };
    let expr = if captures.get(2).is_some() {
        ExprRef::Scalar(captures[2].parse()?)
    } else {
        let op = match &captures[4] {
            "+" => BinOp::Add,
            "-" => BinOp::Sub,
            "*" => BinOp::Mul,
            "/" => BinOp::Div,
            _ => unreachable!(), // Unreachable because of the regex
        };
        let left = captures[3].to_string();
        let right = captures[5].to_string();
        ExprRef::BinOp { op, left, right }
    };
    Ok((captures[1].to_string(), expr))
}

fn into_monkey_ast<T: Into<String>>(
    monkeys: &mut HashMap<String, ExprRef>,
    root: T,
) -> Result<Box<Monkey>> {
    // We remove nodes as a validation step to ensure that no monkey's value is used more than once
    let name = root.into();
    let Some(expr) = monkeys.remove(&name) else {
        return Err(anyhow!("No monkey named root, or its value was already used by another monkey"));
    };

    Ok(Box::new(match expr {
        ExprRef::Scalar(value) => Monkey::Scalar { name, value },
        ExprRef::BinOp { op, left, right } => Monkey::BinOp {
            name,
            op,
            left: into_monkey_ast(monkeys, left)?,
            right: into_monkey_ast(monkeys, right)?,
        },
    }))
}

fn part_b(root_monkey: Monkey) -> Result<isize> {
    // This solution relies on the assumption that each monkey's value is only used once. We use
    // this to treat each monkey as an equation and substitute every monkey into the root one and
    // solve for "humn"
    let Monkey::BinOp { name, left, right, .. } = root_monkey else {
        return Err(anyhow!("Expected root monkey to depend on a binary operation"));
    };
    let mut monkey = &Monkey::BinOp {
        op: BinOp::Sub,
        name,
        left,
        right,
    };
    let mut static_value = 0;
    loop {
        let (name, op, left, right) = match monkey {
            Monkey::BinOp {
                name,
                op,
                left,
                right,
                ..
            } => (name, op, left, right),
            Monkey::Scalar { name, .. } => {
                if name == "humn" {
                    return Ok(static_value);
                } else {
                    return Err(anyhow!("Expected monkey to depend on a binary operation"));
                }
            }
        };

        if left.depends_on("humn") {
            match op {
                BinOp::Add => static_value -= right.eval(),
                BinOp::Sub => static_value += right.eval(),
                BinOp::Mul => static_value /= right.eval(),
                BinOp::Div => static_value *= right.eval(),
            }
            monkey = left;
        } else if right.depends_on("humn") {
            match op {
                BinOp::Add => static_value -= left.eval(),
                BinOp::Sub => static_value = left.eval() - static_value,
                BinOp::Mul => static_value /= left.eval(),
                BinOp::Div => static_value = left.eval() / static_value,
            }
            monkey = right;
        } else {
            return Err(anyhow!(
                "Monkey {:?} does not depend on the value of humn",
                name,
            ));
        };
    }
}

pub fn main(path: &Path) -> Result<(isize, Option<isize>)> {
    let file = File::open(path)?;
    let mut monkeys = io::BufReader::new(file)
        .lines()
        .map(|lr| parse_monkey(&lr?))
        .collect::<Result<HashMap<_, _>>>()?;
    let root_monkey = into_monkey_ast(&mut monkeys, "root")?;
    Ok((root_monkey.eval(), Some(part_b(*root_monkey)?)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_monkeys() -> Monkey {
        let mut monkeys = [
            "root: pppw + sjmn",
            "dbpl: 5",
            "cczh: sllz + lgvd",
            "zczc: 2",
            "ptdq: humn - dvpt",
            "dvpt: 3",
            "lfqf: 4",
            "humn: 5",
            "ljgn: 2",
            "sjmn: drzm * dbpl",
            "sllz: 4",
            "pppw: cczh / lfqf",
            "lgvd: ljgn * ptdq",
            "drzm: hmdt - zczc",
            "hmdt: 32",
        ]
        .into_iter()
        .map(|l| parse_monkey(l))
        .collect::<Result<HashMap<_, _>>>()
        .unwrap();
        *into_monkey_ast(&mut monkeys, "root").unwrap()
    }

    #[test]
    fn test_part_a() -> Result<()> {
        assert_eq!(example_monkeys().eval(), 152);
        Ok(())
    }

    #[test]
    fn test_part_b() -> Result<()> {
        assert_eq!(part_b(example_monkeys())?, 301);
        Ok(())
    }
}
