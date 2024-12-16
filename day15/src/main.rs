use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use nalgebra::Vector2;

#[cfg(test)]
const TEST_INPUT: &str = "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

struct Model1 {
    walls: HashSet<Vector2<i64>>,
    boxes: HashSet<Vector2<i64>>,
    robot: Vector2<i64>,
}

enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Move {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '<' => Move::Left,
            '>' => Move::Right,
            '^' => Move::Up,
            'v' => Move::Down,
            _ => Err(())?,
        })
    }
}

impl Model1 {
    fn move_robot(&mut self, m: Move) {
        let dpos = match m {
            Move::Up => Vector2::new(0, -1),
            Move::Down => Vector2::new(0, 1),
            Move::Left => Vector2::new(-1, 0),
            Move::Right => Vector2::new(1, 0),
        };
        let mut next = self.robot + dpos;
        loop {
            if self.walls.contains(&next) {
                return;
            }
            if !self.boxes.contains(&next) {
                break;
            }
            next += dpos;
        }
        self.robot += dpos;
        if self.boxes.remove(&self.robot) {
            self.boxes.insert(next);
        }
    }

    fn _print(&self) {
        let min_x = self.walls.iter().map(|v| v.x).min().unwrap();
        let max_x = self.walls.iter().map(|v| v.x).max().unwrap();
        let min_y = self.walls.iter().map(|v| v.y).min().unwrap();
        let max_y = self.walls.iter().map(|v| v.y).max().unwrap();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let pos = Vector2::new(x, y);
                if self.walls.contains(&pos) {
                    print!("#");
                } else if self.boxes.contains(&pos) {
                    print!("O");
                } else if self.robot == pos {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn parse1(input: &str) -> (Model1, Vec<Move>) {
    let (model, moves) = input.trim().split_once("\n\n").unwrap();
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();
    let mut robot = None;
    for (i, line) in model.trim().lines().enumerate() {
        for (j, c) in line.trim().chars().enumerate() {
            let pos = Vector2::new(j as i64, i as i64);
            match c {
                '#' => {
                    walls.insert(pos);
                }
                'O' => {
                    boxes.insert(pos);
                }
                '@' => {
                    robot = Some(pos);
                }
                _ => {}
            }
        }
    }
    let moves = moves.chars().flat_map(|c| c.try_into()).collect();
    let model = Model1 {
        walls,
        boxes,
        robot: robot.unwrap(),
    };
    (model, moves)
}

fn process1(input: &str) -> i64 {
    let (mut model, moves) = parse1(input);
    for m in moves {
        model.move_robot(m);
    }
    model.boxes.iter().map(|v| v.x + v.y * 100).sum()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT), 10092)
}

struct Model2 {
    walls: HashSet<Vector2<i64>>,
    boxes: Vec<Vector2<i64>>,
    boxes_pos: HashMap<Vector2<i64>, usize>,
    robot: Vector2<i64>,
    inc: Vector2<i64>,
}

impl Model2 {
    fn move_robot(&mut self, m: Move) {
        let (dpos, ud) = match m {
            Move::Up => (Vector2::new(0, -1), true),
            Move::Down => (Vector2::new(0, 1), true),
            Move::Left => (Vector2::new(-1, 0), false),
            Move::Right => (Vector2::new(1, 0), false),
        };
        let mut boxes_to_move = Vec::new();
        if ud {
            let mut nexts = HashSet::new();
            nexts.insert(self.robot + dpos);
            loop {
                if nexts.iter().any(|next| self.walls.contains(next)) {
                    return;
                }
                let next_boxes = nexts
                    .iter()
                    .filter_map(|next| self.boxes_pos.get(next))
                    .copied()
                    .collect::<HashSet<_>>();
                if next_boxes.is_empty() {
                    break;
                }
                nexts = next_boxes
                    .iter()
                    .flat_map(|&i| [self.boxes[i] + dpos, self.boxes[i] + self.inc + dpos])
                    .collect();
                boxes_to_move.extend(next_boxes);
            }
        } else {
            let mut next = self.robot + dpos;
            loop {
                if self.walls.contains(&next) {
                    return;
                }
                if let Some(i) = self.boxes_pos.get(&next) {
                    boxes_to_move.push(*i);
                    next += 2 * dpos;
                } else {
                    break;
                }
            }
        }
        boxes_to_move.reverse();
        self.robot += dpos;
        for i in boxes_to_move {
            self.boxes_pos.remove(&self.boxes[i]);
            self.boxes_pos.remove(&(self.boxes[i] + self.inc));
            self.boxes[i] += dpos;
            self.boxes_pos.insert(self.boxes[i], i);
            self.boxes_pos.insert(self.boxes[i] + self.inc, i);
        }
    }

    fn _print(&self) {
        println!();
        let min_x = self.walls.iter().map(|v| v.x).min().unwrap();
        let max_x = self.walls.iter().map(|v| v.x).max().unwrap();
        let min_y = self.walls.iter().map(|v| v.y).min().unwrap();
        let max_y = self.walls.iter().map(|v| v.y).max().unwrap();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let pos = Vector2::new(x, y);
                if self.walls.contains(&pos) {
                    print!("#");
                } else if let Some(i) = self.boxes_pos.get(&pos).copied() {
                    if pos == self.boxes[i] {
                        print!("[");
                    } else {
                        print!("]");
                    }
                } else if self.robot == pos {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn parse2(input: &str) -> (Model2, Vec<Move>) {
    let (model, moves) = input.trim().split_once("\n\n").unwrap();
    let mut walls = HashSet::new();
    let mut boxes = Vec::new();
    let mut boxes_pos = HashMap::new();
    let mut robot = None;
    let inc = Vector2::new(1, 0);
    for (i, line) in model.trim().lines().enumerate() {
        for (j, c) in line.trim().chars().enumerate() {
            let pos = Vector2::new((2 * j) as i64, i as i64);
            match c {
                '#' => {
                    walls.insert(pos);
                    walls.insert(pos + inc);
                }
                'O' => {
                    let i = boxes.len();
                    boxes.push(pos);
                    boxes_pos.insert(pos, i);
                    boxes_pos.insert(pos + inc, i);
                }
                '@' => {
                    robot = Some(pos);
                }
                _ => {}
            }
        }
    }
    let moves = moves.chars().flat_map(|c| c.try_into()).collect();
    let model = Model2 {
        walls,
        boxes,
        boxes_pos,
        robot: robot.unwrap(),
        inc,
    };
    (model, moves)
}

fn process2(input: &str) -> i64 {
    let (mut model, moves) = parse2(input);
    for m in moves {
        model.move_robot(m);
    }
    model.boxes.iter().map(|v| v.x + v.y * 100).sum()
}

#[test]
fn test_process2() {
    assert_eq!(process2(TEST_INPUT), 9021)
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
