use std::str::FromStr;

use crate::{prelude::*, vector::Vector2};

pub fn run() -> Result<()> {
    let input = include_str!("input/day15.txt");

    println!("{}", task1(input, 2000000)?);

    println!("{}", task2(input)?);

    Ok(())
}

fn task1(input: &str, line: isize) -> Result<usize> {
    let sensors = input
        .lines()
        .map(|line| line.parse::<Sensor>().unwrap())
        .filter(|s| s.in_range(line))
        .collect_vec();

    let sensors_in_range = sensors.iter().filter(|s| s.in_range(line)).collect_vec();

    let mut ranges = sensors_in_range
        .iter()
        .map(|sensor| {
            let num_covered = sensor.range - (sensor.loc.1 - line).abs();

            sensor.loc.0 - num_covered..=sensor.loc.0 + num_covered
        })
        .collect_vec();

    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut count = 0;
    let mut covered_range = ranges[0].clone();

    for range in ranges.iter().skip(1) {
        if covered_range.start() < range.start() && covered_range.end() > range.end() {
            continue;
        }

        if range.start() <= covered_range.end() {
            covered_range = *covered_range.start()..=*range.end();
            continue;
        }

        count += covered_range.count();
        covered_range = range.clone();
    }

    count += covered_range.count();

    count -= sensors
        .iter()
        .filter_map(|s| {
            if s.loc.1 == line {
                Some(s.loc.0)
            } else if s.beacon_loc.1 == line {
                Some(s.beacon_loc.0)
            } else {
                None
            }
        })
        .unique()
        .count();

    Ok(count)
}

fn task2(input: &str) -> Result<usize> {
    todo!()
}

#[derive(PartialEq, Eq, Debug)]
struct Sensor {
    loc: Vector2,
    beacon_loc: Vector2,
    range: isize,
}

impl Sensor {
    fn in_range(&self, row: isize) -> bool {
        (self.loc.1 - self.range).abs() < row
    }
}

impl FromStr for Sensor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (first, second) = s
            .trim_start_matches("Sensor at x=")
            .split_once(':')
            .ok_or_else(|| anyhow!("Blah"))?;

        // first part
        let parts = first.split_once(", y=").ok_or_else(|| anyhow!("Oh nooo"))?;
        let loc: Vector2 = parts.try_into()?;

        let second = second.trim_start_matches(" closest beacon is at x=");

        let parts = second
            .split_once(", y=")
            .ok_or_else(|| anyhow!("Oh nooo"))?;
        let beacon_loc = parts.try_into()?;

        Ok(Self {
            range: loc.manhattan_distance(&beacon_loc),
            loc,
            beacon_loc,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_sensor() {
        let input = "Sensor at x=12, y=14: closest beacon is at x=10, y=16";

        let sensor = input.parse::<Sensor>().unwrap();
        let expected = Sensor {
            loc: Vector2(12, 14),
            beacon_loc: Vector2(10, 16),
            range: 4,
        };

        assert_eq!(sensor, expected);
    }

    #[test]
    fn test_task_1() {
        let input = include_str!("input/day15_example.txt");

        assert_eq!(task1(input, 10).unwrap(), 26);
    }
}