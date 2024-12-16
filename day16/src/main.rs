use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    hash::RandomState,
    path::PathBuf,
};

use ndarray::Array2;

#[cfg(test)]
const TEST_INPUT: &str = "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

fn parse(input: &str) -> Array2<bool> {
    let nrows = input.trim().lines().count();
    let ncols = input.trim().lines().next().unwrap().trim().chars().count();
    Array2::from_shape_vec(
        [nrows, ncols],
        input
            .trim()
            .lines()
            .flat_map(|l| l.trim().chars().map(|c| c != '#'))
            .collect(),
    )
    .unwrap()
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn next_ind(&self, ind: [usize; 2]) -> [usize; 2] {
        match self {
            Dir::North => [ind[0] - 1, ind[1]],
            Dir::East => [ind[0], ind[1] + 1],
            Dir::South => [ind[0] + 1, ind[1]],
            Dir::West => [ind[0], ind[1] - 1],
        }
    }
    fn rotate_cw(&self) -> Self {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }

    fn rotate_ccw(&self) -> Self {
        match self {
            Dir::North => Dir::West,
            Dir::West => Dir::South,
            Dir::South => Dir::East,
            Dir::East => Dir::North,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
struct Node {
    ind: [usize; 2],
    dir: Dir,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    cost: usize,
    node: Node,
    prev: Option<Node>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

fn process1(input: &str) -> usize {
    let maze = parse(input);
    let [nrows, ncols] = [maze.shape()[0], maze.shape()[1]];
    let source = Node {
        ind: [nrows - 2, 1],
        dir: Dir::East,
    };
    let target = [1, ncols - 2];
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(State {
        cost: 0,
        node: source,
        prev: None,
    }));
    let mut visited = HashSet::<Node>::new();
    while let Some(Reverse(State { cost, node, .. })) = queue.pop() {
        if node.ind == target {
            return cost;
        }
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);
        queue.push(Reverse(State {
            cost: cost + 1000,
            node: Node {
                ind: node.ind,
                dir: node.dir.rotate_cw(),
            },
            prev: None,
        }));
        queue.push(Reverse(State {
            cost: cost + 1000,
            node: Node {
                ind: node.ind,
                dir: node.dir.rotate_ccw(),
            },
            prev: None,
        }));
        let next = node.dir.next_ind(node.ind);
        if maze[next] {
            queue.push(Reverse(State {
                cost: cost + 1,
                node: Node {
                    ind: next,
                    dir: node.dir,
                },
                prev: None,
            }));
        }
    }
    panic!("No path found")
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT), 7036)
}

fn process2(input: &str) -> usize {
    let maze = parse(input);
    let [nrows, ncols] = [maze.shape()[0], maze.shape()[1]];
    let source = Node {
        ind: [nrows - 2, 1],
        dir: Dir::East,
    };
    let target = [1, ncols - 2];
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(State {
        cost: 0,
        node: source,
        prev: None,
    }));
    let mut visited = HashMap::<Node, (usize, HashSet<Node>)>::new();
    while let Some(Reverse(State {
        cost,
        mut node,
        prev,
    })) = queue.pop()
    {
        if node.ind == target {
            node.dir = Dir::North;
        }
        if let Some((prev_cost, prevs)) = visited.get_mut(&node) {
            if cost == *prev_cost {
                prevs.insert(prev.unwrap());
            } else {
                continue;
            }
        } else {
            visited.insert(
                node,
                (
                    cost,
                    prev.map(|prev| {
                        let mut prevs = HashSet::new();
                        prevs.insert(prev);
                        prevs
                    })
                    .unwrap_or_default(),
                ),
            );
        }
        if node.ind == target {
            continue;
        }
        queue.push(Reverse(State {
            cost: cost + 1000,
            node: Node {
                ind: node.ind,
                dir: node.dir.rotate_cw(),
            },
            prev: Some(node),
        }));
        queue.push(Reverse(State {
            cost: cost + 1000,
            node: Node {
                ind: node.ind,
                dir: node.dir.rotate_ccw(),
            },
            prev: Some(node),
        }));
        let next = node.dir.next_ind(node.ind);
        if maze[next] {
            queue.push(Reverse(State {
                cost: cost + 1,
                node: Node {
                    ind: next,
                    dir: node.dir,
                },
                prev: Some(node),
            }));
        }
    }
    let mut best_tiles = HashSet::<[usize; 2]>::new();
    let mut nodes = HashSet::<_, RandomState>::from_iter([Node {
        ind: target,
        dir: Dir::North,
    }]);
    while !nodes.is_empty() {
        best_tiles.extend(nodes.iter().map(|n| n.ind));
        nodes = nodes
            .iter()
            .flat_map(|n| &visited.get(n).unwrap().1)
            .copied()
            .collect();
    }
    best_tiles.len()
}

#[test]
fn test_process2() {
    assert_eq!(process2(TEST_INPUT), 45)
}

fn main() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(concat!("data/", env!("CARGO_PKG_NAME"), ".dat"));
    let input = std::fs::read_to_string(path).unwrap();
    let start = std::time::Instant::now();
    let result = process1(&input);
    println!("Result part 1: {result} in {:?}", start.elapsed());
    let start = std::time::Instant::now();
    let result = process2(&input);
    println!("Result part 2: {result} in {:?}", start.elapsed());
}
