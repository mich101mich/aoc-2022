use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/04.txt");

    let parsed = input
        .lines()
        .map(|l| sscanf!(l, "{usize}-{usize},{usize}-{usize}").unwrap())
        .filter(|(a1, b1, a2, b2)| b1 >= a2 && b2 >= a1)
        .count();

    pv!(parsed);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/04.txt");

    let parsed = input
        .lines()
        .map(|l| sscanf!(l, "{usize}-{usize},{usize}-{usize}").unwrap())
        .filter(|(a1, b1, a2, b2)| {
            let r1 = a1..=b1;
            let r2 = a2..=b2;
            (r1.contains(&a2) && r1.contains(&b2)) || (r2.contains(&a1) && r2.contains(&b1))
        })
        .count();

    pv!(parsed);
}
