use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/09.txt");

    let parsed = input.lines().map(|l| sscanf!(l, "{Dir} {usize}").unwrap());

    let mut knots = vec![(0, 0isize); 10];

    let mut visited = HashSet::new();
    visited.insert(*knots.last().unwrap());

    for (dir, dist) in parsed {
        for _ in 0..dist {
            knots[0] += dir;
            for i in 0..9 {
                let axis_0 = knots[i].0 - knots[i + 1].0;
                let axis_1 = knots[i].1 - knots[i + 1].1;
                if axis_0.abs() > 1 || axis_1.abs() > 1 {
                    knots[i + 1].0 += axis_0.signum();
                    knots[i + 1].1 += axis_1.signum();
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

    let mut head = (0, 0isize);
    let mut tail = (0, 0isize);

    let mut visited = HashSet::new();
    visited.insert(tail);

    for (dir, dist) in parsed {
        for _ in 0..dist {
            head += dir;
            if moore_i(head, tail) > 1 {
                tail = head + Dir::from_difference(head, tail);
                visited.insert(tail);
            }
        }
    }

    pv!(visited.len());
}
