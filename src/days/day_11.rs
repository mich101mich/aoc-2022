use std::mem::replace;

use crate::utils::*;

#[derive(Debug, Clone, Copy, FromScanf)]
enum Value {
    #[sscanf("{}")]
    Number(usize),
    #[sscanf("old")]
    Old,
}

#[derive(Debug, Clone, Copy, FromScanf)]
enum Op {
    #[sscanf("+")]
    Add,
    #[sscanf("-")]
    Sub,
    #[sscanf("*")]
    Mul,
    #[sscanf("/")]
    Div,
}
impl Op {
    fn apply(&self, a: usize, b: &Value) -> usize {
        let b = match b {
            Value::Number(n) => *n,
            Value::Old => a,
        };
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
        }
    }
}

#[derive(Debug, FromScanf)]
#[sscanf(
    "Monkey {_n}:
  Starting items: {items}
  Operation: new = old {op} {operand}
  Test: divisible by {test}
    If true: throw to monkey {true_target}
    If false: throw to monkey {false_target}"
)]
struct Monkey {
    _n: usize,
    #[sscanf(map = |s: &str| s.split(", ").map(parse_u).collect())]
    items: Vec<usize>,
    op: Op,
    operand: Value,
    test: usize,
    true_target: usize,
    false_target: usize,
    #[sscanf(default = 0)]
    activity: usize,
}
impl Monkey {
    fn test_target(&self, item: usize) -> usize {
        if item % self.test == 0 {
            self.true_target
        } else {
            self.false_target
        }
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");

    let mut monkeys = input
        .split("\n\n")
        .map(|s| sscanf!(s, "{Monkey}").unwrap())
        .collect::<Vec<_>>();

    let modulus = monkeys
        .iter()
        .fold(1, |acc, m| num::integer::lcm(acc, m.test));

    let mut items = vec![];
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            std::mem::swap(&mut items, &mut monkeys[i].items);
            monkeys[i].activity += items.len();
            for item in &items {
                let monkey = &monkeys[i];
                let new_item = monkey.op.apply(*item, &monkey.operand) % modulus;
                let target = monkey.test_target(new_item);
                monkeys[target].items.push(new_item);
            }
            items.clear();
        }
    }

    monkeys.sort_unstable_by_key(|m| m.activity);

    let res = monkeys.pop().unwrap().activity * monkeys.pop().unwrap().activity;
    pv!(res);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");

    let mut monkeys = input
        .split("\n\n")
        .map(|s| sscanf!(s, "{Monkey}").unwrap())
        .collect::<Vec<_>>();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let items = replace(&mut monkeys[i].items, vec![]);
            for item in items {
                monkeys[i].activity += 1;
                let mut new_item = monkeys[i].op.apply(item, &monkeys[i].operand);
                new_item /= 3;
                let target = monkeys[i].test_target(new_item);
                monkeys[target].items.push(new_item);
            }
        }
    }

    monkeys.sort_unstable_by_key(|m| m.activity);

    let res = monkeys.pop().unwrap().activity * monkeys.pop().unwrap().activity;
    pv!(res);
}
