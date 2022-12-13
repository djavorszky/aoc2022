// Template

use std::cmp::Ordering;
use std::iter::zip;

use serde_json::Value::{Array, Number};
use serde_json::{json, Value};

use crate::prelude::*;

pub fn run() -> Result<()> {
    let input = include_str!("input/day13.txt");

    println!("{}", task1(input)?);

    println!("{}", task2(input)?);

    Ok(())
}

fn task1(input: &str) -> Result<usize> {
    let values: Vec<(Value, Value)> = input
        .replace("\n\n", "\n")
        .lines()
        .chunks(2)
        .into_iter()
        .map(|chunk| {
            chunk
                .map(|c| serde_json::from_str(c).unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    Ok(zip(values, 1..)
        .map(|(vals, idx)| (cmp_value(&vals.0, &vals.1), idx))
        .filter_map(|(o, idx)| {
            if matches!(o, Ordering::Less) {
                Some(idx)
            } else {
                None
            }
        })
        .sum())
}

fn task2(input: &str) -> Result<usize> {
    let mut values: Vec<Value> = input
        .replace("\n\n", "\n")
        .lines()
        .map(|line| serde_json::from_str(line).map_err(|_| anyhow!("can't parse as json")))
        .collect::<Result<Vec<Value>>>()?;

    let divider1 = json!([[2]]);
    let divider2 = json!([[6]]);

    values.push(divider1.clone());
    values.push(divider2.clone());

    values.sort_by(cmp_value);

    Ok(zip(values, 1..)
        .filter(|(v, _)| v == &divider1 || v == &divider2)
        .map(|(_, idx)| idx)
        .product())
}

fn cmp_value(v1: &Value, v2: &Value) -> Ordering {
    match (v1, v2) {
        (Number(l), Number(r)) => l.as_i64().unwrap().cmp(&r.as_i64().unwrap()),
        (Number(l), Array(_)) => cmp_value(&json!([l.as_i64().unwrap()]), v2),
        (Array(_), Number(r)) => cmp_value(v1, &json!([r.as_i64().unwrap()])),
        (Array(l), Array(r)) => zip(l, r)
            .map(|(l, r)| cmp_value(l, r))
            .find(|c| !matches!(c, Ordering::Equal))
            .unwrap_or_else(|| l.len().cmp(&r.len())),
        _ => panic!("unexpected json values: {v1:?}, {v2:?}"),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_task_1() {
        let input = include_str!("input/day13_example.txt");

        assert_eq!(task1(input).unwrap(), 13);
    }

    #[test]
    fn test_task_2() {
        let input = include_str!("input/day13_example.txt");

        assert_eq!(task2(input).unwrap(), 140);
    }
}
