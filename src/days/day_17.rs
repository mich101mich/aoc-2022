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

type ShapeSize = (HashSet<(isize, isize)>, (isize, isize));

fn run_step<'a>(
    area: &mut Grid<bool>,
    next_shape: &mut impl Iterator<Item = &'a ShapeSize>,
    next_dir: &mut impl Iterator<Item = Dir>,
    highest: &mut isize,
) -> usize {
    let (shape, (_, h)) = next_shape.next().unwrap();

    let shape_points = |pos: (isize, isize)| shape.iter().map(move |p| (pos.0 + p.0, pos.1 + p.1));
    let mut pos = (2, *highest - 3 - h);
    let mut dirs_taken = 0;
    loop {
        let dir = next_dir.next().unwrap();
        dirs_taken += 1;

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
    *highest = pos.1.min(*highest);
    dirs_taken
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");

    let next_shape = SHAPES
        .iter()
        .map(|shape| {
            let w = shape[0].len() as isize;
            let h = shape.len() as isize;
            let shape = shape.join("\n");
            let set = dotted_grid(&shape, '#')
                .pos_iter()
                .map(|(x, y)| (x as isize, y as isize))
                .to_set();
            (set, (w, h))
        })
        .to_vec();
    let num_shapes = next_shape.len();
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
    let num_dirs = next_dir.len();
    let mut next_dir = next_dir.iter().copied().cycle();
    let mut dirs_taken = 0;

    let mut area = Grid::new_clone((7, 100000), false);
    let mut highest = area.h() as isize;

    let height = detect_increment_loop(
        1_000_000_000_000,
        || {
            shapes_taken += 1;
            dirs_taken += run_step(&mut area, &mut next_shape, &mut next_dir, &mut highest);

            let key = (shapes_taken % num_shapes, dirs_taken % num_dirs);
            (key, highest as _)
        },
        (num_dirs * num_shapes) as _,
    );

    let height = area.h() as i128 - height;
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
                .pos_iter()
                .map(|(x, y)| (x as isize, y as isize))
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
        run_step(&mut area, &mut next_shape, &mut next_dir, &mut highest);
    }
    let height = area.h() as isize - highest;
    pv!(height);
}
