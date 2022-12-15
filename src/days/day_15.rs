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

    let area = 0..=4000000;

    let (x, y) = parsed
        .iter()
        .flat_map(|(s, d)| manhattan_ring_iter(*s, d + 1))
        .filter(|p| area.contains(&p.0) && area.contains(&p.1))
        .find(|p| parsed.iter().all(|(s, d)| manhattan_i(*s, *p) > *d))
        .unwrap();

    pv!(x * 4000000 + y);
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

    let mut ranges = vec![];
    let mut beacons = HashSet::new();

    for (sx, sy, bx, by) in parsed {
        if by == 2000000 {
            beacons.insert(bx);
        }
        let dist = manhattan_i((sx, sy), (bx, by));
        let y_diff = (sy - 2000000).abs();
        if dist <= y_diff {
            continue;
        }
        let x_diff = dist - y_diff;
        ranges.push((sx - x_diff)..=(sx + x_diff));
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
