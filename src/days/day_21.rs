use std::fmt::format;

use crate::utils::*;

#[derive(Debug, Clone, FromScanf)]
enum Instr {
    #[sscanf("{}")]
    Number(isize),
    #[sscanf("{a} {op} {b}")]
    Calc {
        a: &'static str,
        b: &'static str,
        op: Op,
    },
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");

    let mut parsed = input
        .lines()
        .map(|l| sscanf!(l, "{str}: {Instr}").unwrap())
        .to_map();

    let (a, b) = match parsed.remove("root").unwrap() {
        Instr::Calc { a, b, .. } => (a, b),
        _ => panic!("root is not a ? b"),
    };
    parsed.remove("humn");

    fn get_value(src: &mut HashMap<&'static str, Instr>, target: &'static str) -> Option<isize> {
        if target == "humn" {
            return None;
        }
        let (a, b, op) = match src.get(target).unwrap() {
            Instr::Number(n) => return Some(*n),
            Instr::Calc { a, b, op } => (*a, *b, *op),
        };
        let a = get_value(src, a);
        let b = get_value(src, b);
        let v = op.apply(a?, b?);
        src.insert(target, Instr::Number(v));
        Some(v)
    }
    fn reverse_value(
        src: &mut HashMap<&'static str, Instr>,
        target: &'static str,
        expected: isize,
    ) -> bool {
        if target == "humn" {
            pv!(expected);
            return true;
        }

        let (a, b, op) = match src.get(target).unwrap() {
            Instr::Number(n) => {
                assert_eq!(*n, expected);
                return true;
            }
            Instr::Calc { a, b, op } => (*a, *b, *op),
        };
        let a_v = get_value(src, a);
        let b_v = get_value(src, b);
        match (a_v, b_v) {
            (Some(a), None) => reverse_value(src, b, op.inverse_left(a, expected)),
            (None, Some(b)) => reverse_value(src, a, op.inverse_right(b, expected)),
            (Some(a), Some(b)) => {
                assert_eq!(op.apply(a, b), expected);
                src.insert(target, Instr::Number(expected));
                true
            }
            (None, None) => false,
        }
    }

    let a_v = get_value(&mut parsed, a);
    let b_v = get_value(&mut parsed, b);
    match (a_v, b_v) {
        (Some(a), None) => while !reverse_value(&mut parsed, b, a) {},
        (None, Some(b)) => while !reverse_value(&mut parsed, a, b) {},
        _ => panic!("invalid condition"),
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");
    // let input = "";

    // let mut grid = _grid(input);

    let mut parsed = input
        .lines()
        .map(|l| sscanf!(l, "{str}: {Instr}").unwrap())
        .to_map();

    fn get_value(src: &mut HashMap<&'static str, Instr>, target: &'static str) -> isize {
        let (a, b, op) = match src.get(target).unwrap() {
            Instr::Number(n) => return *n,
            Instr::Calc { a, b, op } => (*a, *b, *op),
        };
        let a = get_value(src, a);
        let b = get_value(src, b);
        let v = op.apply(a, b);
        src.insert(target, Instr::Number(v));
        v
    }

    pv!(get_value(&mut parsed, "root"));
}
