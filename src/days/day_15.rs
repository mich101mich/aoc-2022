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
        .map(|(sx, sy, bx, by)| (p2(sx, sy), p2(bx, by)))
        .map(|(s, b)| (s, manhattan(s, b)))
        .to_vec();

    // // initial solution
    // let mut x = 0;
    // let mut y = 0;
    // while x <= 4000000 {
    //     y = 0;
    //     while y <= 4000000 {
    //         let next_y = parsed
    //             .iter()
    //             .filter(|(s, d)| manhattan(*s, p2(x, y)) <= *d)
    //             .map(|(s, d)| {
    //                 let x_diff = (s.x - x).abs();
    //                 s.y + (d - x_diff) + 1
    //             })
    //             .max();
    //         if let Some(ny) = next_y {
    //             y = ny;
    //         } else {
    //             pv!(x * 4000000 + y);
    //             return;
    //         }
    //     }
    //     x += 1;
    // }
    // unreachable!();

    let area = 0..=4000000;

    let p = parsed
        .iter()
        .flat_map(|(s, d)| manhattan_ring_iter(*s, d + 1))
        .filter(|p| area.contains(&p.x) && area.contains(&p.y))
        .find(|p| parsed.iter().all(|(s, d)| manhattan(*s, *p) > *d))
        .unwrap();

    pv!(p.x * 4000000 + p.y);
}

#[allow(unused)]
pub fn part_one() {
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
        .map(|(sx, sy, bx, by)| (p2(sx, sy), p2(bx, by)));

    let mut ranges = vec![];
    let mut beacons = HashSet::new();

    for (s, b) in parsed {
        if b.y == 2000000 {
            beacons.insert(b.x);
        }
        let dist = manhattan(s, b);
        let y_diff = (s.y - 2000000).abs();
        if dist <= y_diff {
            continue;
        }
        let x_diff = dist - y_diff;
        ranges.push((s.x - x_diff)..=(s.x + x_diff));
    }

    ranges.sort_by_key(|r| *r.start());
    let mut current = ranges[0].clone();
    let mut sum = 0;
    for r in &ranges[1..] {
        if r.start() <= current.end() {
            current = *current.start()..=*r.end().max(current.end());
        } else {
            sum += current.end() - current.start() + 1;
            current = r.clone();
        }
    }
    sum += current.end() - current.start() + 1;
    sum -= beacons.len() as isize;

    pv!(sum);
}
