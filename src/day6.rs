use crate::prelude::*;

pub fn run() -> Result<()> {
    let input = include_str!("input/day6.txt");

    println!("{}", task1(input)?);

    println!("{}", task2(input)?);

    Ok(())
}

pub fn task1(input: &str) -> Result<usize> {
    input
        .find_unique_span_idx::<4>()
        .ok_or_else(|| anyhow!("Did not find unique span size"))
}

pub fn task2(input: &str) -> Result<usize> {
    input
        .find_unique_span_idx::<14>()
        .ok_or_else(|| anyhow!("Did not find unique span size"))
}

trait UniqueSpan {
    fn find_unique_span_idx<const N: usize>(&self) -> Option<usize>;
}

impl UniqueSpan for &str {
    fn find_unique_span_idx<const N: usize>(&self) -> Option<usize> {
        if self.len() < N {
            return None;
        }

        let mut arr = [' '; N];

        for idx in 0..self.len() - N {
            let end_idx = idx + N;
            self[idx..end_idx]
                .chars()
                .enumerate()
                .for_each(|(idx, c)| arr[idx] = c);
            arr.sort();

            if arr.windows(2).filter(|cs| cs[0] == cs[1]).count() == 0 {
                return Some(end_idx);
            }
        }

        None
    }
}
