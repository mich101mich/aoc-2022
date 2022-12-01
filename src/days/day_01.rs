use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/01.txt");

    let mut parsed = input
        .lines()
        .split_fold(|l| l.is_empty(), || 0, |acc, l| acc + parse_u(l))
        .to_vec();

    parsed.sort_unstable();
    let parsed: usize = parsed[parsed.len() - 3..].iter().sum();
    pv!(parsed);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/01.txt");

    let parsed = input
        .lines()
        .split_fold(|l| l.is_empty(), || 0, |acc, l| acc + parse_u(l))
        .max()
        .unwrap();
    pv!(parsed);
}
