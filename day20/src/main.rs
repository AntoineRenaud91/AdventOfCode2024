use std::path::PathBuf;

use ndarray::Array2;

#[cfg(test)]
const TEST_INPUT: &str = "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

fn parse(input: &str) -> (Array2<bool>, [usize; 2], [usize; 2]) {
    let nrows = input.trim().lines().count();
    let ncols = input.trim().lines().next().unwrap().trim().chars().count();
    let mut grid = Array2::from_elem((nrows, ncols), false);
    let mut source = None;
    let mut target = None;
    for (i, line) in input.trim().lines().enumerate() {
        for (j, c) in line.trim().chars().enumerate() {
            if c == '#' {
                continue;
            }
            grid[[i, j]] = true;
            if c == 'S' {
                source = Some([i, j]);
            }
            if c == 'E' {
                target = Some([i, j]);
            }
        }
    }
    (grid, source.unwrap(), target.unwrap())
}

fn process1(input: &str, threshold: usize) -> usize {
    let (track, source, target) = parse(input);
    let mut path = vec![source];
    let mut current = source;
    while current != target {
        let [i, j] = current;
        for next in [[i + 1, j], [i - 1, j], [i, j + 1], [i, j - 1]] {
            if track[next] && (path.len() == 1 || path[path.len() - 2] != next) {
                path.push(next);
                current = next;
                break;
            }
        }
    }
    (0..path.len() - 1)
        .flat_map(|i| (i + 2..path.len()).map(move |j| (i, j)))
        .filter(|(i, j)| {
            let [i1, j1] = path[*i];
            let [i2, j2] = path[*j];
            (i1.checked_sub(i2).unwrap_or_else(|| i2 - i1) == 2 && j1 == j2)
                || (j1.checked_sub(j2).unwrap_or_else(|| j2 - j1) == 2 && i1 == i2)
        })
        .filter(|(i, j)| (j - i - 1) >= threshold)
        .count()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT, 12), 8)
}

fn process2(input: &str, threshold: usize) -> usize {
    let (track, source, target) = parse(input);
    let mut path = vec![source];
    let mut current = source;
    while current != target {
        let [i, j] = current;
        for next in [[i + 1, j], [i - 1, j], [i, j + 1], [i, j - 1]] {
            if track[next] && (path.len() == 1 || path[path.len() - 2] != next) {
                path.push(next);
                current = next;
                break;
            }
        }
    }
    (0..path.len() - 1)
        .flat_map(|i| (i + 2..path.len()).map(move |j| (i, j)))
        .filter_map(|(i, j)| {
            let [i1, j1] = path[i];
            let [i2, j2] = path[j];
            let dist = i1.checked_sub(i2).unwrap_or_else(|| i2 - i1)
                + j1.checked_sub(j2).unwrap_or_else(|| j2 - j1);
            if dist <= 20 {
                Some((i, j, dist))
            } else {
                None
            }
        })
        .filter(|(i, j, dist)| (j - i - dist) >= threshold)
        .count()
}

#[test]
fn test_process2() {
    assert_eq!(process2(TEST_INPUT, 72), 29)
}

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(concat!("data/", env!("CARGO_PKG_NAME"), ".dat"));
    let input = std::fs::read_to_string(path).unwrap();
    let start = std::time::Instant::now();
    let result = process1(&input, 100);
    println!("Result part 1: {result} in {:?}", start.elapsed());
    let start = std::time::Instant::now();
    let result = process2(&input, 100);
    println!("Result part 2: {result} in {:?}", start.elapsed());
}
