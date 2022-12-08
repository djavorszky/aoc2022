use std::collections::HashSet;

use crate::prelude::*;

pub fn run() -> Result<()> {
    let input = include_str!("input/day6.txt");

    println!("{}", task1(input)?);

    println!("{}", task2(input)?);

    Ok(())
}

pub fn task1(input: &str) -> Result<usize> {
    input
        .find_unique_span_idx(4)
        .ok_or_else(|| anyhow!("Did not find unique span size"))
}

pub fn task2(input: &str) -> Result<usize> {
    input
        .find_unique_span_idx(14)
        .ok_or_else(|| anyhow!("Did not find unique span size"))
}

trait UniqueSpan {
    fn find_unique_span_idx(&self, span_size: usize) -> Option<usize>;
}

impl UniqueSpan for &str {
    fn find_unique_span_idx(&self, span_size: usize) -> Option<usize> {
        if self.len() < span_size {
            return None;
        }

        for idx in 0..self.len() - span_size {
            let end_idx = idx + span_size;
            let set = self[idx..end_idx].chars().collect::<HashSet<_>>();
            if set.len() == span_size {
                return Some(end_idx);
            }
        }

        None
    }
}
