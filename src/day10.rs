use std::{collections::BTreeMap, str::FromStr};

use crate::prelude::*;

pub fn run() -> Result<()> {
    let input = include_str!("input/day10.txt");

    println!("{}", task1(input)?);

    println!("{}", task2(input)?);

    Ok(())
}

fn task1(input: &str) -> Result<isize> {
    let mut cpu = Cpu::new();

    let signals: Vec<Signal> = input
        .lines()
        .map(|line| line.parse::<Signal>())
        .collect::<Result<Vec<Signal>>>()?;

    signals.iter().for_each(|s| cpu.process(s));

    Ok([20, 60, 100, 140, 180, 220]
        .iter()
        .map(|v| cpu.signal(*v) * v)
        .sum())
}

fn task2(input: &str) -> Result<String> {
    let mut cpu = Cpu::new();

    let signals: Vec<Signal> = input
        .lines()
        .map(|line| line.parse::<Signal>())
        .collect::<Result<Vec<Signal>>>()?;

    signals.iter().for_each(|s| cpu.process(s));

    Ok((0..6)
        .map(|row| {
            (0..40)
                .map(|col| {
                    let cycle = row * 40 + col + 1;
                    let signal = cpu.signal(cycle);

                    if (signal - col).abs() < 2 {
                        '#'
                    } else {
                        '.'
                    }
                })
                .join("")
        })
        .join("\n"))
}

struct Cpu {
    register_history: BTreeMap<isize, isize>,
    cycle: isize,
}

impl Cpu {
    fn new() -> Self {
        Self {
            register_history: BTreeMap::from([(0, 1)]),
            cycle: 0,
        }
    }
}

impl Cpu {
    fn process(&mut self, signal: &Signal) {
        match signal {
            Signal::Noop => self.cycle += 1,
            Signal::AddX(x) => {
                self.cycle += 2;
                self.register_history.insert(self.cycle + 1, *x);
            }
        }
    }

    fn signal(&self, cycle: isize) -> isize {
        let mut register_value: isize = 0;

        for (registered_cycle, change) in self.register_history.iter() {
            if *registered_cycle > cycle {
                return register_value;
            }

            register_value += change
        }

        register_value
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Signal {
    Noop,
    AddX(isize),
}

impl FromStr for Signal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == "noop" {
            return Ok(Signal::Noop);
        }

        let (_, value) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("Can't parse value: {s}"))?;

        let value = value.parse::<isize>()?;

        Ok(Signal::AddX(value))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use test_case::test_case;

    #[test_case("noop", Signal::Noop)]
    #[test_case("addx 5", Signal::AddX(5))]
    fn signal_processing(input: &str, expected: Signal) {
        assert_eq!(input.parse::<Signal>().unwrap(), expected);
    }

    #[test]
    fn test_task_1() {
        let input = include_str!("input/day10_example.txt");

        assert_eq!(task1(input).unwrap(), 13140);
    }

    #[test]
    fn test_task_2() {
        let input = include_str!("input/day10_example.txt");
        let expected = include_str!("input/day10_example_task2.txt");

        assert_eq!(task2(input).unwrap(), expected)
    }
}
