use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

use crate::prelude::*;

pub fn run() -> Result<()> {
    let input = include_str!("input/day5.txt");

    println!("{}", task1(input)?);

    println!("{}", task2(input)?);

    Ok(())
}

fn task1(input: &str) -> Result<String> {
    let (crates, instructions) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("malformed input"))?;

    let mut port: CargoPort = crates
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .try_into()?;

    let instructions: Vec<Instruction> = instructions
        .lines()
        .map(|line| line.parse::<Instruction>())
        .collect::<Result<Vec<Instruction>>>()?;

    instructions.into_iter().for_each(|i| port.apply(i));

    Ok(port.top_crates())
}

fn task2(input: &str) -> Result<usize> {
    todo!()
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct Crate(char);

#[derive(Debug, PartialEq, Eq)]
struct CargoPort {
    sections: Vec<Vec<Crate>>,
}

impl CargoPort {
    fn with_capacity(cap: usize) -> Self {
        Self {
            sections: vec![vec![]; cap],
        }
    }

    fn apply(&mut self, instruction: Instruction) {
        (0..instruction.amount).for_each(|_| {
            let c = self.sections[instruction.from].pop().unwrap();
            self.sections[instruction.to].push(c);
        });
    }

    fn top_crates(&self) -> String {
        self.sections
            .iter()
            .map(|s| s.last().unwrap().0)
            .collect::<String>()
    }
}

impl Index<usize> for CargoPort {
    type Output = Vec<Crate>;

    fn index(&self, index: usize) -> &Self::Output {
        self.sections.index(index)
    }
}

impl IndexMut<usize> for CargoPort {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.sections.index_mut(index)
    }
}

impl TryFrom<Vec<String>> for CargoPort {
    type Error = Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let len = value[0].len();
        let mut port = CargoPort::with_capacity(len / 4 + 1);

        for line in value {
            line.chars()
                .chunks(4)
                .into_iter()
                .map(|cs| {
                    cs.filter(|c| *c != ' ' && !c.is_ascii_digit())
                        .collect::<String>()
                })
                .enumerate()
                .filter(|(_, s)| !s.is_empty())
                .map(|(idx, s)| (idx, Crate(s.chars().nth(1).unwrap())))
                .for_each(|(idx, c)| port[idx].push(c));
        }

        port.sections.iter_mut().for_each(|s| s.reverse());

        Ok(port)
    }
}

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(' ').filter_map(|p| p.parse::<usize>().ok());

        let err = || anyhow!("invalid length");

        Ok(Self {
            amount: parts.next().ok_or_else(err)?,
            from: parts.next().ok_or_else(err)? - 1,
            to: parts.next().ok_or_else(err)? - 1,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }
}
