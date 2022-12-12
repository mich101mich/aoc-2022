use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");

    let mut parsed = byte_grid(input);

    let start = parsed.find(b'S').unwrap();
    let end = parsed.find(b'E').unwrap();
    parsed[start] = b'a';
    parsed[end] = b'z';

    let goal = (parsed.w() + 2, 0);

    let neigh = parsed.manhattan();
    let path = a_star_search(
        |pos, out| {
            if parsed[pos] == b'a' {
                out.push((goal, 0));
            }
            let min = parsed[pos].saturating_sub(1);
            for p in neigh.get_all_neighbors(pos) {
                if parsed[p] >= min {
                    out.push((p, 1));
                }
            }
        },
        end,
        goal,
        |_| 0,
    )
    .unwrap();

    pv!(path.cost);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");

    let mut parsed = byte_grid(input);

    let start = parsed.find(b'S').unwrap();
    let end = parsed.find(b'E').unwrap();
    parsed[start] = b'a';
    parsed[end] = b'z';

    let neigh = parsed.manhattan();
    let path = a_star_search(
        |pos, out| {
            let max = parsed[pos] + 1;
            for p in neigh.get_all_neighbors(pos) {
                if parsed[p] <= max {
                    out.push(p);
                }
            }
        },
        start,
        end,
        |p| neigh.heuristic(p, end),
    )
    .unwrap();

    pv!(path.cost);
}
