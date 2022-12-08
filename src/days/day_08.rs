use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");

    let parsed = digit_grid(input);

    let max_dist = parsed
        .grid_iter_index()
        .map(|(p, v)| {
            Dir::all()
                .map(|dir| {
                    parsed
                        .dir_iter(p, dir)
                        .take_while_inclusive(|pos| parsed[pos] < *v)
                        .count()
                })
                .product::<usize>()
        })
        .max()
        .unwrap();

    pv!(max_dist);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");

    let parsed = digit_grid(input);

    let count = parsed
        .grid_iter_index()
        .filter(|(p, v)| {
            Dir::all().any(|dir| parsed.dir_iter(*p, dir).all(|pos| parsed[pos] < **v))
        })
        .count();

    pv!(count);
}
