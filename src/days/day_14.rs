use crate::utils::*;

fn empty(c: &u8) -> bool {
    *c == b'.'
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");

    let mut grid = Grid::new_clone(p2(1000, 1000), b'.');

    for l in input.lines() {
        let mut parts = l
            .split(" -> ")
            .map(|s| sscanf!(s, "{usize},{usize}").unwrap())
            .map(|(x, y)| p2(x, y));
        let mut pos = parts.next().unwrap();
        for next in parts {
            let tl = p2(pos.x.min(next.x), pos.y.min(next.y));
            let br = p2(pos.x.max(next.x) + 1, pos.y.max(next.y) + 1);
            grid.fill_rect(tl, br, b'#');
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
    let mut pos = p2(500, 0usize);
    let mut count = 0;
    loop {
        let down = pos + Down;
        if let Some(next) = [down, down + Left, down + Right]
            .iter()
            .find(|p| grid[*p] == b'.')
        {
            path.push(pos);
            pos = *next;
        } else {
            grid[pos] = b'o';
            count += 1;
            if path.is_empty() {
                break;
            }
            pos = path.pop().unwrap();
        }
    }

    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");

    let mut grid = Grid::new_clone(p2(1000, 1000), b'.');

    for l in input.lines() {
        let mut parts = l
            .split(" -> ")
            .map(|s| sscanf!(s, "{usize},{usize}").unwrap())
            .map(|(x, y)| p2(x, y));
        let mut pos = parts.next().unwrap();
        for next in parts {
            let tl = p2(pos.x.min(next.x), pos.y.min(next.y));
            let br = p2(pos.x.max(next.x) + 1, pos.y.max(next.y) + 1);
            grid.fill_rect(tl, br, b'#');
            pos = next;
        }
    }

    while grid.row(grid.h() - 1).all(empty) {
        grid.pop().unwrap();
    }
    let bottom = grid.h() - 1;

    let mut path = vec![];
    let mut pos = p2(500, 0);
    let mut count = 0;
    loop {
        if pos.y == bottom {
            break;
        }
        let down = pos + Down;
        if let Some(next) = [down, down + Left, down + Right]
            .iter()
            .find(|p| grid[*p] == b'.')
        {
            path.push(pos);
            pos = *next;
        } else {
            grid[pos] = b'o';
            count += 1;
            pos = path.pop().unwrap();
        }
    }

    pv!(count);
}
