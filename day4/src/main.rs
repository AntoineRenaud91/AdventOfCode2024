use std::path::PathBuf;

use ndarray::Array2;

#[cfg(test)]
const TEST_INPUT: &str = "
    MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX
";

fn parse(input: &str) -> Array2<char> {
    let n_lines = input.trim().lines().count();
    let n_cols = input.trim().lines().next().unwrap().trim().len();
    Array2::from_shape_vec(
        [n_lines, n_cols],
        input
            .trim()
            .lines()
            .flat_map(|line| line.trim().chars())
            .collect(),
    )
    .unwrap()
}

#[test]
fn test_parser() {
    assert_eq!(
        parse(TEST_INPUT),
        ndarray::array![
            ['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            ['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            ['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            ['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            ['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            ['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            ['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            ['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            ['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            ['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ]
    );
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    UpLeft,
    Left,
    DownLeft,
    Down,
    DownRight,
    Right,
    UpRight,
}

impl Dir {
    fn list() -> [Dir; 8] {
        [
            Dir::Up,
            Dir::UpLeft,
            Dir::Left,
            Dir::DownLeft,
            Dir::Down,
            Dir::DownRight,
            Dir::Right,
            Dir::UpRight,
        ]
    }
    fn list_x() -> [[Dir; 2]; 4] {
        [
            [Dir::UpLeft, Dir::UpRight],
            [Dir::DownLeft, Dir::DownRight],
            [Dir::UpRight, Dir::DownRight],
            [Dir::UpLeft, Dir::DownLeft],
        ]
    }
    fn next_ind(&self, index: [usize; 2]) -> Option<[usize; 2]> {
        Some(match self {
            Dir::Up => [index[0].checked_sub(1)?, index[1]],
            Dir::UpLeft => [index[0].checked_sub(1)?, index[1].checked_sub(1)?],
            Dir::Left => [index[0], index[1].checked_sub(1)?],
            Dir::DownLeft => [index[0] + 1, index[1].checked_sub(1)?],
            Dir::Down => [index[0] + 1, index[1]],
            Dir::DownRight => [index[0] + 1, index[1] + 1],
            Dir::Right => [index[0], index[1] + 1],
            Dir::UpRight => [index[0].checked_sub(1)?, index[1] + 1],
        })
    }
    fn rev(&self) -> Self {
        match self {
            Dir::Up => Dir::Down,
            Dir::UpLeft => Dir::DownRight,
            Dir::Left => Dir::Right,
            Dir::DownLeft => Dir::UpRight,
            Dir::Down => Dir::Up,
            Dir::DownRight => Dir::UpLeft,
            Dir::Right => Dir::Left,
            Dir::UpRight => Dir::DownLeft,
        }
    }
}

fn check_next(
    array: &Array2<char>,
    ind: [usize; 2],
    dir: Dir,
    expected_char: char,
) -> Option<[usize; 2]> {
    let new_ind = dir.next_ind(ind)?;
    let next_char = array.get(new_ind)?;
    if next_char == &expected_char {
        Some(new_ind)
    } else {
        None
    }
}

fn process1(input: &str) -> usize {
    let array = parse(input);
    let n_lines = array.shape()[0];
    let n_cols = array.shape()[1];
    (0..n_lines)
        .flat_map(|i| (0..n_cols).map(move |j| [i, j]))
        .filter(|ind| array[*ind] == 'X')
        .flat_map(|ind| Dir::list().into_iter().map(move |dir| (ind, dir)))
        .filter_map(|(ind, dir)| {
            check_next(&array, ind, dir, 'M')
                .and_then(|ind| check_next(&array, ind, dir, 'A'))
                .and_then(|ind| check_next(&array, ind, dir, 'S'))
        })
        .count()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT), 18)
}

fn process2(input: &str) -> usize {
    let array = parse(input);
    let n_lines = array.shape()[0];
    let n_cols = array.shape()[1];
    (0..n_lines)
        .flat_map(|i| (0..n_cols).map(move |j| [i, j]))
        .filter(|ind| array[*ind] == 'A')
        .flat_map(|ind| Dir::list_x().into_iter().map(move |dirs| (ind, dirs)))
        .filter_map(|(ind, dirs)| {
            check_next(&array, ind, dirs[0].rev(), 'M')
                .and_then(|_| check_next(&array, ind, dirs[1].rev(), 'M'))
                .and_then(|_| check_next(&array, ind, dirs[0], 'S'))
                .and_then(|_| check_next(&array, ind, dirs[1], 'S'))
        })
        .count()
}

#[test]
fn test_process2() {
    assert_eq!(process2(TEST_INPUT), 9)
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