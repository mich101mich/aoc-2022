use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");

    let parsed = input
        .lines()
        .map(|l| {
            sscanf!(
                l,
                "Sensor at x={isize}, y={isize}: closest beacon is at x={isize}, y={isize}"
            )
            .unwrap()
        })
        .map(|(sx, sy, bx, by)| ((sx, sy), manhattan_i((sx, sy), (bx, by))))
        .to_vec();

    let mut x = 0;
    let mut y = 0;
    while x <= 4000000 {
        y = 0;
        while y <= 4000000 {
            let next_y = parsed
                .iter()
                .filter(|(s, d)| manhattan_i(*s, (x, y)) <= *d)
                .map(|(s, d)| {
                    let x_diff = (s.0 - x).abs();
                    s.1 + (d - x_diff)
                })
                .max();
            if let Some(ny) = next_y {
                if ny == y {
                    y += 1;
                } else {
                    y = ny;
                }
            } else {
                pv!(x * 4000000 + y);
                return;
            }
        }
        x += 1;
    }
    unreachable!();
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");

    let parsed = input.lines().map(|l| {
        sscanf!(
            l,
            "Sensor at x={isize}, y={isize}: closest beacon is at x={isize}, y={isize}"
        )
        .unwrap()
    });

    let mut row = HashSet::new();

    for (sx, sy, bx, by) in parsed {
        let dist = manhattan_i((sx, sy), (bx, by));
        let y_diff = (sy - 2000000).abs();
        if dist <= y_diff {
            continue;
        }
        let x_diff = (dist - y_diff).abs();
        for x in (sx - x_diff)..=(sx + x_diff) {
            if by == 2000000 && x == bx {
                continue;
            }
            row.insert(x);
        }
    }

    pv!(row.len());
}
