use crate::prelude::ConstChunkIterator;

pub fn run() {
    let input = include_str!("input/day3.txt");

    println!("{}", task1(input));

    println!("{}", task2(input));
}

fn task1(input: &str) -> usize {
    let iter: ConstChunkIterator<2> = ConstChunkIterator::new(input);

    for x in iter {
        println!("{x:?}")
    }

    0
}

fn task2(input: &str) -> usize {
    0
}

fn priority(c: char) -> usize {
    todo!()
}
