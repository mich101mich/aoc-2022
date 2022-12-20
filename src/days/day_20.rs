use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");

    let mut parsed = input
        .lines()
        .map(parse)
        .enumerate()
        .map(|(i, n)| (n * 811589153, i))
        .to_queue();

    let len = parsed.len() as isize - 1;
    for _ in 0..10 {
        for i in 0..parsed.len() {
            let remove_pos = parsed.iter().position(|(_, j)| *j == i).unwrap();
            let (x, _) = parsed.remove(remove_pos).unwrap();
            let mut new_pos = (((remove_pos as isize + x) % len + len) % len) as usize;
            parsed.insert(new_pos, (x, i));
        }
    }

    let zero_pos = parsed.iter().position(|(x, _)| *x == 0).unwrap();

    let res = parsed[(zero_pos + 1000) % parsed.len()].0
        + parsed[(zero_pos + 2000) % parsed.len()].0
        + parsed[(zero_pos + 3000) % parsed.len()].0;

    pv!(res);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");

    let mut parsed = input.lines().map(parse).map(|n| (n, false)).to_queue();

    let mut i = 0;
    while i < parsed.len() {
        if parsed[i].1 {
            i += 1;
            continue;
        }

        let (x, _) = parsed.remove(i).unwrap();
        let new_pos = (i as isize + x + 20 * parsed.len() as isize) as usize % parsed.len();
        parsed.insert(new_pos, (x, true));
    }

    let zero_pos = parsed.iter().position(|(x, _)| *x == 0).unwrap();

    let res = parsed[(zero_pos + 1000) % parsed.len()].0
        + parsed[(zero_pos + 2000) % parsed.len()].0
        + parsed[(zero_pos + 3000) % parsed.len()].0;

    pv!(res);
}
