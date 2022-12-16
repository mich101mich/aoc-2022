use crate::utils::*;

#[derive(Debug)]
struct Valve {
    rate: usize,
    neighbors: Vec<(usize, usize)>,
}

fn parse(input: &str) -> Vec<Valve> {
    let mut index_map = HashMap::new();
    index_map.insert("AA", 0);
    let mut next_index = 1;
    let mut ret = vec![];
    let mut names = vec![];

    let mut full_graph = HashMap::new();

    let iter = input.lines().map(|l| {
        sscanf_unescaped!(
            l,
            "Valve {str} has flow rate={usize}; tunnels? leads? to valves? {str}"
        )
        .unwrap()
    });
    for (name, rate, targets) in iter {
        if rate > 0 || name == "AA" {
            let index = *index_map.entry(name).or_insert_with(|| {
                next_index += 1;
                next_index - 1
            });
            while ret.len() <= index {
                ret.push(Valve {
                    rate: 0,
                    neighbors: vec![],
                });
                names.push("");
            }
            ret[index].rate = rate;
            names[index] = name;
        }
        full_graph.insert(name, targets.split(", ").to_set());
    }

    let n = next_index;
    assert!(n <= 32);
    for id in 0..n {
        let goals = &names[id + 1..];
        let paths = dijkstra_search(
            |p, out: &mut Vec<&str>| {
                if let Some(neighbors) = full_graph.get(p) {
                    out.extend(neighbors);
                }
            },
            names[id],
            goals,
        );
        for (goal, path) in paths {
            let goal = index_map[goal];
            ret[id].neighbors.push((goal, path.cost));
            ret[goal].neighbors.push((id, path.cost));
        }
    }
    ret
}

fn solve(total_time: usize, input: &str) -> Vec<(u32, usize)> {
    let valves = parse(input);

    let mut reachable = vec![HashMap::new(); total_time + 1];
    reachable[0].insert((0usize, 0u32), 0usize);

    fn insert(p: (usize, u32), score: usize, reachable: &mut HashMap<(usize, u32), usize>) {
        match reachable.entry(p) {
            Entry::Occupied(mut entry) => {
                let value = entry.get_mut();
                *value = score.max(*value);
            }
            Entry::Vacant(entry) => {
                entry.insert(score);
            }
        }
    }

    for time in 0..total_time {
        let current = std::mem::take(&mut reachable[time]);
        for ((pos, opened), score) in current {
            insert((pos, opened), score, &mut reachable[total_time]);
            for (next, cost) in valves[pos].neighbors.iter().copied() {
                let next_time = time + cost + 1;
                if next_time > total_time || opened & (1 << next) != 0 {
                    continue;
                }
                let next_opened = opened | (1 << next);
                let next_score = score + valves[next].rate * (total_time - next_time);
                insert((next, next_opened), next_score, &mut reachable[next_time]);
            }
        }
    }

    reachable[total_time]
        .iter()
        .map(|((_, opened), score)| (*opened, *score))
        .collect()
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");

    let states = solve(26, input);

    let best = states
        .iter()
        .enumerate()
        .flat_map(|(i, you)| {
            states[i + 1..]
                .iter()
                .filter(|elephant| you.0 & elephant.0 == 0)
                .map(|elephant| you.1 + elephant.1)
        })
        .max()
        .unwrap();
    pv!(best);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");

    let states = solve(30, input);
    let best = states.iter().map(|(_, score)| score).max().unwrap();
    pv!(best);
}
