use itertools::Itertools;
use std::collections::HashSet;

pub fn run() {
    let input = include_str!("input/day3.txt");

    println!("{}", task1(input));

    println!("{}", task2(input));
}

fn task1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(c1, c2)| {
            let bp: HashSet<_> = c1.chars().collect();

            c2.chars().find(|c| bp.contains(c)).map(score).unwrap()
        })
        .sum()
}

fn score(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 0x60,
        'A'..='Z' => 26 + c as usize - 0x40,
        _ => panic!("it ain't ascii char: {c}"),
    }
}

fn task2(input: &str) -> usize {
    input
        .lines()
        .map(|line| HashSet::from_iter(line.chars()))
        .chunks(3)
        .into_iter()
        .map(|bags| {
            bags.into_iter()
                .reduce(|acc, next| acc.intersection(&next).cloned().collect::<HashSet<_>>())
                .map(|set| set.into_iter().next().unwrap())
                .map(score)
                .unwrap()
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_example_test() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(task1(input), 157)
    }

    #[test]
    fn task_2_example_test() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(task2(input), 70)
    }

    #[test]
    fn test_score() {
        ('a'..='z').enumerate().for_each(|(idx, c)| {
            assert_eq!(score(c), idx + 1);
        });

        ('A'..='Z').enumerate().for_each(|(idx, c)| {
            assert_eq!(score(c), idx + 27);
        });
    }
}
