use crate::prelude::*;

pub fn run() -> Result<()> {
    let input = include_str!("input/day12.txt");

    println!("{}", task1(input)?);

    println!("{}", task2(input)?);

    Ok(())
}

fn task1(input: &str) -> Result<usize> {
    todo!()
}

fn task2(input: &str) -> Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }
}
