use std::str::FromStr;

use crate::prelude::*;

pub fn run() -> Result<()> {
    let input = include_str!("input/day11.txt");

    println!("{}", task1(input)?);

    println!("{}", task2(input)?);

    Ok(())
}

enum Worry {
    Defo,
    Nope,
}

fn task1(input: &str) -> Result<usize> {
    play_rounds(input, 20, Worry::Nope)
}

fn task2(input: &str) -> Result<usize> {
    play_rounds(input, 10000, Worry::Defo)
}

fn play_rounds(input: &str, rounds: usize, worry: Worry) -> Result<usize> {
    let mut monkeys = input
        .split("\n\n")
        .map(|chunk| chunk.parse::<Monkey>())
        .collect::<Result<Vec<Monkey>>>()?;

    let mut items = monkeys.iter().map(|m| m.items.clone()).collect_vec();
    let divisor: usize = monkeys.iter().map(|m| m.division_value).product();
    for _ in 0..rounds {
        for (idx, monkey) in monkeys.iter_mut().enumerate() {
            let processed_items = monkey.process_items(&items[idx], &worry, divisor);

            items[idx].clear();

            for (idx, item) in processed_items {
                items[idx].push(item);
            }
        }
    }

    let mut inspect_counts: Vec<usize> = monkeys.iter().map(|m| m.inspect_count).collect();

    inspect_counts.sort_unstable();

    Ok(inspect_counts.iter().rev().take(2).product())
}

#[derive(Debug, Eq, PartialEq)]
enum Op {
    Mul(usize),
    Add(usize),
    Sqr,
}

impl Op {
    fn apply(&self, rhs: usize) -> usize {
        match self {
            Op::Mul(v) => v * rhs,
            Op::Add(v) => v + rhs,
            Op::Sqr => rhs * rhs,
        }
    }
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split_ascii_whitespace().skip(1).take(2).collect();

        if parts[1] == "old" {
            return Ok(Self::Sqr);
        }

        let v = parts[1].parse::<usize>()?;

        if parts[0] == "+" {
            Ok(Self::Add(v))
        } else {
            Ok(Self::Mul(v))
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Monkey {
    items: Vec<usize>,
    op: Op,
    division_value: usize,
    happy: usize,
    sad: usize,
    inspect_count: usize,
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines().skip(1);

        // items:
        let items: Vec<usize> = lines
            .next()
            .ok_or_else(|| anyhow!("Unexpected end of input"))?
            .split(": ")
            .nth(1)
            .ok_or_else(|| anyhow!("No starting items?"))?
            .split(", ")
            .map(|num| {
                num.parse::<usize>()
                    .map_err(|err| anyhow!("failed parsing {num}: {err}"))
            })
            .collect::<Result<Vec<usize>>>()?;

        // operation
        let op = lines
            .next()
            .ok_or_else(|| anyhow!("Unexpected end of input"))?
            .split("= ")
            .last()
            .ok_or_else(|| anyhow!("No op?"))?
            .parse::<Op>()?;

        // test
        let division_value = last_usize(
            lines
                .next()
                .ok_or_else(|| anyhow!("Unexpected end of input"))?,
        )?;

        // if happy
        let happy = last_usize(
            lines
                .next()
                .ok_or_else(|| anyhow!("Unexpected end of input"))?,
        )?;

        // if sad
        let sad = last_usize(
            lines
                .next()
                .ok_or_else(|| anyhow!("Unexpected end of input"))?,
        )?;

        Ok(Self {
            items,
            op,
            division_value,
            happy,
            sad,
            inspect_count: 0,
        })
    }
}

impl Monkey {
    fn process_item(&self, item: usize, worry: &Worry, divisor: usize) -> (usize, usize) {
        let x = {
            let tmp = self.op.apply(item);
            if matches!(worry, Worry::Nope) {
                tmp / 3
            } else {
                tmp
            }
        };

        if self.division_value == 0 {
            (self.happy, x % divisor)
        } else {
            (self.sad, x % divisor)
        }
    }

    fn process_items(
        &mut self,
        thrown_items: &[usize],
        worry: &Worry,
        divisor: usize,
    ) -> Vec<(usize, usize)> {
        let items = thrown_items
            .iter()
            .map(|i| self.process_item(*i, worry, divisor))
            .collect_vec();

        self.inspect_count += items.len();

        items
    }
}

fn last_usize(s: &str) -> Result<usize> {
    s.split_ascii_whitespace()
        .last()
        .ok_or_else(|| anyhow!("No test?"))?
        .parse::<usize>()
        .map_err(|err| anyhow!("failed getting usize: {err}"))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_task_1() {
        let input = include_str!("input/day11_example.txt");

        assert_eq!(task1(input).unwrap(), 10605);
    }

    #[test]
    fn test_task_2() {
        let input = include_str!("input/day11_example.txt");

        assert_eq!(task2(input).unwrap(), 2713310158);
    }
}
