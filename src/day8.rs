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
    let matrix = to_matrix(input);
    let len = matrix.len();

    (1..len - 1)
        .cartesian_product(1..len - 1)
        .map(|(row, col)| score_horizontal(&matrix, row, col) * score_vertical(&matrix, row, col))
        .max()
        .ok_or_else(|| anyhow!("No max for usizes: press X for doubt."))
}

fn score_vertical(matrix: &[Vec<usize>], row: usize, col: usize) -> usize {
    let tree = &matrix[row][col];

    let col: Vec<&usize> = (0..matrix.len()).map(|idx| &matrix[idx][col]).collect();

    let mut score_down = 0;
    for t in col.iter().skip(row + 1) {
        score_down += 1;
        if t >= &tree {
            break;
        }
    }
    let mut score_up = 0;
    for t in col.iter().take(row).rev() {
        score_up += 1;
        if t >= &tree {
            break;
        }
    }

    score_down * score_up
}

fn score_horizontal(matrix: &[Vec<usize>], row: usize, col: usize) -> usize {
    let row = &matrix[row];
    let tree = row[col];

    let mut score_right = 0;
    for t in row.iter().skip(col + 1) {
        score_right += 1;
        if t >= &tree {
            break;
        }
    }
    let mut score_left = 0;

    for t in row.iter().take(col).rev() {
        score_left += 1;
        if t >= &tree {
            break;
        }
    }

    score_left * score_right
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

    #[test]
    fn test_task_2() {
        let input = include_str!("input/day8_example.txt");

        assert_eq!(task2(input).unwrap(), 8)
    }

    #[test]
    fn test_score_for_tree() {
        let input = to_matrix(include_str!("input/day8_example.txt"));

        assert_eq!(score_vertical(&input, 3, 2), 2);
        assert_eq!(score_horizontal(&input, 3, 2), 4)
    }
}
