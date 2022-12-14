use crate::utils::*;

fn empty(c: &u8) -> bool {
    *c == b'.'
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");

    let mut grid = Grid::new_clone((1000, 1000), b'.');

    for l in input.lines() {
        let mut parts = l
            .split(" -> ")
            .map(|s| sscanf!(s, "{usize},{usize}").unwrap());
        let mut pos = parts.next().unwrap();
        for next in parts {
            if pos.0 == next.0 {
                for y in pos.1.min(next.1)..=pos.1.max(next.1) {
                    grid[(pos.0, y)] = b'#';
                }
            } else {
                for x in pos.0.min(next.0)..=pos.0.max(next.0) {
                    grid[(x, pos.1)] = b'#';
                }
            }
            pos = next;
        }
    }

    let w = grid.w();

    if !grid.row(grid.h() - 1).all(empty) {
        grid.push(vec![b'.'; w]);
    }
    while grid.row(grid.h() - 2).all(empty) {
        grid.pop().unwrap();
    }
    grid.push(vec![b'#'; w]);

    let mut path = vec![];
    let mut pos = (500, 0usize);
    let mut count = 0;
    loop {
        path.push(pos);
        if grid[(pos.0, pos.1 + 1)] == b'.' {
            // move down
        } else if grid[(pos.0 - 1, pos.1 + 1)] == b'.' {
            pos.0 -= 1;
        } else if grid[(pos.0 + 1, pos.1 + 1)] == b'.' {
            pos.0 += 1;
        } else {
            path.pop().unwrap();
            grid[pos] = b'o';
            count += 1;
            if path.is_empty() {
                break;
            }
            pos = path.pop().unwrap();
            continue;
        }
        pos.1 += 1;
    }

    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");

    let mut grid = Grid::new_clone((1000, 1000), b'.');

    for l in input.lines() {
        let mut parts = l
            .split(" -> ")
            .map(|s| sscanf!(s, "{usize},{usize}").unwrap());
        let mut pos = parts.next().unwrap();
        for next in parts {
            if pos.0 == next.0 {
                for y in pos.1.min(next.1)..=pos.1.max(next.1) {
                    grid[(pos.0, y)] = b'#';
                }
            } else {
                for x in pos.0.min(next.0)..=pos.0.max(next.0) {
                    grid[(x, pos.1)] = b'#';
                }
            }
            pos = next;
        }
    }

    while grid.row(grid.h() - 1).all(empty) {
        grid.pop().unwrap();
    }
    let bottom = grid.h() - 1;

    let mut path = vec![];
    let mut pos = (500, 0);
    let mut count = 0;
    loop {
        if pos.1 == bottom {
            break;
        }
        path.push(pos);
        if grid[(pos.0, pos.1 + 1)] == b'.' {
            // move down
        } else if grid[(pos.0 - 1, pos.1 + 1)] == b'.' {
            pos.0 -= 1;
        } else if grid[(pos.0 + 1, pos.1 + 1)] == b'.' {
            pos.0 += 1;
        } else {
            path.pop().unwrap();
            grid[pos] = b'o';
            count += 1;
            pos = path.pop().unwrap();
            continue;
        }
        pos.1 += 1;
    }

    pv!(count);
}
