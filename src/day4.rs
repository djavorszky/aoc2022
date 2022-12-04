use std::{ops::RangeInclusive, str::FromStr};

use crate::prelude::*;

pub fn run() -> Result<()> {
    let input = include_str!("input/day4.txt");

    println!("{}", task1(input));

    println!("{}", task2(input));

    Ok(())
}

fn task1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<SectionsPair>().expect("Couldn't parse line"))
        .filter(|s| s.is_subset_sections())
        .count()
}

fn task2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<SectionsPair>().expect("Couldn't parse line"))
        .filter(|s| s.is_overlapping_sections())
        .count()
}

struct SectionsPair {
    left: Sections,
    right: Sections,
}

impl FromStr for SectionsPair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (left, right) = s.split_once(',').context("No comma found")?;

        let left = left.parse::<Sections>()?;
        let right = right.parse::<Sections>()?;

        Ok(SectionsPair { left, right })
    }
}

impl SectionsPair {
    fn is_overlapping_sections(&self) -> bool {
        self.left.overlaps(&self.right) || self.right.overlaps(&self.left)
    }

    fn is_subset_sections(&self) -> bool {
        self.left.contains(&self.right) || self.right.contains(&self.left)
    }
}

struct Sections {
    range: RangeInclusive<usize>,
}

impl FromStr for Sections {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        s.split_once('-')
            .map(|(min, max)| {
                let min = min.parse()?;
                let max = max.parse()?;

                Ok(Self { range: (min..=max) })
            })
            .context(format!("Failed parsing section: {s}"))?
    }
}

impl Sections {
    fn contains(&self, other: &Sections) -> bool {
        self.range.contains(other.range.start()) && self.range.contains(other.range.end())
    }

    fn overlaps(&self, other: &Sections) -> bool {
        self.range.contains(other.range.start()) || self.range.contains(other.range.end())
    }
}
