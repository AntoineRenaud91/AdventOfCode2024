use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use ndarray::Array2;

#[cfg(test)]
const TEST_INPUT: &str = "
    89010123
    78121874
    87430965
    96549874
    45678903
    32019012
    01329801
    10456732
";

fn parse(input: &str) -> Array2<u32> {
    let n_rows = input.trim().lines().count();
    let n_cols = input.trim().lines().next().unwrap().trim().chars().count();
    Array2::from_shape_vec(
        [n_rows, n_cols],
        input
            .trim()
            .lines()
            .flat_map(|line| line.trim().chars().map(|c| c.to_digit(10).unwrap()))
            .collect(),
    )
    .unwrap()
}

fn next_indices([i, j]: [usize; 2], [n_rows, n_cols]: [usize; 2]) -> Vec<[usize; 2]> {
    let mut indices = Vec::new();
    if i > 0 {
        indices.push([i - 1, j]);
    }
    if i < n_rows - 1 {
        indices.push([i + 1, j]);
    }
    if j > 0 {
        indices.push([i, j - 1]);
    }
    if j < n_cols - 1 {
        indices.push([i, j + 1]);
    }
    indices
}

fn process_single1(map: &Array2<u32>, source: [usize; 2]) -> usize {
    let mut paths = HashSet::new();
    let shape = [map.shape()[0], map.shape()[1]];
    paths.insert(source);
    for i in 1..10 {
        paths = paths
            .into_iter()
            .flat_map(|ind| {
                next_indices(ind, shape)
                    .into_iter()
                    .filter(|ind| map[*ind] == i)
            })
            .collect();
    }
    paths.len()
}

fn process1(input: &str) -> usize {
    let map = parse(input);
    let [n_rows, n_cols] = [map.shape()[0], map.shape()[1]];
    (0..n_rows)
        .flat_map(|i| (0..n_cols).map(move |j| [i, j]))
        .filter(|ind| map[*ind] == 0)
        .map(|source| process_single1(&map, source))
        .sum()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT), 36)
}

fn process_single2(map: &Array2<u32>, source: [usize; 2]) -> usize {
    let mut paths = HashMap::new();
    let shape = [map.shape()[0], map.shape()[1]];
    paths.insert(source, 1);
    for i in 1..10 {
        paths = paths
            .into_iter()
            .flat_map(|(ind, rating)| {
                next_indices(ind, shape)
                    .into_iter()
                    .filter(|ind| map[*ind] == i)
                    .map(move |ind| (ind, rating))
            })
            .fold(HashMap::new(), |mut acc, (ind, rating)| {
                *acc.entry(ind).or_default() += rating;
                acc
            });
    }
    paths.values().sum()
}

fn process2(input: &str) -> usize {
    let map = parse(input);
    let [n_rows, n_cols] = [map.shape()[0], map.shape()[1]];
    (0..n_rows)
        .flat_map(|i| (0..n_cols).map(move |j| [i, j]))
        .filter(|ind| map[*ind] == 0)
        .map(|source| process_single2(&map, source))
        .sum()
}

#[test]
fn test_process2() {
    assert_eq!(process2(TEST_INPUT), 81)
}

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(concat!("data/", env!("CARGO_PKG_NAME"), ".dat"));
    let input = std::fs::read_to_string(path).unwrap();
    let start = std::time::Instant::now();
    let result = process1(&input);
    println!("Result part 1: {result} in {:?}",start.elapsed());
    let start = std::time::Instant::now();
    let result = process2(&input);
    println!("Result part 2: {result} in {:?}",start.elapsed());
}