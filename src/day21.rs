use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static MONKEY_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([a-z]{4}): (?:(\d+)|([a-z]{4}) ([-+*/]) ([a-z]{4}))$").unwrap());

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Scalar(isize),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

fn find_monkey_expr<'a>(monkeys: &'a HashMap<String, Expr>, monkey: &str) -> Result<&'a Expr> {
    monkeys
        .get(monkey)
        .ok_or_else(|| anyhow!("No such monkey {:?}", monkey))
}

impl Expr {
    fn operands(&self) -> Option<(&str, &str)> {
        match self {
            Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) => Some((a, b)),
            Expr::Scalar(_) => None,
        }
    }

    fn depends_on(&self, monkeys: &HashMap<String, Expr>, monkey: &str) -> Result<bool> {
        let Some((a, b)) = self.operands() else {
            return Ok(false);
        };
        if a == monkey || b == monkey {
            return Ok(true);
        }
        let a_expr = find_monkey_expr(monkeys, a)?;
        let b_expr = find_monkey_expr(monkeys, b)?;
        Ok(a_expr.depends_on(monkeys, monkey)? || b_expr.depends_on(monkeys, monkey)?)
    }

    fn eval(&self, monkeys: &HashMap<String, Expr>) -> Result<isize> {
        Ok(match self {
            Expr::Scalar(n) => *n,
            Expr::Add(a, b) => {
                find_monkey_expr(monkeys, a)?.eval(monkeys)?
                    + find_monkey_expr(monkeys, b)?.eval(monkeys)?
            }
            Expr::Sub(a, b) => {
                find_monkey_expr(monkeys, a)?.eval(monkeys)?
                    - find_monkey_expr(monkeys, b)?.eval(monkeys)?
            }
            Expr::Mul(a, b) => {
                find_monkey_expr(monkeys, a)?.eval(monkeys)?
                    * find_monkey_expr(monkeys, b)?.eval(monkeys)?
            }
            Expr::Div(a, b) => {
                find_monkey_expr(monkeys, a)?.eval(monkeys)?
                    / find_monkey_expr(monkeys, b)?.eval(monkeys)?
            }
        })
    }
}

fn parse_monkey(s: &str) -> Result<(String, Expr)> {
    let Some(captures) = MONKEY_RE.captures(s) else {
        return Err(anyhow!("Invalid monkey {:?}", s));
    };
    let expr = if captures.get(2).is_some() {
        Expr::Scalar(captures[2].parse()?)
    } else {
        match &captures[4] {
            "+" => Expr::Add(captures[3].to_string(), captures[5].to_string()),
            "-" => Expr::Sub(captures[3].to_string(), captures[5].to_string()),
            "*" => Expr::Mul(captures[3].to_string(), captures[5].to_string()),
            "/" => Expr::Div(captures[3].to_string(), captures[5].to_string()),
            _ => unreachable!(),
        }
    };
    Ok((captures[1].to_string(), expr))
}

fn part_a(monkeys: &HashMap<String, Expr>) -> Result<isize> {
    let Some(expr) = monkeys.get("root") else {
        return Err(anyhow!("No monkey named root"));
    };
    expr.eval(monkeys)
}

fn part_b(monkeys: &HashMap<String, Expr>) -> Result<isize> {
    // This solution relies on the assumption that each monkey's value is only used once. We use
    // this to treat each monkey as an equation and substitute every monkey into the root one and
    // solve for "humn"
    let Some(root_expr) = monkeys.get("root") else {
        return Err(anyhow!("No monkey named root"));
    };
    let Some((root_left, root_right)) = root_expr.operands() else {
        return Err(anyhow!("Expected root monkey to depend on a binary operation"));
    };

    // a - b = 0 means a and b are equal
    let mut static_value = 0;
    let mut expr = &Expr::Sub(root_left.to_string(), root_right.to_string());
    loop {
        let Some((left, right)) = expr.operands() else {
            return Err(anyhow!("Expected monkey to depend on a binary operation"));
        };
        let left_expr = find_monkey_expr(monkeys, left)?;
        let right_expr = find_monkey_expr(monkeys, right)?;

        // Our solution will never work if both the left and right side depends on humn
        if left_expr.depends_on(monkeys, "humn")? && right_expr.depends_on(monkeys, "humn")? {
            return Err(anyhow!("humn is depended upon in multiple locations"));
        }

        if left == "humn" || left_expr.depends_on(monkeys, "humn")? {
            let right_eval = right_expr.eval(monkeys)?;
            match expr {
                Expr::Add(_, _) => static_value -= right_eval,
                Expr::Sub(_, _) => static_value += right_eval,
                Expr::Mul(_, _) => static_value /= right_eval,
                Expr::Div(_, _) => static_value *= right_eval,
                Expr::Scalar(_) => unreachable!(),
            }
            expr = left_expr;
            if left == "humn" {
                return Ok(static_value);
            }
        } else if right == "humn" || right_expr.depends_on(monkeys, "humn")? {
            let left_eval = left_expr.eval(monkeys)?;
            match expr {
                Expr::Add(_, _) => static_value -= left_eval,
                Expr::Sub(_, _) => static_value = left_eval - static_value,
                Expr::Mul(_, _) => static_value /= left_eval,
                Expr::Div(_, _) => static_value = left_eval / static_value,
                Expr::Scalar(_) => unreachable!(),
            }
            expr = right_expr;
            if right == "humn" {
                return Ok(static_value);
            }
        } else {
            return Err(anyhow!(
                "Monkey with expr {:?} does not depend on the value of humn",
                expr
            ));
        };
    }
}

pub fn main(path: &Path) -> Result<(isize, Option<isize>)> {
    let file = File::open(path)?;
    let monkeys = io::BufReader::new(file)
        .lines()
        .map(|lr| parse_monkey(&lr?))
        .collect::<Result<HashMap<_, _>>>()?;
    Ok((part_a(&monkeys)?, Some(part_b(&monkeys)?)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_monkeys() -> HashMap<String, Expr> {
        [
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
        .unwrap()
    }

    #[test]
    fn test_part_a() -> Result<()> {
        assert_eq!(part_a(&example_monkeys())?, 152);
        Ok(())
    }

    #[test]
    fn test_part_b() -> Result<()> {
        assert_eq!(part_b(&example_monkeys())?, 301);
        Ok(())
    }
}
