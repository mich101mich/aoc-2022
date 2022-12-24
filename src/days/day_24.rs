use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");

    let mut grid = byte_grid(input);
    let bounds = grid.bounds() - p2(2, 2);
    let (mut blizzards, blizzard_dirs): (Vec<_>, Vec<_>) = grid
        .grid_iter_index()
        .filter_map(|(p, &c)| {
            let s = (c as char).to_string();
            sscanf!(s, "{Dir}").ok().map(|dir| (p - p2(1, 1), dir))
        })
        .unzip();

    let mut has_blizzard = Grid::new_clone(bounds, false);
    for &p in &blizzards {
        has_blizzard[p] = true;
    }
    let mut next_blizzards = blizzards.clone();
    let mut sim_blizzards = |has_blizzard: &mut Grid<bool>| {
        for ((cur, dir), next) in blizzards
            .iter()
            .zip(&blizzard_dirs)
            .zip(&mut next_blizzards)
        {
            *next = dir.wrapping_add(*cur, bounds);
        }
        std::mem::swap(&mut blizzards, &mut next_blizzards);
        has_blizzard.fill(false);
        for &p in &blizzards {
            has_blizzard[p] = true;
        }
    };

    let mut positions = HashSet::new();
    let mut next_positions = HashSet::new();

    let mut find_path = |start, goal| {
        positions.clear();
        positions.insert(start);
        for minute in 1.. {
            sim_blizzards(&mut has_blizzard);

            next_positions.clear();
            for p in &positions {
                if has_blizzard.get(*p) == Some(&false) || *p == start {
                    next_positions.insert(*p);
                }
                next_positions.extend(
                    Dir::all()
                        .map(|dir| *p + dir)
                        .filter(|&p| has_blizzard.get(p) == Some(&false)),
                );
            }
            std::mem::swap(&mut positions, &mut next_positions);

            if positions.contains(&goal) {
                sim_blizzards(&mut has_blizzard);
                positions.clear();
                positions.insert(goal);
                return minute + 1;
            }
        }
        unreachable!()
    };

    let start_a = p2(0, -1isize);
    let start_b = p2(bounds.x as isize - 1, bounds.y as isize);
    let goal_a = p2(bounds.x as isize - 1, bounds.y as isize - 1);
    let goal_b = p2(0, 0);

    let mut minutes = 0;
    minutes += find_path(start_a, goal_a);
    minutes += find_path(start_b, goal_b);
    minutes += find_path(start_a, goal_a);
    pv!(minutes);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");

    let mut grid = byte_grid(input);
    let bounds = grid.bounds() - p2(2, 2);
    let (mut blizzards, blizzard_dirs): (Vec<_>, Vec<_>) = grid
        .grid_iter_index()
        .filter_map(|(p, &c)| {
            let s = (c as char).to_string();
            sscanf!(s, "{Dir}").ok().map(|dir| (p - p2(1, 1), dir))
        })
        .unzip();

    let mut has_blizzard = Grid::new_clone(bounds, false);
    for &p in &blizzards {
        has_blizzard[p] = true;
    }

    let mut positions = HashSet::new();
    positions.insert(p2(0, -1isize));

    let mut next_blizzards = blizzards.clone();
    let mut next_positions = HashSet::new();
    for minute in 1.. {
        for ((cur, dir), next) in blizzards
            .iter()
            .zip(&blizzard_dirs)
            .zip(&mut next_blizzards)
        {
            *next = dir.wrapping_add(*cur, bounds);
        }
        std::mem::swap(&mut blizzards, &mut next_blizzards);
        has_blizzard.fill(false);
        for &p in &blizzards {
            has_blizzard[p] = true;
        }

        next_positions.clear();
        for p in &positions {
            if has_blizzard.get(*p) == Some(&false) || *p == p2(0, -1isize) {
                next_positions.insert(*p);
            }
            next_positions.extend(
                Dir::all()
                    .map(|dir| *p + dir)
                    .filter(|&p| has_blizzard.get(p) == Some(&false)),
            );
        }
        std::mem::swap(&mut positions, &mut next_positions);

        if positions.contains(&p2(bounds.x as isize - 1, bounds.y as isize - 1)) {
            println!("minutes: {}", minute + 1);
            break;
        }
    }
}
