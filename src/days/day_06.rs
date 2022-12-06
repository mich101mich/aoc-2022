use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");

    let parsed = input
        .as_bytes()
        .windows(14)
        .position(|w| w.iter().copied().to_set().len() == 14)
        .unwrap()
        + 14;

    pv!(parsed);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");

    let parsed = input
        .as_bytes()
        .windows(4)
        .position(|w| {
            w[0] != w[1]
                && w[0] != w[2]
                && w[0] != w[3]
                && w[1] != w[2]
                && w[1] != w[3]
                && w[2] != w[3]
        })
        .unwrap()
        + 4;

    pv!(parsed);
}
