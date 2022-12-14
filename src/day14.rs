use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

use crate::{prelude::*, vector::Vector2};

pub fn run() -> Result<()> {
    let input = include_str!("input/day14.txt");

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

    let mut cave = InfiniteCave::from_topology(topology);

    cave.start_simulation();

    println!("{cave}");

    Ok(cave
        .map
        .values()
        .filter(|v| matches!(v, Thing::Sand))
        .count())
}

fn task2(input: &str) -> Result<usize> {
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

    let mut cave = BoundedCave::from_topology(topology);

    cave.start_simulation();

    Ok(cave
        .map
        .values()
        .filter(|v| matches!(v, Thing::Sand))
        .count())
}

trait Cave {
    fn is_blocked(&self, loc: Vector2) -> bool;
    fn is_out_of_bounds(&self, loc: &Vector2) -> bool;

    fn can_fall(&self, loc: &Vector2, direction: &Vector2) -> bool {
        !self.is_blocked(loc + direction)
    }
}

struct BoundedCave {
    map: HashMap<Vector2, Thing>,
    floor_y: isize,
}

impl Cave for BoundedCave {
    fn is_blocked(&self, loc: Vector2) -> bool {
        self.map.contains_key(&loc) || loc.1 == self.floor_y
    }

    fn is_out_of_bounds(&self, _loc: &Vector2) -> bool {
        false
    }
}

impl BoundedCave {
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

        let floor_y = map.keys().map(|v| v.1).max().map(|v| v + 2).unwrap();

        Self { map, floor_y }
    }

    fn start_simulation(&mut self) {
        loop {
            let sand = Vector2(500, 0);
            if self.is_blocked(sand) {
                return;
            }

            self.simulate(sand);
        }
    }

    fn simulate(&mut self, mut sand: Vector2) -> SandResult {
        while self.can_fall(&sand, &DOWN) {
            sand = &sand + &DOWN;
        }

        if self.can_fall(&sand, &LEFT) {
            return self.simulate(&sand + &LEFT);
        } else if self.can_fall(&sand, &RIGHT) {
            return self.simulate(&sand + &RIGHT);
        }

        self.map.insert(sand, Thing::Sand);

        SandResult::Settled
    }
}

struct InfiniteCave {
    map: HashMap<Vector2, Thing>,
    min_x: isize,
    max_x: isize,
    max_y: isize,
}

impl Cave for InfiniteCave {
    fn is_blocked(&self, loc: Vector2) -> bool {
        self.map.contains_key(&loc)
    }

    fn is_out_of_bounds(&self, loc: &Vector2) -> bool {
        loc.0 < self.min_x || loc.0 > self.max_x || loc.1 > self.max_y
    }
}

impl InfiniteCave {
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

    fn start_simulation(&mut self) {
        loop {
            let sand = Vector2(500, 0);

            let res = self.simulate(sand);
            if matches!(res, SandResult::Fellthrough) {
                return;
            }
        }
    }

    fn simulate(&mut self, mut sand: Vector2) -> SandResult {
        while self.can_fall(&sand, &DOWN) {
            if self.is_out_of_bounds(&sand) {
                return SandResult::Fellthrough;
            }

            sand = &sand + &DOWN;
        }

        if self.can_fall(&sand, &LEFT) {
            return self.simulate(&sand + &LEFT);
        } else if self.can_fall(&sand, &RIGHT) {
            return self.simulate(&sand + &RIGHT);
        }

        self.map.insert(sand, Thing::Sand);

        SandResult::Settled
    }
}

const DOWN: Vector2 = Vector2(0, 1);
const LEFT: Vector2 = Vector2(-1, 1);
const RIGHT: Vector2 = Vector2(1, 1);

impl Display for InfiniteCave {
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

enum SandResult {
    Settled,
    Fellthrough,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_task_1() {
        let input = include_str!("input/day14_example.txt");
        assert_eq!(task1(input).unwrap(), 24);
    }

    #[test]
    fn test_task_2() {
        let input = include_str!("input/day14_example.txt");
        assert_eq!(task2(input).unwrap(), 93);
    }
}
