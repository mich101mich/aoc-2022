use crate::utils::*;

const MOORE: [PointI; 8] = [
    p2(0, -1),
    p2(1, -1),
    p2(1, 0),
    p2(1, 1),
    p2(0, 1),
    p2(-1, 1),
    p2(-1, 0),
    p2(-1, -1),
];

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");

    let mut positions = hashtag_grid(input)
        .pos_iter()
        .map(|p| p.cast::<isize>().unwrap())
        .to_vec();
    let mut elves = positions.iter().copied().to_set();

    let mut directions = [Up, Down, Left, Right].into_iter().to_queue();

    let mut suggestions = HashMap::with_capacity(elves.len());
    let mut targets = positions.clone();
    for round in 1.. {
        suggestions.clear();
        for (pos, target) in positions.iter().zip(&mut targets) {
            *target = *pos;
            if MOORE.iter().map(|d| pos + d).all(|p| !elves.contains(&p)) {
                continue;
            }
            for dir in &directions {
                let next = *pos + *dir;
                if !elves.contains(&next)
                    && !elves.contains(&(next + dir.clockwise()))
                    && !elves.contains(&(next + dir.counter_clockwise()))
                {
                    *suggestions.entry(next).or_insert(0) += 1;
                    *target = next;
                    break;
                }
            }
        }
        if suggestions.is_empty() {
            pv!(round);
            break;
        }
        elves.clear();
        for (next, pos) in targets.iter().zip(&mut positions) {
            if suggestions.get(next) == Some(&1) {
                elves.insert(*next);
                *pos = *next;
            } else {
                elves.insert(*pos);
            }
        }
        directions.rotate_left(1);
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");

    let mut positions = hashtag_grid(input)
        .pos_iter()
        .map(|p| p.cast::<isize>().unwrap())
        .to_vec();
    let mut elves = positions.iter().copied().to_set();

    let mut directions = [Up, Down, Left, Right].into_iter().to_queue();

    let mut suggestions = HashMap::with_capacity(elves.len());
    let mut targets = positions.clone();
    for _ in 0..10 {
        suggestions.clear();
        for (pos, target) in positions.iter().zip(&mut targets) {
            *target = *pos;
            if MOORE.iter().map(|d| pos + d).all(|p| !elves.contains(&p)) {
                continue;
            }
            for dir in &directions {
                let next = *pos + *dir;
                if !elves.contains(&next)
                    && !elves.contains(&(next + dir.clockwise()))
                    && !elves.contains(&(next + dir.counter_clockwise()))
                {
                    *suggestions.entry(next).or_insert(0) += 1;
                    *target = next;
                    break;
                }
            }
        }
        elves.clear();
        for (next, pos) in targets.iter().zip(&mut positions) {
            if suggestions.get(next) == Some(&1) {
                elves.insert(*next);
                *pos = *next;
            } else {
                elves.insert(*pos);
            }
        }
        directions.rotate_left(1);
    }

    let (min, max) = positions
        .iter()
        .fold((positions[0], positions[0]), |(min, max), p| {
            (
                p2(min.x.min(p.x), min.y.min(p.y)),
                p2(max.x.max(p.x), max.y.max(p.y)),
            )
        });
    let area = (max.x - min.x + 1) * (max.y - min.y + 1);
    let empty = area as usize - elves.len();
    pv!(empty);
}
