use crate::prelude::*;

pub fn run() -> Result<()> {
    let input = include_str!("input/day8.txt");

    println!("{}", task1(input)?);

    println!("{}", task2(input)?);

    Ok(())
}

fn task1(input: &str) -> Result<usize> {
    let matrix = to_matrix(input);
    let len = matrix.len();

    let edges = len * 4 - 4;

    let visible_trees = (1..len - 1)
        .cartesian_product(1..len - 1)
        .filter(|(row, col)| {
            visible_horizontal(&matrix, *row, *col) || visible_vertical(&matrix, *row, *col)
        })
        .count();

    Ok(visible_trees + edges)
}

fn visible_vertical(matrix: &[Vec<usize>], row: usize, col: usize) -> bool {
    let tree = &matrix[row][col];

    let col: Vec<&usize> = (0..matrix.len()).map(|idx| &matrix[idx][col]).collect();

    let visible_up = col[0..row].iter().all(|t| t < &tree);
    let visible_down = col[row + 1..col.len()].iter().all(|t| t < &tree);

    visible_up || visible_down
}

fn visible_horizontal(matrix: &[Vec<usize>], row: usize, col: usize) -> bool {
    let row = &matrix[row];

    let tree = row[col];

    let visible_left = row[0..col].iter().all(|t| t < &tree);
    let visible_right = row[col + 1..row.len()].iter().all(|t| t < &tree);

    visible_left || visible_right
}

fn task2(input: &str) -> Result<usize> {
    todo!()
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_task_1() {
        let input = include_str!("input/day8_example.txt");

        assert_eq!(task1(input).unwrap(), 21)
    }
}
