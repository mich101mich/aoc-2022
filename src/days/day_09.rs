use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/09.txt");

    let parsed = input.lines().map(|l| sscanf!(l, "{Dir} {usize}").unwrap());

    let mut knots = vec![p2(0, 0isize); 10];

    let mut visited = HashSet::new();
    visited.insert(*knots.last().unwrap());

    for (dir, dist) in parsed {
        for _ in 0..dist {
            knots[0] += dir;
            for i in 0..9 {
                let delta = knots[i] - knots[i + 1];
                if delta.x.abs() > 1 || delta.y.abs() > 1 {
                    knots[i + 1].x += delta.x.signum();
                    knots[i + 1].y += delta.y.signum();
                }
            }
            visited.insert(*knots.last().unwrap());
        }
    }

    pv!(visited.len());
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/09.txt");

    let parsed = input.lines().map(|l| sscanf!(l, "{Dir} {usize}").unwrap());

    let mut head = p2(0, 0isize);
    let mut tail = p2(0, 0isize);

    let mut visited = HashSet::new();
    visited.insert(tail);

    for (dir, dist) in parsed {
        for _ in 0..dist {
            head += dir;
            if moore(head, tail) > 1 {
                tail = head + Dir::from_difference(head, tail);
                visited.insert(tail);
            }
        }
    }

    pv!(visited.len());
}
