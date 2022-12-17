use core::panic;

use crate::utils::*;

const SHAPES: [&[&str]; 5] = [
    &[
        "####", //
    ],
    &[
        ".#.", //
        "###", //
        ".#.", //
    ],
    &[
        "..#", //
        "..#", //
        "###", //
    ],
    &[
        "#", //
        "#", //
        "#", //
        "#", //
    ],
    &[
        "##", //
        "##", //
    ],
];

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");
    // let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    let next_shape = SHAPES
        .iter()
        .map(|shape| {
            let w = shape[0].len() as isize;
            let h = shape.len() as isize;
            let shape = shape.join("\n");
            let set = dotted_grid(&shape, '#')
                .grid_iter_index()
                .filter_map(|((x, y), t)| t.then_some((x as isize, y as isize)))
                .to_set();
            (set, (w, h))
        })
        .to_vec();
    let num_shapes = next_shape.len() as u64;
    let mut next_shape = next_shape.iter().cycle();
    let mut shapes_taken = 0;

    let next_dir = input
        .bytes()
        .map(|c| match c {
            b'>' => Dir::Right,
            b'<' => Dir::Left,
            c => panic!("Invalid direction: {}", c),
        })
        .to_vec();
    let num_dirs = next_dir.len() as u64;
    let mut next_dir = next_dir.iter().copied().cycle();
    let mut dirs_taken = 0;

    let mut area = Grid::new_clone((7, 1000000), false);
    let mut highest = area.h() as isize;

    let mut seen_cycles = HashMap::new();
    let mut height_offset = 0;

    let target = 1_000_000_000_000u64;
    while shapes_taken < target {
        let (shape, (w, h)) = next_shape.next().unwrap();
        shapes_taken += 1;

        let shape_points =
            |pos: (isize, isize)| shape.iter().map(move |p| (pos.0 + p.0, pos.1 + p.1));
        let mut pos = (2, highest - 3 - h);
        loop {
            let dir = next_dir.next().unwrap();
            dirs_taken = (dirs_taken + 1) % num_dirs;

            if shape_points(pos)
                .map(|p| p + dir)
                .all(|p| area.in_bounds(p) && !area[p])
            {
                pos += dir;
            }
            let dir = Dir::Down;
            if shape_points(pos)
                .map(|p| p + dir)
                .all(|p| area.in_bounds(p) && !area[p])
            {
                pos += dir;
            } else {
                break;
            }
        }
        for p in shape_points(pos) {
            area[p] = true;
        }
        highest = highest.min(pos.1);

        if shapes_taken < num_dirs * num_shapes || height_offset > 0 {
            continue;
        }

        let key = (shapes_taken % num_shapes, dirs_taken);
        match seen_cycles.entry(key) {
            Entry::Occupied(e) => {
                let (cycle_start, start_height) = e.get();
                let cycle_length = shapes_taken - cycle_start;
                let cycle_gain = (area.h() as isize - highest) - start_height;
                let cycles_left = (target - shapes_taken) / cycle_length;
                shapes_taken += cycles_left * cycle_length;
                height_offset = cycles_left as isize * cycle_gain;
            }
            Entry::Vacant(e) => {
                e.insert((shapes_taken, area.h() as isize - highest));
            }
        }
    }
    let height = area.h() as isize - highest + height_offset;
    pv!(height);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");

    let next_shape = SHAPES
        .iter()
        .map(|shape| {
            let w = shape[0].len() as isize;
            let h = shape.len() as isize;
            let shape = shape.join("\n");
            let set = dotted_grid(&shape, '#')
                .grid_iter_index()
                .filter_map(|((x, y), t)| t.then_some((x as isize, y as isize)))
                .to_set();
            (set, (w, h))
        })
        .to_vec();
    let mut next_shape = next_shape.iter().cycle();

    let next_dir = input
        .bytes()
        .map(|c| match c {
            b'>' => Dir::Right,
            b'<' => Dir::Left,
            c => panic!("Invalid direction: {}", c),
        })
        .to_vec();
    let mut next_dir = next_dir.iter().copied().cycle();

    let mut area = Grid::new_clone((7, 10000), false);
    let mut highest = area.h() as isize;

    for _ in 0..2022 {
        let (shape, (w, h)) = next_shape.next().unwrap();
        let shape_points =
            |pos: (isize, isize)| shape.iter().map(move |p| (pos.0 + p.0, pos.1 + p.1));
        let mut pos = (2, highest - 3 - h);
        loop {
            let dir = next_dir.next().unwrap();

            if shape_points(pos)
                .map(|p| p + dir)
                .all(|p| area.in_bounds(p) && !area[p])
            {
                pos += dir;
            }
            let dir = Dir::Down;
            if shape_points(pos)
                .map(|p| p + dir)
                .all(|p| area.in_bounds(p) && !area[p])
            {
                pos += dir;
            } else {
                break;
            }
        }
        for p in shape_points(pos) {
            area[p] = true;
        }
        highest = highest.min(pos.1);
    }
    let height = area.h() as isize - highest;
    pv!(height);
}
