use std::str::FromStr;

pub fn run() {
    let input = include_str!("input/day2.txt");

    println!("{}", task1(input));

    println!("{}", task2(input));
}

fn task1(input: &str) -> usize {
    input.lines().map(Play::from).map(|p| p.score()).sum()
}

fn task2(input: &str) -> usize {
    input
        .lines()
        .map(Play::from_expected)
        .map(|p| p.score())
        .sum()
}

struct Play(Hand, Hand);

impl Play {
    fn from(line: &str) -> Self {
        line.split_once(' ')
            .map(|(them, us)| Play(them.into(), us.into()))
            .expect("What kinda input is that")
    }

    fn from_expected(line: &str) -> Self {
        line.split_once(' ')
            .map(|(them, expected)| {
                let them = them.into();

                let us = match expected {
                    "X" => match them {
                        Hand::Rock => Hand::Scissors,
                        Hand::Scissors => Hand::Paper,
                        Hand::Paper => Hand::Rock,
                    }, //lose
                    "Y" => them.clone(),
                    "Z" => match them {
                        Hand::Rock => Hand::Paper,
                        Hand::Scissors => Hand::Rock,
                        Hand::Paper => Hand::Scissors,
                    }, //lose,
                    _ => panic!("haha business"),
                };

                Play(them, us)
            })
            .unwrap()
    }

    fn score(&self) -> usize {
        self.1.score()
            + match (&self.0, &self.1) {
                (Hand::Rock, Hand::Rock)
                | (Hand::Paper, Hand::Paper)
                | (Hand::Scissors, Hand::Scissors) => 3,
                (Hand::Rock, Hand::Paper)
                | (Hand::Paper, Hand::Scissors)
                | (Hand::Scissors, Hand::Rock) => 6,
                _ => 0,
            }
    }
}

#[derive(Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Don't understand this"),
        }
    }
}

impl Hand {
    fn score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}
