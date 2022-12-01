pub fn run() {
    let input = include_str!("input/day1.txt");

    println!("{}", task1(input));

    println!("{}", task2(input));
}

fn task1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|rows| {
            rows.lines()
                .map(|l| l.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap()
}

fn task2(input: &str) -> usize {
    let mut calories = input
        .split("\n\n")
        .map(|rows| {
            rows.lines()
                .map(|l| l.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>();

    calories.sort_unstable();

    calories.into_iter().rev().take(3).sum::<usize>()
}
