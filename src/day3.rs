use crate::prelude::ChunkLineIterator;

pub fn run() {
    let input = include_str!("input/day3.txt");

    println!("{}", task1(input));

    println!("{}", task2(input));
}

fn task1(input: &str) -> usize {
    let what = ChunkLineIterator::new(input, 2);

    for x in what {
        println!("Chunk: {x:?}")
    }

    0
}

fn task2(input: &str) -> usize {
    0
}

fn priority(c: char) -> usize {
    todo!()
}
