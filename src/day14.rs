// Template

use std::{
    collections::HashMap,
    fmt::{Display, Write},
    hash::Hash,
};

use crate::{prelude::*, vector::Vector2};

pub fn run() -> Result<()> {
    let input = include_str!("input/day14_example.txt");

    println!("{}", task1(input)?);

    println!("{}", task2(input)?);

    Ok(())
}

fn task1(input: &str) -> Result<usize> {
    let topology = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|part| {
                    let (x, y) = part.split_once(',').unwrap();
                    Vector2(x.parse().unwrap(), y.parse().unwrap())
                })
                .collect_vec()
        })
        .collect_vec();

    let cave = Cave::from_topology(topology);

    println!("{cave}");

    todo!()
}

fn task2(input: &str) -> Result<usize> {
    todo!()
}

struct Cave {
    map: HashMap<Vector2, Thing>,
    min_x: isize,
    max_x: isize,
    max_y: isize,
}

impl Cave {
    fn from_topology(top: Vec<Vec<Vector2>>) -> Self {
        let mut map = HashMap::new();

        top.iter().for_each(|segment| {
            segment
                .windows(2)
                .flat_map(|vs| vs[0].line_to(&vs[1]))
                .for_each(|v| {
                    map.insert(v, Thing::Rock);
                });
        });

        let (min_x, max_x, max_y) =
            map.keys()
                .fold((isize::MAX, isize::MIN, isize::MIN), |mut acc, v| {
                    if v.0 < acc.0 {
                        // X is smaller than the minimum
                        acc.0 = v.0
                    }

                    if v.0 > acc.1 {
                        // X is larger than the maximum
                        acc.1 = v.0
                    }

                    if v.1 > acc.2 {
                        // Y is larger than the maximum
                        acc.2 = v.1
                    }

                    acc
                });

        Self {
            map,
            min_x,
            max_x,
            max_y,
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "min_x: {}, max_x: {}, max_y: {}",
            self.min_x, self.max_x, self.max_y
        )
        .unwrap();

        for y in 0..=self.max_y {
            for x in self.min_x..self.max_x {
                let c = self
                    .map
                    .get(&Vector2(x, y))
                    .map(|v| v.char())
                    .unwrap_or('.');

                f.write_char(c).unwrap();
            }
            writeln!(f).unwrap()
        }

        Ok(())
    }
}

enum Thing {
    Rock,
    Sand,
}

impl Thing {
    fn char(&self) -> char {
        match self {
            Thing::Rock => '#',
            Thing::Sand => 'o',
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_task_1() {
        let input = include_str!("input/day14_example.txt");
        assert_eq!(task1(input).unwrap(), 24);
    }
}
