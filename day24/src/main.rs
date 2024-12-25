use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    path::PathBuf,
};

#[cfg(test)]
const TEST_INPUT: &str = "
    x00: 1
    x01: 0
    x02: 1
    x03: 1
    x04: 0
    y00: 1
    y01: 1
    y02: 1
    y03: 1
    y04: 1

    ntg XOR fgs -> mjb
    y02 OR x01 -> tnw
    kwq OR kpj -> z05
    x00 OR x03 -> fst
    tgd XOR rvg -> z01
    vdt OR tnw -> bfw
    bfw AND frj -> z10
    ffh OR nrd -> bqk
    y00 AND y03 -> djm
    y03 OR y00 -> psh
    bqk OR frj -> z08
    tnw OR fst -> frj
    gnj AND tgd -> z11
    bfw XOR mjb -> z00
    x03 OR x00 -> vdt
    gnj AND wpb -> z02
    x04 AND y00 -> kjc
    djm OR pbm -> qhw
    nrd AND vdt -> hwm
    kjc AND fst -> rvg
    y04 OR y02 -> fgs
    y01 AND x02 -> pbm
    ntg OR kjc -> kwq
    psh XOR fgs -> tgd
    qhw XOR tgd -> z09
    pbm OR djm -> kpj
    x03 XOR y03 -> ffh
    x00 XOR y04 -> ntg
    bfw OR bqk -> z06
    nrd XOR fgs -> wpb
    frj XOR qhw -> z04
    bqk OR frj -> z07
    y03 OR x01 -> nrd
    hwm AND bqk -> z03
    tgd XOR rvg -> z12
    tnw OR pbm -> gnj
";

#[derive(PartialEq, Clone, Copy)]
enum Operation {
    And,
    Or,
    Xor,
}

struct Problem<'a> {
    wires: BTreeMap<&'a str, Option<bool>>,
    gates: Vec<(Operation, &'a str, &'a str, &'a str)>,
    output_len: usize,
}

impl<'a> Problem<'a> {
    fn parse(input: &'a str) -> Self {
        let mut wires: BTreeMap<&str, Option<bool>> = BTreeMap::default();
        let mut gates = vec![];
        let (init, instr) = input.trim().split_once("\n\n").unwrap();
        for line in init.lines() {
            let (wire, value) = line.trim().split_once(": ").unwrap();
            wires.insert(wire, Some(value == "1"));
        }
        for line in instr.lines() {
            let (a, rest) = line.trim().split_once(" ").unwrap();
            let (op, rest) = rest.split_once(" ").unwrap();
            let (b, c) = rest.split_once(" -> ").unwrap();
            wires.entry(a).or_default();
            wires.entry(b).or_default();
            wires.entry(c).or_default();
            let op = match op {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "XOR" => Operation::Xor,
                _ => panic!("Unknown operation {op}"),
            };
            gates.push((op, a, b, c));
        }
        let output_len = wires
            .keys()
            .filter_map(|k| k.strip_prefix("z").map(|n| n.parse::<usize>().unwrap()))
            .max()
            .unwrap()
            + 1;
        Self {
            wires,
            gates,
            output_len,
        }
    }

    fn solve_p1(&mut self) -> usize {
        while !self.gates.is_empty() {
            let next_op_index = (0..self.gates.len())
                .find(|&i| {
                    let (_, a, b, _) = &self.gates[i];
                    self.wires[*a].is_some() && self.wires[*b].is_some()
                })
                .unwrap();
            let (op, a, b, c) = self.gates.swap_remove(next_op_index);
            *self.wires.get_mut(c).unwrap() = match op {
                Operation::And => Some(self.wires[a].unwrap() & self.wires[b].unwrap()),
                Operation::Or => Some(self.wires[a].unwrap() | self.wires[b].unwrap()),
                Operation::Xor => Some(self.wires[a].unwrap() ^ self.wires[b].unwrap()),
            };
        }
        self.wires
            .iter()
            .filter(|(k, _)| k.starts_with('z'))
            .enumerate()
            .map(|(i, (_, v))| if v.unwrap() { 1 << i } else { 0 })
            .sum()
    }

    fn solve_p2(&mut self) -> String {
        let mut wrong = BTreeSet::new();
        let highest_z_key = format!("z{:0>2}", self.output_len - 1);
        let first_chars = HashSet::from(['x', 'y', 'z']);
        for (op, a, b, c) in self.gates.iter().copied() {
            if c.starts_with('z') && op != Operation::Xor && c != highest_z_key {
                wrong.insert(c);
            }
            let a_first = a.chars().next().unwrap();
            let b_first = b.chars().next().unwrap();
            let c_first = c.chars().next().unwrap();
            if op == Operation::Xor
                && !first_chars.contains(&a_first)
                && !first_chars.contains(&b_first)
                && !first_chars.contains(&c_first)
            {
                wrong.insert(c);
            }
            if op == Operation::And && a != "x00" && b != "x00" {
                for (op, aa, bb, _) in self.gates.iter().copied() {
                    if (c == aa || c == bb) && op != Operation::Or {
                        wrong.insert(c);
                    }
                }
            }
            if op == Operation::Xor {
                for (op, aa, bb, _) in self.gates.iter().copied() {
                    if (c == aa || c == bb) && op == Operation::Or {
                        wrong.insert(c);
                    }
                }
            }
        }
        wrong.into_iter().collect::<Vec<&str>>().join(",")
    }
}

fn process1(input: &str) -> usize {
    Problem::parse(input).solve_p1()
}

#[test]
fn test_process1() {
    assert_eq!(process1(TEST_INPUT), 2024)
}

fn process2(input: &str) -> String {
    Problem::parse(input).solve_p2()
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
