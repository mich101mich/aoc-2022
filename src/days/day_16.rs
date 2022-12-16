use crate::utils::*;

#[derive(Debug)]
struct Valve {
    rate: usize,
    neighbors: Vec<(usize, usize)>,
}

fn parse(input: &'static str) -> Vec<Valve> {
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

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");

    let valves = parse(input);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct State {
        you_busy_till: usize,
        you_pos: usize,
        elephant_busy_till: usize,
        elephant_pos: usize,
        opened: u32,
    }

    let mut needs_update = vec![HashMap::new(); 27];
    needs_update[0].insert(
        State {
            you_busy_till: 0,
            you_pos: 0,
            elephant_busy_till: 0,
            elephant_pos: 0,
            opened: 0,
        },
        0usize,
    );

    let mut insert =
        |s: State, score: usize, timestamp: &mut HashMap<State, usize>| match timestamp.entry(s) {
            Entry::Occupied(mut entry) => {
                if *entry.get() < score {
                    *entry.get_mut() = score;
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(score);
            }
        };

    for time in 0..26 {
        let current = std::mem::replace(&mut needs_update[time], HashMap::new());
        pv!(time, current.len());
        for (state, score) in current {
            insert(state, score, &mut needs_update[26]);
            let mut possible_next_states = vec![];
            if state.you_busy_till == time {
                for (next, cost) in valves[state.you_pos].neighbors.iter().copied() {
                    let you_busy_till = state.you_busy_till + cost + 1;
                    if you_busy_till > 26 || state.opened & (1 << next) != 0 {
                        continue;
                    }
                    let next_opened = state.opened | (1 << next);
                    let next_score = score + valves[next].rate * (26 - you_busy_till);
                    let next_state = State {
                        you_busy_till,
                        you_pos: next,
                        opened: next_opened,
                        ..state
                    };
                    let next_update = next_state.you_busy_till.min(next_state.elephant_busy_till);
                    if next_update == time {
                        possible_next_states.push((next_score, next_state));
                    } else {
                        insert(next_state, next_score, &mut needs_update[next_update]);
                    }
                }
            } else {
                possible_next_states.push((score, state));
            }

            if state.elephant_busy_till == time {
                for (score, state) in possible_next_states {
                    for (next, cost) in valves[state.elephant_pos].neighbors.iter().copied() {
                        let elephant_busy_till = state.elephant_busy_till + cost + 1;
                        if elephant_busy_till > 26 || state.opened & (1 << next) != 0 {
                            continue;
                        }
                        let next_opened = state.opened | (1 << next);
                        let next_score = score + valves[next].rate * (26 - elephant_busy_till);
                        let next_state = State {
                            elephant_busy_till,
                            elephant_pos: next,
                            opened: next_opened,
                            ..state
                        };
                        let next_update =
                            next_state.you_busy_till.min(next_state.elephant_busy_till);
                        insert(next_state, next_score, &mut needs_update[next_update]);
                    }
                }
            }
        }
    }
    let best = needs_update[26].values().max().unwrap();
    pv!(best);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");

    let valves = parse(input);

    let mut reachable = vec![HashMap::new(); 31];
    reachable[0].insert((0usize, 0u32), 0usize);

    let mut insert =
        |p: (usize, u32), score: usize, reachable: &mut HashMap<(usize, u32), usize>| {
            match reachable.entry(p) {
                Entry::Occupied(mut entry) => {
                    if *entry.get() < score {
                        *entry.get_mut() = score;
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(score);
                }
            }
        };

    for time in 0..30 {
        let current = std::mem::replace(&mut reachable[time], HashMap::new());
        for ((pos, opened), score) in current {
            insert((pos, opened), score, &mut reachable[time + 1]);
            for (next, cost) in valves[pos].neighbors.iter().copied() {
                let next_time = time + cost + 1;
                if next_time > 30 || opened & (1 << next) != 0 {
                    continue;
                }
                let next_opened = opened | (1 << next);
                let next_score = score + valves[next].rate * (30 - next_time);
                insert((next, next_opened), next_score, &mut reachable[next_time]);
            }
        }
    }
    let best = reachable[30].values().max().unwrap();
    pv!(best);
}
