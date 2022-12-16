use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

static VALVE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z]{2}(?:, [A-Z]{2})*)$")
        .unwrap()
});

const FIRST_VALVE: &str = "AA";

#[derive(Debug)]
struct ValveSpec {
    name: String,
    flow_rate: usize,
    leads_to: Vec<String>,
}

#[derive(Debug)]
struct ValveInfo {
    cost: usize,
    flow_rate: usize,
}

impl FromStr for ValveSpec {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = VALVE_RE.captures(s) else {
            return Err(anyhow!("Invalid valve {:?}", s));
        };
        Ok(Self {
            name: captures[1].to_string(),
            flow_rate: captures[2].parse()?,
            leads_to: captures[3].split(", ").map(ToString::to_string).collect(),
        })
    }
}

fn find_shortest_path_lens(
    valves: &HashMap<String, ValveSpec>,
    source: &str,
) -> Result<HashMap<String, usize>> {
    let mut to_explore = VecDeque::new();
    to_explore.push_back(vec![source.to_string()]);
    let mut visited = HashSet::new();

    let mut shortest_path_lens = HashMap::new();
    while let Some(path) = to_explore.pop_front() {
        let source_name = path.last().unwrap();
        let Some(source) = valves.get(source_name) else {
            return Err(anyhow!("No such tunnel {:?}", source_name));
        };

        for next_valve_name in source.leads_to.iter().cloned() {
            if !visited.insert(next_valve_name.clone()) {
                continue;
            }
            shortest_path_lens.insert(next_valve_name.clone(), path.len());

            let mut next_path = path.clone();
            next_path.push(next_valve_name);
            to_explore.push_back(next_path);
        }
    }
    Ok(shortest_path_lens)
}

fn valve_cost_map(
    valves: &HashMap<String, ValveSpec>,
) -> Result<HashMap<String, HashMap<String, ValveInfo>>> {
    let mut cost_map = HashMap::new();
    for parent_valve in valves.values() {
        // Skip building a cost map for nodes we'll never open valves at
        if parent_valve.flow_rate == 0 && parent_valve.name != FIRST_VALVE {
            continue;
        }

        let mut local_cost_map = HashMap::new();
        for (valve_name, cost) in find_shortest_path_lens(valves, &parent_valve.name)? {
            let Some(valve) = valves.get(&valve_name) else {
                return Err(anyhow!("No such valve {:?}", valve_name));
            };
            if valve.name != FIRST_VALVE && valve.flow_rate == 0 {
                continue;
            }
            local_cost_map.insert(
                valve_name,
                ValveInfo {
                    cost,
                    flow_rate: valve.flow_rate,
                },
            );
        }
        cost_map.insert(parent_valve.name.clone(), local_cost_map);
    }
    Ok(cost_map)
}

fn part_a(cost_map: &HashMap<String, HashMap<String, ValveInfo>>) -> Result<usize> {
    let time_limit = 30usize;

    let mut to_visit = VecDeque::new();
    to_visit.push_back((vec![FIRST_VALVE.to_string()], time_limit, 0));
    let mut max_pressure = 0;
    while let Some((path, time_remaining, acc_pressure)) = to_visit.pop_front() {
        let curr_valve_name = path.last().unwrap();
        max_pressure = max_pressure.max(acc_pressure);

        for (next_valve, ValveInfo { cost, flow_rate }) in cost_map.get(curr_valve_name).unwrap() {
            if path.contains(next_valve) {
                continue;
            }
            let Some(next_time_remaining) = time_remaining.checked_sub(cost + 1) else {
                continue;
            };
            let mut new_path = path.clone();
            new_path.push(next_valve.clone());
            to_visit.push_back((
                new_path,
                next_time_remaining,
                acc_pressure + next_time_remaining * flow_rate,
            ));
        }
    }
    Ok(max_pressure)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let valves = io::BufReader::new(File::open(path)?)
        .lines()
        .map(|lr| {
            let valve: ValveSpec = lr?.parse()?;
            Ok((valve.name.clone(), valve))
        })
        .collect::<Result<HashMap<String, ValveSpec>>>()?;
    let valve_costs = valve_cost_map(&valves)?;
    Ok((part_a(&valve_costs)?, None))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_valves() -> HashMap<String, HashMap<String, ValveInfo>> {
        let valves = [
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
            "Valve HH has flow rate=22; tunnel leads to valve GG",
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
            "Valve JJ has flow rate=21; tunnel leads to valve II",
        ]
        .into_iter()
        .map(|l| {
            let valve: ValveSpec = l.parse()?;
            Ok((valve.name.clone(), valve))
        })
        .collect::<Result<HashMap<_, _>>>()
        .unwrap();
        valve_cost_map(&valves).unwrap()
    }

    #[test]
    fn test_example_a() -> Result<()> {
        assert_eq!(part_a(&example_valves())?, 1651);
        Ok(())
    }
}
