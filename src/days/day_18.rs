use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");

    let cubes = input
        .lines()
        .map(|l| sscanf!(l, "{usize},{usize},{usize}").unwrap())
        .map(|(x, y, z)| p3(x + 1, y + 1, z + 1))
        .to_set();

    let bounds = cubes.iter().fold(Point3D::zero(), |max, p| {
        p3(max.x.max(p.x + 2), max.y.max(p.y + 2), max.z.max(p.z + 2))
    });

    let mut is_outside = vec![vec![vec![false; bounds.z]; bounds.y]; bounds.x];

    let mut queue = vec![Point3D::zero()];
    while let Some(next) = queue.pop() {
        is_outside[next.x][next.y][next.z] = true;
        queue.extend(
            Dir3D::all()
                .filter_map(|dir| dir.bounded_add(next, bounds))
                .filter(|p| !cubes.contains(p) && !is_outside[p.x][p.y][p.z]),
        );
    }

    let mut sum = 0;
    for pos in &cubes {
        let mut open = 6;
        for dir in Dir3D::all() {
            if let Some(other) = dir.bounded_add(*pos, bounds) {
                if !is_outside[other.x][other.y][other.z] {
                    open -= 1;
                }
            }
        }
        sum += open;
    }

    pv!(sum);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");

    let cubes = input
        .lines()
        .map(|l| sscanf!(l, "{usize},{usize},{usize}").unwrap())
        .map(|(x, y, z)| p3(x, y, z))
        .to_set();

    let mut sum = 0;
    for pos in &cubes {
        let closed = Dir3D::all()
            .filter_map(|dir| dir.checked_add(*pos))
            .filter(|p| cubes.contains(p))
            .count();
        sum += 6 - closed;
    }

    pv!(sum);
}
