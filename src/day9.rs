use std::{
    collections::HashSet,
    ops::{Add, Sub},
    str::FromStr,
};

use crate::prelude::*;

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
    todo!()
}

const NORTH: Vector2 = Vector2(0, 1);
const EAST: Vector2 = Vector2(1, 0);
const SOUTH: Vector2 = Vector2(0, -1);
const WEST: Vector2 = Vector2(-1, 0);

const ZERO: Vector2 = Vector2(0, 0);

#[derive(Debug)]
struct World {
    touched: HashSet<Vector2>,
    head: Vector2,
    tail: Vector2,
}

impl World {
    fn new() -> Self {
        Self {
            touched: HashSet::from([ZERO]),
            head: ZERO,
            tail: ZERO,
        }
    }

    fn tick(&mut self, i: Instruction) {
        (0..i.amount).for_each(|_| {
            self.head = self.head + &i.direction;

            if !self.tail.touching(&self.head) {
                self.tail = self.tail.move_towards(&self.head);
                self.touched.insert(self.tail);
            }

            // dbg!(&self.head, self.tail);
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
            "U" => NORTH,
            "D" => SOUTH,
            "L" => WEST,
            "R" => EAST,
            _ => bail!("Unknown direction {first}"),
        };

        Ok(Self { amount, direction })
    }
}

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy)]
struct Vector2(isize, isize);

impl Vector2 {
    fn clamp(&self, min: isize, max: isize) -> Self {
        Self(self.0.clamp(min, max), self.1.clamp(min, max))
    }

    fn abs(&self) -> Self {
        Self(self.0.abs(), self.1.abs())
    }

    fn touching(&self, other: &Vector2) -> bool {
        (other - self).len() < 2.0
    }

    fn len(&self) -> f32 {
        let s = self.abs();
        ((s.0 * s.0 + s.1 * s.1) as f32).sqrt()
    }

    fn move_towards(&self, other: &Vector2) -> Vector2 {
        *self + &(other - self).clamp(-1, 1)
    }
}

impl Add<&Vector2> for Vector2 {
    type Output = Self;

    fn add(self, rhs: &Vector2) -> Self::Output {
        Vector2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<&Vector2> for &Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: &Vector2) -> Self::Output {
        Vector2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;

    #[test]
    fn test_task_1() {
        let input = include_str!("input/day9_example.txt");

        assert_eq!(task1(input).unwrap(), 13);
    }

    #[test_case(Vector2(4, 0), Vector2(1, 0), Vector2(3, 0))]
    #[test_case(Vector2(1, 0), Vector2(3, 0), Vector2(-2, 0))]
    #[test_case(Vector2(5, 2), Vector2(3, 1), Vector2(2, 1))]
    fn test_sub_vec(first: Vector2, second: Vector2, expected: Vector2) {
        let res = &first - &second;

        assert_eq!(res, expected);
    }

    #[test]
    fn test_vec_touching_immediate() {
        (-1..1)
            .cartesian_product(-1..1)
            .for_each(|(x, y)| assert!(ZERO.touching(&Vector2(x, y,))))
    }

    #[test_case(Vector2(4, 0), Vector2(1, 0), false)]
    #[test_case(Vector2(3, 0), Vector2(4, 3), false)]
    #[test_case(Vector2(3, 3), Vector2(3, 3), true)]
    #[test_case(Vector2(3, 3), Vector2(1, 1), false)]
    #[test_case(Vector2(3, 3), Vector2(-2, -1), false)]
    #[test_case(Vector2(3, 3), Vector2(4, -1), false)]
    #[test_case(ZERO, Vector2(2, 0), false)]
    #[test_case(Vector2(3, 0), Vector2(1, 0), false)]
    fn test_vec_touching(first: Vector2, second: Vector2, expected: bool) {
        assert_eq!(first.touching(&second), expected);
    }

    #[test_case(ZERO, Vector2(0, 2), Vector2(0, 1) ; "move n")]
    #[test_case(ZERO, Vector2(2, 0), Vector2(1, 0) ; "move e")]
    #[test_case(Vector2(1,0), Vector2(3, 0), Vector2(2, 0) ; "move e again")]
    #[test_case(ZERO, Vector2(0, -2), Vector2(0, -1) ; "move s")]
    #[test_case(ZERO, Vector2(-2, 0), Vector2(-1, 0) ; "move w")]
    fn test_move_towards_single(first: Vector2, second: Vector2, expected: Vector2) {
        assert_eq!(first.move_towards(&second), expected);
    }

    #[test_case(ZERO, Vector2(1, 2), Vector2(1, 1) ; "move ne")]
    #[test_case(ZERO, Vector2(1, -2), Vector2(1, -1); "move se")]
    #[test_case(ZERO, Vector2(-1, -2), Vector2(-1, -1) ; "move sw")]
    #[test_case(ZERO, Vector2(-1, 2), Vector2(-1, 1) ; "move nw")]
    fn test_move_towards_double(first: Vector2, second: Vector2, expected: Vector2) {
        assert_eq!(first.move_towards(&second), expected);
    }
}
