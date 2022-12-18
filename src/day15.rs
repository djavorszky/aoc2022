use std::{ops::RangeInclusive, str::FromStr};

use crate::{prelude::*, vector::Vector2};

pub fn run() -> Result<()> {
    let input = include_str!("input/day15.txt");

    println!("{}", task1(input, 2000000)?);

    println!("{}", task2(input, 1..=4000000)?);

    Ok(())
}

fn task1(input: &str, line: isize) -> Result<usize> {
    let sensors = input
        .lines()
        .map(|line| line.parse::<Sensor>().unwrap())
        .collect_vec();

    let mut count = covered_ranges(&sensors, line)
        .into_iter()
        .flat_map(|rs| rs.into_iter().map(|r| r.count()).collect_vec())
        .sum();

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

fn task2(input: &str, range: RangeInclusive<isize>) -> Result<isize> {
    let sensors = input
        .lines()
        .map(|line| line.parse::<Sensor>().unwrap())
        .collect_vec();

    for line in range {
        if let Some(ranges) = covered_ranges(&sensors, line) {
            if ranges.len() > 1 {
                let first = ranges.first().unwrap();
                let x = first.clone().last().unwrap() + 1;
                return Ok(x * 4000000 + line);
            }
        }
    }

    bail!("Did not find");
}

fn covered_ranges(sensors: &[Sensor], line: isize) -> Option<Vec<RangeInclusive<isize>>> {
    let sensors_in_range = sensors.iter().filter(|s| s.in_range(line)).collect_vec();

    if sensors_in_range.is_empty() {
        return None;
    }

    let mut ranges = sensors_in_range
        .iter()
        .map(|sensor| {
            let num_covered = sensor.range - (sensor.loc.1 - line).abs();

            sensor.loc.0 - num_covered..=sensor.loc.0 + num_covered
        })
        .collect_vec();

    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut row_ranges: Vec<RangeInclusive<isize>> = Vec::new();
    let mut current_range = ranges[0].clone();

    for range in ranges.iter().skip(1) {
        if current_range.start() <= range.start() && current_range.end() >= range.end() {
            continue;
        }

        if range.start() - 1 <= *current_range.end() {
            current_range = *current_range.start()..=*range.end();
            continue;
        }

        row_ranges.push(current_range);
        current_range = range.clone();
    }

    if row_ranges.is_empty() || row_ranges.last().unwrap() != &current_range {
        row_ranges.push(current_range);
    }

    Some(row_ranges)
}

#[derive(PartialEq, Eq, Debug)]
struct Sensor {
    loc: Vector2,
    beacon_loc: Vector2,
    range: isize,
}

impl Sensor {
    fn in_range(&self, row: isize) -> bool {
        (self.loc.1 - row).abs() <= self.range
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
    fn sensor_in_range() {
        let s = Sensor {
            loc: Vector2(10, 10),
            beacon_loc: Vector2(10, 20),
            range: 10,
        };

        (0..=20).for_each(|row| assert!(s.in_range(row), "row {row}"));

        (21..30).for_each(|row| assert!(!s.in_range(row), "row {row}"));
        (-10..0).for_each(|row| assert!(!s.in_range(row), "row {row}"));
    }

    #[test]
    fn test_task_1() {
        let input = include_str!("input/day15_example.txt");

        assert_eq!(task1(input, 10).unwrap(), 26);
    }

    #[test]
    fn test_task_2() {
        let input = include_str!("input/day15_example.txt");

        assert_eq!(task2(input, 1..=20).unwrap(), 56000011);
    }
}
