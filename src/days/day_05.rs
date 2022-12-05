use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");

    let mut iter = input.lines();

    let mut stacks = vec![];
    for l in iter.by_ref().take_while(|l| !l.is_empty()) {
        for (i, c) in l.as_bytes().chunks(4).enumerate() {
            if c[0] == b'[' {
                if i >= stacks.len() {
                    stacks.resize(i + 1, vec![]);
                }
                stacks[i].push(c[1] as char);
            }
        }
    }
    stacks.iter_mut().for_each(|s| s.reverse());

    let parsed = iter
        .map(|l| sscanf!(l, "move {usize} from {usize} to {usize}").unwrap())
        .map(|(n, from, to)| (n, from - 1, to - 1));

    for (n, from, to) in parsed {
        let end = stacks[from].len() - n;
        let moved = stacks[from]
            .drain(end..)
            .collect::<Vec<_>>();
        stacks[to].extend(moved);
    }

    let mut s = String::new();
    for stack in stacks {
        s.push(*stack.last().unwrap());
    }
    pv!(s);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");

    let mut iter = input.lines();

    let mut stacks = vec![];
    for l in iter.by_ref().take_while(|l| !l.is_empty()) {
        for (i, c) in l.as_bytes().chunks(4).enumerate() {
            if c[0] == b'[' {
                if i >= stacks.len() {
                    stacks.resize(i + 1, vec![]);
                }
                stacks[i].push(c[1] as char);
            }
        }
    }
    stacks.iter_mut().for_each(|s| s.reverse());

    let parsed = iter
        .map(|l| sscanf!(l, "move {usize} from {usize} to {usize}").unwrap())
        .map(|(n, from, to)| (n, from - 1, to - 1));

    for (n, from, to) in parsed {
        for _ in 0..n {
            let c = stacks[from].pop().unwrap();
            stacks[to].push(c);
        }
    }

    let mut s = String::new();
    for stack in stacks {
        s.push(*stack.last().unwrap());
    }
    pv!(s);
}
