use crate::utils::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::map,
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
enum Entry {
    Number(usize),
    List(Vec<Entry>),
}
impl FromStr for Entry {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        take_entry(s).map(|(_, e)| e).map_err(|_| ())
    }
}
impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Entry::Number(a), Entry::Number(b)) => a.partial_cmp(b),
            (Entry::List(a), Entry::List(b)) => a.partial_cmp(b),
            (Entry::Number(a), Entry::List(b)) => {
                (&[Entry::Number(*a)][..]).partial_cmp(b.as_slice())
            }
            (Entry::List(a), Entry::Number(b)) => {
                a.as_slice().partial_cmp(&[Entry::Number(*b)][..])
            }
        }
    }
}
impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn take_entry(i: &str) -> IResult<&str, Entry> {
    alt((
        map(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
            Entry::Number(s.parse().unwrap())
        }),
        map(
            delimited(tag("["), separated_list0(tag(","), take_entry), tag("]")),
            |v| Entry::List(v),
        ),
    ))(i)
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");

    let mut parsed = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| (l, false))
        .chain([("[[2]]", true), ("[[6]]", true)])
        .map(|(l, b)| (l.parse::<Entry>().unwrap(), b))
        .collect::<Vec<_>>();
    parsed.sort_unstable();

    let a = parsed.iter().position(|e| e.1).unwrap() + 1;
    let b = parsed[a..].iter().position(|e| e.1).unwrap() + a + 1;
    pv!(a * b)
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");

    let mut lines = input.lines();
    let mut sum = 0;
    for index in 1.. {
        let a = Entry::from_str(lines.next().unwrap()).unwrap();
        let b = Entry::from_str(lines.next().unwrap()).unwrap();

        if a < b {
            sum += index;
        }

        match lines.next() {
            Some(_) => {}
            None => break,
        }
    }
    pv!(sum);
}
