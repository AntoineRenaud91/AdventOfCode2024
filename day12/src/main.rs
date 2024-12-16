use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use ndarray::Array2;

#[cfg(test)]
const TEST_INPUT_SMALL: &str = "
    RRR
    RRR
    VVV
";

#[cfg(test)]
const TEST_INPUT: &str = "
    RRRRIICCFF
    RRRRIICCCF
    VVRRRCCFFF
    VVRCCCJFFF
    VVVVCJJCFE
    VVIVCCJJEE
    VVIIICJJEE
    MIIIIIJJEE
    MIIISIJEEE
    MMMISSJEEE
";

fn parse(input: &str) -> Array2<char> {
    let n_rows = input.trim().lines().count();
    let n_cols = input.trim().lines().next().unwrap().trim().chars().count();
    Array2::from_shape_vec(
        [n_rows, n_cols],
        input
            .trim()
            .lines()
            .flat_map(|line| line.trim().chars())
            .collect(),
    )
    .unwrap()
}

#[derive(Clone, Debug, PartialEq)]
pub struct Region {
    plant_kind: char,
    cells: HashSet<[usize; 2]>,
    edges: HashSet<[usize; 2]>,
}

impl Region {
    fn new(plant_kind: char, cell: [usize; 2]) -> Self {
        let mut cells = HashSet::new();
        cells.insert(cell);
        let mut edges = HashSet::new();
        edges.extend([
            [2 * cell[0], 2 * cell[1] + 1],
            [2 * cell[0] + 1, 2 * cell[1]],
            [2 * cell[0] + 2, 2 * cell[1] + 1],
            [2 * cell[0] + 1, 2 * cell[1] + 2],
        ]);
        Self {
            plant_kind,
            cells,
            edges,
        }
    }
    fn try_merge(&mut self, other: &Self) -> bool {
        if self.plant_kind != other.plant_kind {
            return false;
        }
        if self.edges.intersection(&other.edges).count() == 0 {
            return false;
        }
        self.edges = self
            .edges
            .symmetric_difference(&other.edges)
            .copied()
            .collect();
        self.cells = self.cells.union(&other.cells).copied().collect();
        true
    }

    fn area(&self) -> usize {
        self.cells.len()
    }

    fn perimeter(&self) -> usize {
        self.edges.len()
    }
}

#[test]
fn test_region() {
    let mut r1 = Region::new('A', [0, 0]);
    let mut r2 = Region::new('A', [0, 1]);
    let r3 = Region::new('A', [0, 0]);
    assert!(r1.try_merge(&r2));
    assert!(r2.try_merge(&r3));
    assert_eq!(r1, r2);
    let mut r3 = Region::new('A', [1, 0]);
    let r4 = Region::new('A', [2, 0]);
    assert!(r3.try_merge(&r4));
    assert!(r3.try_merge(&r1));
    assert_eq!(r3.area(), 4);
    assert_eq!(r3.perimeter(), 10);
}

fn process1(input: &str) -> usize {
    let map = parse(input);
    let mut regions: HashMap<char, Vec<Region>> = HashMap::new();
    let [n_rows, n_cols] = [map.shape()[0], map.shape()[1]];
    for i in 0..n_rows {
        for j in 0..n_cols {
            let region = Region::new(map[[i, j]], [i, j]);
            if let Some(regions) = regions.get_mut(&region.plant_kind) {
                if !regions.iter_mut().any(|r| r.try_merge(&region)) {
                    regions.push(region);
                }
            } else {
                regions.insert(region.plant_kind, vec![region]);
            }
        }
    }
    for regions in regions.values_mut() {
        loop {
            let mut merged_region = vec![regions[0].clone()];
            for region in regions.iter().skip(1) {
                if !merged_region.iter_mut().any(|r| r.try_merge(region)) {
                    merged_region.push(region.clone());
                }
            }
            let do_break = merged_region.len() == regions.len();
            *regions = merged_region;
            if do_break {
                break;
            }
        }
    }
    regions
        .into_values()
        .flat_map(|regions| regions.into_iter())
        .map(|r| r.area() * r.perimeter())
        .sum()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT_SMALL), 6 * 10 + 3 * 8);
    assert_eq!(process1(TEST_INPUT), 1930);
}

#[derive(Debug, Clone)]
struct Side {
    updown: bool,
    ref_i: usize,
    nodes: [usize; 2],
}

impl Side {
    fn new(edges: [usize; 2]) -> Self {
        if edges[0] % 2 == 0 {
            let updown = true;
            let ref_i = edges[0] / 2;
            let nodes = [(edges[1] - 1) / 2, (edges[1] - 1) / 2 + 1];
            Self {
                updown,
                ref_i,
                nodes,
            }
        } else {
            let updown = false;
            let ref_i = edges[1] / 2;
            let nodes = [(edges[0] - 1) / 2, (edges[0] - 1) / 2 + 1];
            Self {
                updown,
                ref_i,
                nodes,
            }
        }
    }

