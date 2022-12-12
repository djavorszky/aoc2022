use pathfinding::prelude::dijkstra;

use crate::{
    prelude::*,
    vector::{self, Vector2},
};

pub fn run() -> Result<()> {
    let input = include_str!("input/day12.txt");

    println!("{}", task1(input)?);

    println!("{}", task2(input)?);

    Ok(())
}

fn task1(input: &str) -> Result<usize> {
    let height_map = input
        .lines()
        .map(|line| line.chars().map(height).collect_vec())
        .collect_vec();

    let width = height_map[0].len();

    let input = input.replace('\n', "");

    let start = input
        .find('S')
        .map(|idx| Vector2::from_idx(idx, width))
        .ok_or_else(|| anyhow!("Can't find start"))?;

    let end = input
        .find('E')
        .map(|idx| Vector2::from_idx(idx, width))
        .ok_or_else(|| anyhow!("Can't find end"))?;

    let map = Map::new(height_map);

    let result = dijkstra(
        &start,
        |p| map.neighbours(p).into_iter().map(|p| (p, 1)),
        |p| *p == end,
    )
    .ok_or_else(|| anyhow!("Path not found"))?;

    Ok(result.1)
}

fn task2(input: &str) -> Result<usize> {
    let height_map = input
        .lines()
        .map(|line| line.chars().map(height).collect_vec())
        .collect_vec();

    let width = height_map[0].len();

    let input = input.replace('\n', "");

    let starting_points = input
        .chars()
        .enumerate()
        .filter_map(|(idx, c)| if c == 'a' { Some(idx) } else { None })
        .map(|idx| Vector2::from_idx(idx, width))
        .collect_vec();

    let end = input
        .find('E')
        .map(|idx| Vector2::from_idx(idx, width))
        .ok_or_else(|| anyhow!("Can't find end"))?;

    let map = Map::new(height_map);

    let result = starting_points
        .into_iter()
        .filter_map(|p| {
            dijkstra(
                &p,
                |p| map.neighbours(p).into_iter().map(|p| (p, 1)),
                |p| *p == end,
            )
        })
        .map(|(_, cost)| cost)
        .min()
        .ok_or_else(|| anyhow!("No minimum path found"))?;

    Ok(result)
}

fn height(c: char) -> usize {
    match c {
        'S' => 0,
        'E' => 25,
        c => c as usize - 'a' as usize,
    }
}

struct Map {
    height_map: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(height_map: Vec<Vec<usize>>) -> Self {
        let width = height_map[0].len();
        let height = height_map.len();
        Self {
            height,
            width,
            height_map,
        }
    }

    fn in_bounds(&self, pos: &Vector2) -> bool {
        (0..self.width as isize).contains(&pos.0) && (0..self.height as isize).contains(&pos.1)
    }

    fn neighbours(&self, pos: &Vector2) -> Vec<Vector2> {
        let height = self.height_at(pos);
        [vector::NORTH, vector::EAST, vector::SOUTH, vector::WEST]
            .iter()
            .map(|v| pos + v)
            .filter(|new_pos| self.in_bounds(new_pos))
            .filter(|new_pos| height + 1 >= self.height_at(new_pos))
            .collect_vec()
    }

    fn height_at(&self, pos: &Vector2) -> usize {
        self.height_map[pos.1 as usize][pos.0 as usize]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_task1() {
        let input = include_str!("input/day12_example.txt");
        assert_eq!(task1(input).unwrap(), 31);
    }

    #[test]
    fn test_task2() {
        let input = include_str!("input/day12_example.txt");
        assert_eq!(task2(input).unwrap(), 29);
    }
}
