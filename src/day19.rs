use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

const PART_A_TIME_LIMIT: usize = 24;
const PART_B_TIME_LIMIT: usize = 32;

static BLUEPRINT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(concat!(
        r"^Blueprint (\d+):",
        r" Each ore robot costs (\d+) ore.",
        r" Each clay robot costs (\d+) ore.",
        r" Each obsidian robot costs (\d+) ore and (\d+) clay.",
        r" Each geode robot costs (\d+) ore and (\d+) obsidian.$",
    ))
    .unwrap()
});

struct Blueprint {
    id: usize,
    ore_robot_ore_cost: usize,
    clay_robot_ore_cost: usize,
    obsidian_robot_ore_cost: usize,
    obsidian_robot_clay_cost: usize,
    geode_robot_ore_cost: usize,
    geode_robot_obsidian_cost: usize,
}

#[derive(Debug, Clone, Copy, Default)]
struct Resources {
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
}

impl Resources {
    fn gather_resources(self) -> Self {
        Self {
            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geodes: self.geodes + self.geode_robots,
            ..self
        }
    }
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = BLUEPRINT_RE.captures(s) else {
            return Err(anyhow!("Invalid blueprint {:?}", s));
        };
        Ok(Self {
            id: captures[1].parse()?,
            ore_robot_ore_cost: captures[2].parse()?,
            clay_robot_ore_cost: captures[3].parse()?,
            obsidian_robot_ore_cost: captures[4].parse()?,
            obsidian_robot_clay_cost: captures[5].parse()?,
            geode_robot_ore_cost: captures[6].parse()?,
            geode_robot_obsidian_cost: captures[7].parse()?,
        })
    }
}

fn find_max_geodes(blueprint: &Blueprint, time_limit: usize) -> usize {
    // Since we can only build one robot per turn we limit the number of each robot type to the
    // maximum resource requirement of that type for any bot. If we allowed more robots to be
    // built we would produce more than what could be consumed
    let max_ore_robots = blueprint
        .ore_robot_ore_cost
        .max(blueprint.clay_robot_ore_cost)
        .max(blueprint.obsidian_robot_ore_cost)
        .max(blueprint.geode_robot_ore_cost);
    let max_clay_robots = blueprint.obsidian_robot_clay_cost;
    let max_obsidian_robots = blueprint.geode_robot_obsidian_cost;

    let mut build_plans = Vec::new();
    let initial_state = Resources {
        ore_robots: 1,
        ..Default::default()
    };
    build_plans.push((time_limit, initial_state));

    let mut max_geodes = 0;
    while let Some((time_remaining, resources)) = build_plans.pop() {
        if time_remaining == 0 {
            max_geodes = max_geodes.max(resources.geodes);
            continue;
        }

        // Could we beat our current max score if we build a new robot every single minute until we
        // hit the time limit? If not we prune this branch
        let max_additional_geodes =
            time_remaining * resources.geode_robots + (0..time_remaining).sum::<usize>();
        if resources.geodes + max_additional_geodes <= max_geodes {
            continue;
        }

        let updated_resources = resources.gather_resources();
        if resources.ore >= blueprint.geode_robot_ore_cost
            && resources.obsidian >= blueprint.geode_robot_obsidian_cost
        {
            let mut r = updated_resources;
            r.geode_robots += 1;
            r.ore -= blueprint.geode_robot_ore_cost;
            r.obsidian -= blueprint.geode_robot_obsidian_cost;
            build_plans.push((time_remaining - 1, r));
        }
        if resources.obsidian_robots < max_obsidian_robots
            && resources.ore >= blueprint.obsidian_robot_ore_cost
            && resources.clay >= blueprint.obsidian_robot_clay_cost
        {
            let mut r = updated_resources;
            r.obsidian_robots += 1;
            r.ore -= blueprint.obsidian_robot_ore_cost;
            r.clay -= blueprint.obsidian_robot_clay_cost;
            build_plans.push((time_remaining - 1, r));
        }
        if resources.clay_robots < max_clay_robots && resources.ore >= blueprint.clay_robot_ore_cost
        {
            let mut r = updated_resources;
            r.clay_robots += 1;
            r.ore -= blueprint.clay_robot_ore_cost;
            build_plans.push((time_remaining - 1, r));
        }
        if resources.ore_robots < max_ore_robots && resources.ore >= blueprint.ore_robot_ore_cost {
            let mut r = updated_resources;
            r.ore_robots += 1;
            r.ore -= blueprint.ore_robot_ore_cost;
            build_plans.push((time_remaining - 1, r));
        }
        build_plans.push((time_remaining - 1, updated_resources));
    }
    max_geodes
}

fn part_a(blueprints: &[Blueprint]) -> usize {
    blueprints
        .iter()
        .map(|b| b.id * find_max_geodes(b, PART_A_TIME_LIMIT))
        .sum()
}

fn part_b(blueprints: &[Blueprint]) -> usize {
    blueprints
        .iter()
        .take(3)
        .map(|b| find_max_geodes(b, PART_B_TIME_LIMIT))
        .product()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let blueprints = io::BufReader::new(File::open(path)?)
        .lines()
        .map(|lr| lr?.parse())
        .collect::<Result<Vec<Blueprint>>>()?;
    Ok((part_a(&blueprints), Some(part_b(&blueprints))))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_BLUEPRINT_1: Blueprint = Blueprint {
        id: 1,
        ore_robot_ore_cost: 4,
        clay_robot_ore_cost: 2,
        obsidian_robot_ore_cost: 3,
        obsidian_robot_clay_cost: 14,
        geode_robot_ore_cost: 2,
        geode_robot_obsidian_cost: 7,
    };

    const EXAMPLE_BLUEPRINT_2: Blueprint = Blueprint {
        id: 2,
        ore_robot_ore_cost: 2,
        clay_robot_ore_cost: 3,
        obsidian_robot_ore_cost: 3,
        obsidian_robot_clay_cost: 8,
        geode_robot_ore_cost: 3,
        geode_robot_obsidian_cost: 12,
    };

    #[test]
    fn test_example_a() {
        assert_eq!(part_a(&[EXAMPLE_BLUEPRINT_1, EXAMPLE_BLUEPRINT_2]), 33);
    }

    #[test]
    fn test_example_b() {
        assert_eq!(part_b(&[EXAMPLE_BLUEPRINT_1, EXAMPLE_BLUEPRINT_2]), 3472);
    }
}
