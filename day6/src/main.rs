use std::{
    collections::HashSet,
    path::PathBuf,
};

use ndarray::Array2;

#[cfg(test)]
const TEST_INPUT: &str = "
    ....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#...
";

fn parse(input: &str) -> (Array2<bool>, [usize;2]) {
    let n_lines = input.trim().lines().count();
    let n_cols = input.trim().lines().next().unwrap().trim().len();
    let map = Array2::from_shape_vec(
        [n_lines, n_cols],input
        .trim()
        .lines()
        .flat_map(|line| line
            .trim()
            .chars()
            .map(|c| c == '#')
        ).collect()
    ).unwrap();
    let pos = input.trim().lines().enumerate().flat_map(|(i,line)| line
        .trim()
        .chars()
        .enumerate()
        .find_map(|(j,c)| if c == '^' { Some([i,j]) } else { None })
    ).next().unwrap();
    (map,pos)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn next_pos(&self, pos:[usize;2], shape: &[usize]) -> Option<[usize;2]> {
        match self {
            Dir::Up => if pos[0] == 0 { None } else { Some([pos[0]-1, pos[1]]) },
            Dir::Down => if pos[0] == shape[0]-1 { None } else { Some([pos[0]+1, pos[1]]) },
            Dir::Left => if pos[1] == 0 { None } else { Some([pos[0], pos[1]-1]) },
            Dir::Right => if pos[1] == shape[1]-1 { None } else { Some([pos[0], pos[1]+1]) },
        }
    }
    fn next_dir(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}


fn process1(input: &str) -> usize {
    let (map, mut pos) = parse(input);
    let shape = map.shape();
    let mut dir = Dir::Up;
    let mut visited = HashSet::<[usize;2]>::new();
    visited.insert(pos);
    while let Some(next_pos) = dir.next_pos(pos, shape) {
        if map[next_pos] {
            dir = dir.next_dir();
        } else {
            pos = next_pos;
            visited.insert(pos);
        }
    }
    visited.len()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT), 41)
}

fn process2(input: &str) -> usize {
    let (map, init_pos) = parse(input);
    let mut pos = init_pos;
    let shape = map.shape();
    let mut dir = Dir::Up;
    let mut visited = HashSet::<[usize;2]>::new();
    while let Some(next_pos) = dir.next_pos(pos, shape) {
        if map[next_pos] {
            dir = dir.next_dir();
        } else {
            pos = next_pos;
            visited.insert(pos);
        }
    }
    let mut valid_block_pos_count = 0 ;
    for block_pos in visited.into_iter() {
        let mut visited = HashSet::<([usize;2],Dir)>::default();
        let mut dir = Dir::Up;
        let mut pos = init_pos;
        while let Some(next_pos) = dir.next_pos(pos, shape) {
            if map[next_pos] || next_pos == block_pos{
                dir = dir.next_dir();
            } else {
                pos = next_pos;
                if !visited.insert((pos,dir)) {
                    valid_block_pos_count += 1;
                    break;
                };
            }
        }
    }
    valid_block_pos_count
}

#[test]
fn test_process2() {
    assert_eq!(process2(TEST_INPUT), 6)
}

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("data/day6.dat");
    let input = std::fs::read_to_string(path).unwrap();
    let result = process1(&input);
    println!("Day1 result: {result}");
    let result = process2(&input);
    println!("Day2 result: {result}");
}
