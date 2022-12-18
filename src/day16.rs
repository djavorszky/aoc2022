use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::prelude::*;

pub fn run() -> Result<()> {
    let input = include_str!("input/day16.txt");

    println!("{}", task1(input)?);

    println!("{}", task2(input)?);

    Ok(())
}

fn task1(input: &str) -> Result<isize> {
    let valves = input
        .lines()
        .map(|line| line.parse::<Valve>().unwrap())
        .map(|v| (v.name.clone(), v))
        .collect();

    let mut cave = Cave::from(valves);

    cave.play();

    Ok(cave.relieved_pressure)
}

fn task2(input: &str) -> Result<usize> {
    todo!()
}

#[derive(Debug)]
struct Cave {
    valves: HashMap<String, Valve>,
    open_valves: HashSet<String>,
    location: String,
    remaining_minutes: isize,
    relieving_pressure: Vec<isize>,
    relieved_pressure: isize,
}

impl Cave {
    fn from(valves: HashMap<String, Valve>) -> Self {
        Self {
            valves,
            open_valves: HashSet::new(),
            location: "AA".to_string(),
            remaining_minutes: 30,
            relieving_pressure: vec![],
            relieved_pressure: 0,
        }
    }

    fn play(&mut self) {
        while self.remaining_minutes >= 0 {
            let Some((potential, next)) = self
                .valves
                .values()
                .filter(|v| {
                    v.flow_rate != 0
                        && v.name != self.location
                        && !self.open_valves.contains(&v.name)
                })
                .map(|valve| (self.potential(valve), valve))
                .max_by(|v1, v2| v1.0.cumulative_gain.cmp(&v2.0.cumulative_gain)) else {
                    self.relieved_pressure += self.relieving_pressure.iter().sum::<isize>() * self.remaining_minutes;

                    println!("oh no bye");
                    return;
                };

            self.remaining_minutes -= potential.distance;

            let pressure = self.relieving_pressure.iter().sum::<isize>();

            println!("Relievin pressure: {}", pressure);
            self.relieved_pressure += pressure * potential.distance;
            self.relieving_pressure.push(next.flow_rate);
            self.location = next.name.clone();
            self.open_valves.insert(self.location.clone());

            println!(
                "Opened {}, elapsed time: {}, relieved pressure: {}",
                self.location,
                30 - self.remaining_minutes,
                self.relieved_pressure
            );
        }
    }

    fn distance_between(
        &self,
        from: &String,
        to: &String,
        path: &mut HashSet<String>,
    ) -> Option<isize> {
        path.insert(from.clone());
        let valve = self.valves.get(from).unwrap();
        for c in valve.connections.iter() {
            if path.contains(c) {
                continue;
            }
            if c == to {
                return Some(1);
            }

            if let Some(res) = self.distance_between(c, to, path) {
                return Some(res + 1);
            }
        }
        path.remove(from);

        None
    }

    fn potential(&self, valve: &Valve) -> Potential {
        if valve.flow_rate == 0 {
            return Potential::default();
        };

        let d = self
            .distance_between(
                &self.location,
                &valve.name,
                &mut HashSet::from([self.location.clone()]),
            )
            .context(format!("{} -> {}", self.location, valve.name))
            .unwrap();

        Potential {
            cumulative_gain: (self.remaining_minutes - d) * valve.flow_rate,
            distance: d,
        }
    }
}

#[derive(Debug, Default)]
struct Potential {
    distance: isize,
    cumulative_gain: isize,
}

#[derive(PartialEq, Eq, Debug)]
struct Valve {
    name: String,
    flow_rate: isize,
    connections: Vec<String>,
}

impl FromStr for Valve {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split_ascii_whitespace();

        let name = parts.nth(1).unwrap().to_string();

        let flow_rate = parts
            .nth(2) // go to "rate=num"
            .map(|p| {
                let (_, num) = p.split_once('=').unwrap();
                let mut chars = num.chars();
                chars.next_back();
                chars.as_str().parse::<isize>().unwrap()
            })
            .unwrap();

        // go to end (valve)
        parts.nth(3);

        let connections = parts
            .map(|v| v.trim_end_matches(',').to_string())
            .collect_vec();

        Ok(Self {
            name,
            flow_rate,
            connections,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;

    #[test_case("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB", Valve { name: "AA".to_string(), flow_rate: 0, connections: vec!["DD".to_string(), "II".to_string(), "BB".to_string()]})]
    #[test_case("Valve HH has flow rate=22; tunnel leads to valve GG", Valve { name: "HH".to_string(), flow_rate: 22, connections: vec!["GG".to_string()]})]
    fn test_parse_valve(input: &str, expected: Valve) {
        assert_eq!(input.parse::<Valve>().unwrap(), expected);
    }

    #[test]
    fn test_task_1() {
        let input = include_str!("input/day16_example.txt");

        assert_eq!(task1(input).unwrap(), 1651)
    }
}
