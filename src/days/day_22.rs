use crate::utils::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::digit1,
    combinator::map,
    multi::many1,
    IResult,
};

enum Instruction {
    TurnClockwise,
    TurnCounterClockwise,
    Forward(usize),
}

fn take_number(input: &str) -> IResult<&str, usize> {
    map(digit1, |s: &str| s.parse().unwrap())(input)
}

fn take_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(tag("L"), |_| Instruction::TurnCounterClockwise),
        map(tag("R"), |_| Instruction::TurnClockwise),
        map(take_number, Instruction::Forward),
    ))(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Walkable,
    Wall,
    Warp,
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");

    let w = 50;

    let mut split = input.split("\n\n");

    let faces = {
        let mut lines = split.next().unwrap().lines().to_vec();
        let faces = lines
            .chunks_exact(w)
            .map(|c| {
                (0..c[0].len() / w)
                    .map(|i| {
                        if c[0].as_bytes()[i * w] == b' ' {
                            return None;
                        }
                        let grid = c
                            .iter()
                            .map(|row| hashtag_line(&row[i * w..(i + 1) * w]))
                            .to_vec();
                        Some(Grid::from(grid))
                    })
                    .to_vec()
            })
            .to_vec();
        faces.into_iter().flatten().flatten().to_vec()
    };
    assert_eq!(faces.len(), 6);

    let d = w - 1;
    // my input: (yes, I know this is hardcoding, but 3d cubes are hard)
    //   0 1
    //   2
    // 3 4
    // 5

    let mut face_offsets = [
        p2(w, 0),
        p2(2 * w, 0),
        p2(w, w),
        p2(0, 2 * w),
        p2(w, 2 * w),
        p2(0, 3 * w),
    ];

    // [face] -> [side] -> (target_face, clockwise_rotations)
    let mut warp_map = [[(9, 9); 4]; 6];
    {
        // [(face, dir, face_out, clockwise_rotations)]
        #[rustfmt::skip]
        let connections = [
            (0, Up   , 5, 1),
            (0, Right, 1, 0),
            (0, Down , 2, 0),
            (0, Left , 3, 2),
            (1, Up   , 5, 0),
            (1, Right, 4, 2),
            (1, Down , 2, 1),
            (2, Down , 4, 0),
            (2, Left , 3, 3),
            (3, Right, 4, 0),
            (3, Down , 5, 0),
            (4, Down , 5, 1),
        ];

        for (a, dir, b, r) in connections {
            warp_map[a][dir as usize] = (b, r);
            let op_dir = (dir as usize + r + 2) % 4;
            let op_r = (4 - r) % 4;
            warp_map[b][op_dir] = (a, op_r);
        }
    }

    let instructions = {
        let line = split.next().unwrap();
        many1(take_instruction)(line).unwrap().1
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Position {
        face: usize,
        pos: Point,
        dir: Dir,
    }

    let warp_move = |cur: Position| {
        let next = faces[cur.face].checked_move(cur.pos, cur.dir);
        if let Some(next) = next {
            return Position { pos: next, ..cur };
        }
        let mut pos = faces[cur.face].wrapping_move(cur.pos, cur.dir);
        let (face, rotations) = warp_map[cur.face][cur.dir as usize];
        let mut dir = cur.dir;
        for _ in 0..rotations {
            pos = p2(d - pos.y, pos.x);
            dir = dir.clockwise();
        }
        Position { face, pos, dir }
    };

    // consistency check
    // for face in 0..6 {
    //     for dir in Dir::all() {
    //         let start = Position {
    //             face,
    //             pos: p2(1, 2),
    //             dir,
    //         };
    //         let mut pos = start;
    //         for _ in 0..4 * w {
    //             pos = warp_move(pos);
    //         }
    //         if pos != start {
    //             pv!(start);
    //             pos = start;
    //             for _ in 0..4 * w {
    //                 pos = warp_move(pos);
    //                 pv!(pos);
    //             }
    //             panic!();
    //         }
    //     }

    //     for (pos, dir) in [
    //         (p2(1, 0), Up),
    //         (p2(0, 3), Left),
    //         (p2(2, d), Down),
    //         (p2(d, 1), Right),
    //     ] {
    //         let start = Position { face, pos, dir };
    //         let next = warp_move(start);
    //         assert_ne!(next.face, face);
    //         let back = Position {
    //             dir: next.dir.opposite(),
    //             ..next
    //         };
    //         let mut original = warp_move(back);
    //         original.dir = original.dir.opposite();
    //         assert_eq!(
    //             original, start,
    //             "{:?} -> {:?} -> {:?} -> {:?}",
    //             start, next, back, original
    //         );
    //     }
    // }

    let start_x = faces[0][0].iter().position(|t| !*t).unwrap();
    let mut p = Position {
        face: 0,
        pos: p2(start_x, 0),
        dir: Dir::Right,
    };

    for instr in instructions {
        match instr {
            Instruction::TurnClockwise => p.dir = p.dir.clockwise(),
            Instruction::TurnCounterClockwise => p.dir = p.dir.counter_clockwise(),
            Instruction::Forward(n) => {
                for _ in 0..n {
                    let next = warp_move(p);
                    if faces[next.face][next.pos] {
                        break;
                    }
                    p = next;
                }
            }
        }
    }

    let pos = p.pos + face_offsets[p.face] + p2(1, 1);
    let mut score = pos.y * 1000 + pos.x * 4 + (p.dir as usize + 3) % 4;
    pv!(score);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");

    let mut split = input.split("\n\n");
    let mut grid = {
        let mut lines = split
            .next()
            .unwrap()
            .lines()
            .map(|l| {
                l.as_bytes()
                    .iter()
                    .map(|c| match *c {
                        b'#' => Tile::Wall,
                        b'.' => Tile::Walkable,
                        b' ' => Tile::Warp,
                        _ => panic!("Unknown tile: {}", *c),
                    })
                    .to_vec()
            })
            .to_vec();
        let len = lines.iter().map(|l| l.len()).max().unwrap();
        for l in &mut lines {
            l.resize(len, Tile::Warp);
            l.insert(0, Tile::Warp);
            l.push(Tile::Warp);
        }
        lines.insert(0, vec![Tile::Warp; len + 2]);
        lines.push(vec![Tile::Warp; len + 2]);
        Grid::from(lines)
    };

    let instructions = {
        let line = split.next().unwrap();
        many1(take_instruction)(line).unwrap().1
    };

    let start_x = grid[1].iter().position(|t| *t == Tile::Walkable).unwrap();
    let mut pos = p2(start_x, 1);
    let mut dir = Dir::Right;

    for instr in instructions {
        match instr {
            Instruction::TurnClockwise => dir = dir.clockwise(),
            Instruction::TurnCounterClockwise => dir = dir.counter_clockwise(),
            Instruction::Forward(n) => {
                for _ in 0..n {
                    let mut next = pos + dir;
                    if grid[next] == Tile::Warp {
                        let op = dir.opposite();
                        while grid[next + op] != Tile::Warp {
                            next += op;
                        }
                    }
                    if grid[next] == Tile::Wall {
                        break;
                    }
                    pos = next;
                }
            }
        }
    }

    let mut score = pos.y * 1000 + pos.x * 4 + (dir as usize + 3) % 4;
    pv!(score);
}
