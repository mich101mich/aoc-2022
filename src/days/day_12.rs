use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");

    let mut parsed = byte_grid(input);

    let letter_s = parsed.find(b'S').unwrap();
    let start = parsed.find(b'E').unwrap();
    parsed[letter_s] = b'a';
    parsed[start] = b'z';

    let neigh = parsed.manhattan();
    let path = open_dijkstra(
        |pos, out| {
            if parsed[pos] == b'a' {
                return true;
            }
            let min = parsed[pos] - 1;
            out.extend(neigh.get_all_neighbors(pos).filter(|p| parsed[p] >= min));
            false
        },
        start,
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
            out.extend(neigh.get_all_neighbors(pos).filter(|p| parsed[p] <= max));
        },
        start,
        end,
        |p| neigh.heuristic(p, end),
    )
    .unwrap();

    pv!(path.cost);
}
