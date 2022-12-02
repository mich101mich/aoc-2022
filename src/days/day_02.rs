use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");

    let parsed = input
        .lines()
        .map(|l| sscanf!(l, "{char} {char}").unwrap())
        .map(|(a, b)| (a as u8 - b'A', b))
        .map(|(a, b)| {
            let (b, points) = match b {
                'X' => ((a + 2) % 3, 0),
                'Y' => (a, 3),
                'Z' => ((a + 1) % 3, 6),
                _ => unreachable!(),
            };
            points + b as usize + 1
        })
        .sum::<usize>();

    pv!(parsed);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");

    let parsed = input
        .lines()
        .map(|l| sscanf!(l, "{char} {char}").unwrap())
        .map(|(a, b)| {
            (match (a, b) {
                ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
                ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
                ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
                _ => unreachable!(),
            }) + (b as u8 - b'X' + 1) as usize
        })
        .sum::<usize>();

    pv!(parsed);
}