    fn nodes(&self) -> [[usize; 2]; 2] {
        if self.updown {
            [[self.ref_i, self.nodes[0]], [self.ref_i, self.nodes[1]]]
        } else {
            [[self.nodes[0], self.ref_i], [self.nodes[1], self.ref_i]]
        }
    }

    fn try_merge(&mut self, other: &Self) -> bool {
        if self.updown != other.updown {
            return false;
        }
        if self.ref_i != other.ref_i {
            return false;
        }
        if self.nodes[0] == other.nodes[1] {
            self.nodes = [other.nodes[0], self.nodes[1]];
            return true;
        }
        if self.nodes[1] == other.nodes[0] {
            self.nodes = [self.nodes[0], other.nodes[1]];
            return true;
        }
        false
    }
}

impl Region {
    fn sides(&self) -> usize {
        let mut nodes_to_sides: HashMap<[usize; 2], usize> = HashMap::new();
        let mut sides = vec![];
        let mut add_count: i64 = 0;
        let mut closed_nodes: HashSet<[usize; 2]> = HashSet::new();
        for edge in self.edges.iter().copied() {
            let side = Side::new(edge);
            let [node1, node2] = side.nodes();
            match (nodes_to_sides.remove(&node1), nodes_to_sides.remove(&node2)) {
                (None, None) => {
                    let i = sides.len();
                    sides.push(side);
                    nodes_to_sides.insert(node1, i);
                    nodes_to_sides.insert(node2, i);
                }
                (Some(i), None) => {
                    if !sides[i].try_merge(&side) {
                        let i = sides.len();
                        sides.push(side);
                        nodes_to_sides.insert(node2, i);
                    } else {
                        nodes_to_sides.insert(node2, i);
                        if !closed_nodes.insert(node1) {
                            add_count += 2;
                        }
                    }
                }
                (None, Some(i)) => {
                    if !sides[i].try_merge(&side) {
                        let i = sides.len();
                        sides.push(side);
                        nodes_to_sides.insert(node1, i);
                    } else {
                        nodes_to_sides.insert(node1, i);
                        if !closed_nodes.insert(node2) {
                            add_count += 2;
                        }
                    }
                }
                (Some(i), Some(j)) => {
                    match (sides[i].try_merge(&side), sides[j].try_merge(&side)) {
                        (true, true) => {
                            if !closed_nodes.insert(node1) {
                                add_count += 2;
                            }
                            if !closed_nodes.insert(node2) {
                                add_count += 2;
                            }
                            add_count -= 1;
                        }
                        (false, false) => {
                            add_count += 1;
                        }
                        (true, false) => {
                            if !closed_nodes.insert(node1) {
                                add_count += 2;
                            }
                        }
                        (false, true) => {
                            if !closed_nodes.insert(node2) {
                                add_count += 2;
                            }
                        }
                    }
                }
            }
        }
        (sides.len() as i64 + add_count) as usize
    }
}

#[test]
fn test_region_sides() {
    let r1 = Region::new('A', [0, 0]);
    assert_eq!(r1.sides(), 4);
}

fn process2(input: &str) -> usize {
    let map = parse(input);
    let mut regions: HashMap<char, Vec<Region>> = HashMap::new();
    let [n_rows, n_cols] = [map.shape()[0], map.shape()[1]];
    for i in 0..n_rows {
        for j in 0..n_cols {
            let region = Region::new(map[[i, j]], [i, j]);
            if let Some(regions) = regions.get_mut(&region.plant_kind) {
                if !regions.iter_mut().any(|r| r.try_merge(&region)) {
                    regions.push(region);
                }
            } else {
                regions.insert(region.plant_kind, vec![region]);
            }
        }
    }
    for regions in regions.values_mut() {
        loop {
            let mut merged_region = vec![regions[0].clone()];
            for region in regions.iter().skip(1) {
                if !merged_region.iter_mut().any(|r| r.try_merge(region)) {
                    merged_region.push(region.clone());
                }
            }
            let do_break = merged_region.len() == regions.len();
            *regions = merged_region;
            if do_break {
                break;
            }
        }
    }
    regions
        .values()
        .flat_map(|regions| regions.iter())
        .map(|r| r.area() * r.sides())
        .sum()
}

#[test]
fn test_process2() {
    assert_eq!(
        process2(
            "
    AAAA
    BBCD
    BBCC
    EEEC"
        ),
        80
    );
    assert_eq!(
        process2(
            "
    OOOOO
    OXOXO
    OOOOO
    OXOXO
    OOOOO"
        ),
        436
    );
    assert_eq!(process2(TEST_INPUT), 1206);
    assert_eq!(
        process2(
            "
    EEEEE
    EXXXX
    EEEEE
    EXXXX
    EEEEE
    "
        ),
        236
    );
    assert_eq!(
        process2(
            "
    AAAAAA
    AAABBA
    AAABBA
    ABBAAA
    ABBAAA
    AAAAAA"
        ),
        368
    );
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
