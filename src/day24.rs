use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

pub fn a(input: impl BufRead) -> u64 {
    let mut circuit = parse_input(input);
    circuit.eval()
}

#[derive(Debug, Clone, Default)]
struct Circuit(HashMap<Wire, Node>);

impl Circuit {
    fn outputs(&self) -> Vec<Wire> {
        let mut outputs = self
            .0
            .keys()
            .filter(|wire| wire.starts_with('z'))
            .cloned()
            .collect::<Vec<_>>();
        outputs.sort_unstable();
        outputs
    }

    fn eval(&mut self) -> u64 {
        self.outputs()
            .iter()
            .map(|wire| self.get_or_eval_node(wire))
            .enumerate()
            .fold(0, |acc, (i, value)| acc | ((value as u64) << i))
    }

    fn get_or_eval_node(&mut self, wire: &Wire) -> bool {
        let Some(node) = self.0.get(wire) else {
            panic!("Output wire not found in circuit")
        };
        let new_value = match node.clone() {
            Node::Value(value) => value,
            Node::Gate(lhs, op, rhs) => {
                op.apply(self.get_or_eval_node(&lhs), self.get_or_eval_node(&rhs))
            }
        };
        self.0.insert(wire.clone(), Node::Value(new_value));
        new_value
    }
}

#[derive(Copy, Clone, Debug)]
enum Op {
    And,
    Or,
    Xor,
}

impl FromStr for Op {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Op::And),
            "OR" => Ok(Op::Or),
            "XOR" => Ok(Op::Xor),
            _ => Err("Unknown op"),
        }
    }
}

impl Op {
    fn apply(&self, lhs: bool, rhs: bool) -> bool {
        match self {
            Op::And => lhs && rhs,
            Op::Or => lhs || rhs,
            Op::Xor => lhs != rhs,
        }
    }
}

type Wire = String;

#[derive(Clone, Debug)]
enum Node {
    Value(bool),
    Gate(Wire, Op, Wire),
}

fn parse_input(input: impl BufRead) -> Circuit {
    let mut circuit: Circuit = Default::default();

    let mut input = input.lines();
    for line in &mut input {
        let line = line.unwrap();
        if line.trim().is_empty() {
            break;
        }
        let mut line = line.split(": ");
        let wire = line.next().unwrap();
        let value = match line.next().unwrap() {
            "0" => false,
            "1" => true,
            _ => panic!("Must be boolean value"),
        };
        circuit.0.insert(wire.to_string(), Node::Value(value));
    }
    for line in &mut input {
        let line = line.unwrap();
        let mut line = line.split(" ");
        let lhs = line.next().unwrap().to_string();
        let op = line.next().unwrap().parse::<Op>().unwrap();
        let rhs = line.next().unwrap().to_string();
        assert!(line.next().unwrap() == "->");
        let wire = line.next().unwrap().to_string();
        circuit.0.insert(wire, Node::Gate(lhs, op, rhs));
    }
    circuit
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_str;

    #[test]
    fn test_a() {
        let example = "x00: 1
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
tnw OR pbm -> gnj";

        assert_eq!(a(read_str(example)), 2024);
    }
}
