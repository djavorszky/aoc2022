use crate::prelude::*;

pub fn run() -> Result<()> {
    let input = include_str!("input/day8.txt");

    println!("{}", task1(input)?);

    println!("{}", task2(input)?);

    Ok(())
}

fn task1(input: &str) -> Result<usize> {
    let matrix = to_matrix(input);

    let by_rows = matrix
        .iter()
        .skip(1)
        .take(matrix.len() - 2)
        .map(|trees| count_trees(trees))
        .sum::<usize>();

    let width = matrix.first().map(|f| f.len()).unwrap();

    let transposed = transpose(matrix);

    let by_cols = transposed
        .iter()
        .skip(1)
        .take(transposed.len() - 2)
        .map(|trees| count_trees(trees))
        .sum::<usize>();

    let height = transposed.first().map(|f| f.len()).unwrap();

    let edge = (2 * width + 2 * height - 4);

    dbg!(width, height);
    dbg!(edge);

    Ok(edge + by_rows + by_cols)
}

fn task2(input: &str) -> Result<usize> {
    todo!()
}

fn count_trees(trees: &[usize]) -> usize {
    // todo: 1232241 - current algo finds only "123".
    // todo: need to find 1234 (because 4 is visible above 224)
    trees
        .iter()
        .tuple_windows()
        .take_while(|(n1, n2)| n2 > n1)
        .count()
        + trees
            .iter()
            .rev()
            .tuple_windows()
            .take_while(|(n1, n2)| n2 > n1)
            .count()
}

fn to_matrix(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

/// from https://stackoverflow.com/a/64499219/5664341
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn transpose_works() {
        let input = vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3]];
        let expected = vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]];

        assert_eq!(transpose(input), expected);
    }

    #[test]
    fn test_task_1() {
        let input = include_str!("input/day8_example.txt");

        assert_eq!(task1(input).unwrap(), 21)
    }
}
