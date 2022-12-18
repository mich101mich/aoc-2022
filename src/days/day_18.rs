use crate::utils::*;

const DELTAS: [(isize, isize, isize); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];
fn offset(pos: (usize, usize, usize), i: usize) -> Option<(usize, usize, usize)> {
    pos.0.checked_add_signed(DELTAS[i].0).and_then(|x| {
        pos.1
            .checked_add_signed(DELTAS[i].1)
            .and_then(|y| pos.2.checked_add_signed(DELTAS[i].2).map(|z| (x, y, z)))
    })
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");
    // let input = "";

    let cubes = input
        .lines()
        .map(|l| sscanf!(l, "{usize},{usize},{usize}").unwrap())
        .to_set();

    let bounds = cubes.iter().fold((0, 0, 0), |(a, b, c), (x, y, z)| {
        (a.max(*x + 1), b.max(*y + 1), c.max(*z + 1))
    });

    let mut is_outside = vec![vec![vec![true; bounds.2]; bounds.1]; bounds.0];

    for pos in &cubes {
        is_outside[pos.0][pos.1][pos.2] = false;
    }

    for x in 0..bounds.0 {
        for y in 0..bounds.1 {
            for z in 0..bounds.2 {
                if !is_outside[x][y][z] {
                    continue;
                }
                let path = open_dijkstra(
                    |p, out| {
                        for i in 0..6 {
                            let next = offset(p, i)
                                .filter(|p| p.0 < bounds.0 && p.1 < bounds.1 && p.2 < bounds.2);
                            if let Some(next) = next {
                                if is_outside[next.0][next.1][next.2] {
                                    out.push(next);
                                }
                            } else {
                                return true;
                            }
                        }
                        false
                    },
                    (x, y, z),
                );
                if path.is_some() {
                    continue;
                }
                let mut queue = vec![(x, y, z)];
                while let Some(next) = queue.pop() {
                    is_outside[next.0][next.1][next.2] = false;
                    queue.extend(
                        (0..6)
                            .filter_map(|i| offset(next, i))
                            .filter(|p| is_outside[p.0][p.1][p.2]),
                    );
                }
            }
        }
    }

    let mut sum = 0;
    for pos in &cubes {
        let mut open = 6;
        for delta in 0..6 {
            if let Some(other) =
                offset(*pos, delta).filter(|p| p.0 < bounds.0 && p.1 < bounds.1 && p.2 < bounds.2)
            {
                if !is_outside[other.0][other.1][other.2] {
                    open -= 1;
                }
            }
        }
        sum += open;
    }

    pv!(sum);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");

    let cubes = input
        .lines()
        .map(|l| sscanf!(l, "{usize},{usize},{usize}").unwrap())
        .to_set();

    let mut sum = 0;
    for pos in &cubes {
        let mut open = 6;
        for delta in 0..6 {
            if let Some(other) = offset(*pos, delta) {
                if cubes.contains(&other) {
                    open -= 1;
                }
            }
        }
        sum += open;
    }

    pv!(sum);
}
