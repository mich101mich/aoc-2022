use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");

    let parsed = input.lines().map(|l| l.bytes().to_set()).to_vec();
    let parsed = parsed
        .chunks_exact(3)
        .map(|c| {
            let shared = c[0].intersection(&c[1]).find(|x| c[2].contains(x)).unwrap();
            (if shared.is_ascii_lowercase() {
                shared - b'a' + 1
            } else {
                shared - b'A' + 27
            }) as usize
        })
        .sum::<usize>();

    pv!(parsed);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");

    let parsed = input
        .lines()
        .map(|l| {
            let (a, b) = l.split_at(l.len() / 2);
            let first = a.bytes().to_set();
            let shared = b.bytes().find(|c| first.contains(c)).unwrap();
            (if shared.is_ascii_lowercase() {
                shared - b'a' + 1
            } else {
                shared - b'A' + 27
            }) as usize
        })
        .sum::<usize>();

    pv!(parsed);
}
