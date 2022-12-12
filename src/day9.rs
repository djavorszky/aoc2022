use std::{collections::HashSet, str::FromStr};

use crate::{
    prelude::*,
    vector::{self, Vector2},
};

pub fn run() -> Result<()> {
    let input = include_str!("input/day9.txt");

    println!("{}", task1(input)?);

    println!("{}", task2(input)?);

    Ok(())
}

fn task1(input: &str) -> Result<usize> {
    let mut world = World::new();

    for line in input.lines() {
        let instruction: Instruction = line.parse()?;

        world.tick(instruction);
    }

    Ok(world.touched.len())
}

fn task2(input: &str) -> Result<usize> {
    let mut world = LongerRopeWorld::new();

    for line in input.lines() {
        let instruction: Instruction = line.parse()?;

        world.tick(instruction);
    }

    Ok(world.touched.len())
}
struct LongerRopeWorld {
    touched: HashSet<Vector2>,
    rope: [Vector2; 10],
}

impl LongerRopeWorld {
    fn new() -> Self {
        Self {
            touched: HashSet::from([vector::ZERO]),
            rope: [vector::ZERO; 10],
        }
    }

    fn tick(&mut self, i: Instruction) {
        (0..i.amount).for_each(|_| {
            let mut new_rope = [vector::ZERO; 10];

            new_rope[0] = &self.rope[0] + &i.direction;

            self.rope
                .iter()
                .enumerate()
                .skip(1)
                .for_each(|(idx, knot)| {
                    let parent = new_rope[idx - 1];

                    new_rope[idx] = if !knot.touching(&parent) {
                        knot.move_towards(&parent)
                    } else {
                        *knot
                    }
                });
            self.touched.insert(new_rope[9]);

            self.rope = new_rope;
        });
    }
}

#[derive(Debug)]
struct World {
    touched: HashSet<Vector2>,
    head: Vector2,
    tail: Vector2,
}

impl World {
    fn new() -> Self {
        Self {
            touched: HashSet::from([vector::ZERO]),
            head: vector::ZERO,
            tail: vector::ZERO,
        }
    }

    fn tick(&mut self, i: Instruction) {
        (0..i.amount).for_each(|_| {
            self.head = &self.head + &i.direction;

            if !self.tail.touching(&self.head) {
                self.tail = self.tail.move_towards(&self.head);
                self.touched.insert(self.tail);
            }
        })
    }
}

struct Instruction {
    direction: Vector2,
    amount: usize,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (first, second) = s.split_once(' ').ok_or_else(|| anyhow!("can't parse"))?;

        let amount = second.parse::<usize>()?;

        let direction = match first {
            "U" => vector::NORTH,
            "D" => vector::SOUTH,
            "L" => vector::WEST,
            "R" => vector::EAST,
            _ => bail!("Unknown direction {first}"),
        };

        Ok(Self { amount, direction })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_task_1() {
        let input = include_str!("input/day9_example.txt");

        assert_eq!(task1(input).unwrap(), 13);
    }

    #[test]
    fn test_task_2_small() {
        let input = include_str!("input/day9_example.txt");

        assert_eq!(task2(input).unwrap(), 1);
    }

    #[test]
    fn test_task_2_large() {
        let input = include_str!("input/day9_example_large.txt");

        assert_eq!(task2(input).unwrap(), 36);
    }
}
