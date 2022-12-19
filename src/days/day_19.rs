use crate::utils::*;
use std::collections::BTreeSet;

type Blueprint = [u32; 4];

fn geode(n: u32) -> u8 {
    (n & 0xff) as u8
}
fn obsidian(n: u32) -> u8 {
    ((n >> 8) & 0xff) as u8
}
fn clay(n: u32) -> u8 {
    ((n >> 16) & 0xff) as u8
}
fn ore(n: u32) -> u8 {
    ((n >> 24) & 0xff) as u8
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    robots: u32,
    resources: u32,
}
impl State {
    fn new() -> Self {
        Self {
            robots: 0x01_00_00_00,
            resources: 0,
        }
    }
    fn has_resources(&self, cost: u32) -> bool {
        ore(self.resources) >= ore(cost)
            && clay(self.resources) >= clay(cost)
            && obsidian(self.resources) >= obsidian(cost)
            && geode(self.resources) >= geode(cost)
    }
    fn step(mut self, blueprint: &Blueprint, out: &mut BTreeSet<State>) {
        let added_resources = self.robots;
        for (i, cost) in blueprint.iter().enumerate() {
            if self.has_resources(*cost) {
                let mut new_state = self;
                new_state.robots += 1 << ((3 - i) * 8);
                new_state.resources = new_state.resources - *cost + added_resources;
                out.insert(new_state);
            }
        }
        self.resources += added_resources;
        out.insert(self);
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        geode(self.resources)
            .cmp(&geode(other.resources))
            .then_with(|| geode(self.robots).cmp(&geode(other.robots)))
            .then_with(|| obsidian(self.resources).cmp(&obsidian(other.resources)))
            .then_with(|| obsidian(self.robots).cmp(&obsidian(other.robots)))
            .then_with(|| clay(self.resources).cmp(&clay(other.resources)))
            .then_with(|| clay(self.robots).cmp(&clay(other.robots)))
            .then_with(|| ore(self.resources).cmp(&ore(other.resources)))
            .then_with(|| ore(self.robots).cmp(&ore(other.robots)))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");

    let parsed = input
        .lines()
        .take(3)
        .map(|l| sscanf!(l, "Blueprint {usize}: Each ore robot costs {u8} ore. Each clay robot costs {u8} ore. Each obsidian robot costs {u8} ore and {u8} clay. Each geode robot costs {u8} ore and {u8} obsidian.").unwrap());

    let mut blueprints = vec![];
    for (id, ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian) in parsed {
        let costs = [
            u32::from_be_bytes([ore_ore, 0, 0, 0]),
            u32::from_be_bytes([clay_ore, 0, 0, 0]),
            u32::from_be_bytes([obsidian_ore, obsidian_clay, 0, 0]),
            u32::from_be_bytes([geode_ore, 0, geode_obsidian, 0]),
        ];
        blueprints.push(costs);
    }

    let product: usize = blueprints
        .par_iter()
        .map(|costs| {
            let mut states = BTreeSet::new();
            states.insert(State::new());

            let mut new_states = BTreeSet::new();
            for time in 0..32 {
                new_states.clear();
                for state in &states {
                    state.step(costs, &mut new_states);
                }
                std::mem::swap(&mut states, &mut new_states);
                while states.len() > 10000 {
                    states.pop_first();
                }
            }
            let best = states.iter().map(|s| geode(s.resources)).max().unwrap();
            best as usize
        })
        .product();

    pv!(product);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");

    let parsed = input
        .lines()
        .map(|l| sscanf!(l, "Blueprint {usize}: Each ore robot costs {u8} ore. Each clay robot costs {u8} ore. Each obsidian robot costs {u8} ore and {u8} clay. Each geode robot costs {u8} ore and {u8} obsidian.").unwrap());

    let mut blueprints = vec![];
    for (id, ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian) in parsed {
        let costs = [
            u32::from_be_bytes([ore_ore, 0, 0, 0]),
            u32::from_be_bytes([clay_ore, 0, 0, 0]),
            u32::from_be_bytes([obsidian_ore, obsidian_clay, 0, 0]),
            u32::from_be_bytes([geode_ore, 0, geode_obsidian, 0]),
        ];
        blueprints.push((id, costs));
    }

    let sum: usize = blueprints
        .par_iter()
        .map(|(id, costs)| {
            let mut states = BTreeSet::new();
            states.insert(State::new());

            let mut new_states = BTreeSet::new();
            for time in 0..24 {
                new_states.clear();
                for state in &states {
                    state.step(costs, &mut new_states);
                }
                std::mem::swap(&mut states, &mut new_states);
                while states.len() > 10000 {
                    states.pop_first();
                }
            }
            let best = states.iter().map(|s| geode(s.resources)).max().unwrap();
            let quality = *id * best as usize;
            quality
        })
        .sum();

    pv!(sum);
}
