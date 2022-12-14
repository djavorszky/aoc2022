use std::{
    cmp::{max, min},
    ops::{Add, Sub},
};

use crate::prelude::*;
use itertools::Itertools;

pub const NORTH: Vector2 = Vector2(0, 1);
pub const NORTH_EAST: Vector2 = Vector2(1, 1);
pub const EAST: Vector2 = Vector2(1, 0);
pub const SOUTH_EAST: Vector2 = Vector2(1, -1);
pub const SOUTH: Vector2 = Vector2(0, -1);
pub const SOUTH_WEST: Vector2 = Vector2(-1, -1);
pub const WEST: Vector2 = Vector2(-1, 0);
pub const NORTH_WEST: Vector2 = Vector2(-1, 1);

pub const ZERO: Vector2 = Vector2(0, 0);

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy)]
pub struct Vector2(pub isize, pub isize);

impl Vector2 {
    pub fn manhattan_distance(&self, other: &Vector2) -> isize {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }

    pub fn clamp(&self, min: isize, max: isize) -> Self {
        Self(self.0.clamp(min, max), self.1.clamp(min, max))
    }

    pub fn abs(&self) -> Self {
        Self(self.0.abs(), self.1.abs())
    }

    pub fn touching(&self, other: &Vector2) -> bool {
        (other - self).len() < 2.0
    }

    pub fn len(&self) -> f32 {
        let s = self.abs();
        ((s.0 * s.0 + s.1 * s.1) as f32).sqrt()
    }

    pub fn move_towards(&self, other: &Vector2) -> Vector2 {
        self + &(other - self).clamp(-1, 1)
    }

    pub fn from_idx(idx: usize, width: usize) -> Vector2 {
        let x = (idx % width) as isize;
        let y = (idx / width) as isize;

        Vector2(x, y)
    }

    pub fn line_to(&self, other: &Vector2) -> Vec<Vector2> {
        if self.0 == other.0 {
            // vertical line
            let (min, max) = (min(self.1, other.1), max(self.1, other.1));
            (min..=max).map(|y| Vector2(self.0, y)).collect_vec()
        } else {
            // horizontal line
            let (min, max) = (min(self.0, other.0), max(self.0, other.0));
            (min..=max).map(|x| Vector2(x, self.1)).collect_vec()
        }
    }
}

impl Add<&Vector2> for &Vector2 {
    type Output = Vector2;

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

impl TryFrom<(&str, &str)> for Vector2 {
    type Error = Error;

    fn try_from((v1, v2): (&str, &str)) -> Result<Self> {
        let v1 = v1.parse::<isize>()?;
        let v2 = v2.parse::<isize>()?;

        Ok(Self(v1, v2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case(Vector2(4, 0), Vector2(1, 0), Vector2(3, 0))]
    #[test_case(Vector2(1, 0), Vector2(3, 0), Vector2(-2, 0))]
    #[test_case(Vector2(5, 2), Vector2(3, 1), Vector2(2, 1))]
    fn test_sub_vec(first: Vector2, second: Vector2, expected: Vector2) {
        let res = &first - &second;

        assert_eq!(res, expected);
    }

    #[test_case(Vector2(4, 0), Vector2(1, 0), 3)]
    #[test_case(Vector2(2, 1), Vector2(3, 0), 2)]
    #[test_case(Vector2(5, 2), Vector2(3, 1), 3)]
    fn test_manhattan_distance(first: Vector2, second: Vector2, expected: isize) {
        let res = first.manhattan_distance(&second);

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
