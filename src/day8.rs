use std::collections::HashMap;

use itertools::Itertools;
use num::Integer;

#[derive(Debug)]
pub struct Input {
    directions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

pub fn generator(input: &str) -> Input {
    let mut it = input.lines();
    let directions = it
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!(),
        })
        .collect_vec();
    assert!(it.next() == Some(""));

    let pattern = regex::Regex::new(r#"^(\w+) = \((\w+), (\w+)\)$"#).unwrap();
    let nodes = it
        .map(|line| {
            let m = pattern.captures(line).unwrap();
            (m[1].to_owned(), (m[2].to_owned(), m[3].to_owned()))
        })
        .collect();
    Input { directions, nodes }
}

impl Input {
    fn gen_directions(&self) -> impl '_ + Iterator<Item = Direction> {
        self.directions.iter().copied().cycle()
    }

    fn do_step(&self, from_node: &str, direction: Direction) -> Option<&str> {
        let (left, right) = self.nodes.get(from_node)?;
        Some(match direction {
            Direction::Left => left,
            Direction::Right => right,
        })
    }

    fn walk_from<'a>(&'a self, mut node: &'a str) -> impl 'a + Iterator<Item = &str> {
        self.gen_directions().map_while(move |direction| {
            let result = node;
            node = self.do_step(node, direction)?;
            Some(result)
        })
    }
}

pub fn part_1(input: &Input) -> usize {
    if !input.nodes.contains_key("AAA") {
        // Cannot do part 1 with example input for part 2
        return 0;
    }
    input
        .walk_from("AAA")
        .take_while(|node| *node != "ZZZ")
        .count()
}

#[derive(Debug)]
struct CycleDescr {
    offset: usize,
    length: usize,
}

fn join_cycles(mut first: CycleDescr, mut second: CycleDescr) -> CycleDescr {
    if second.offset < first.offset {
        std::mem::swap(&mut first, &mut second);
    }
    let mut offset = second.offset;
    while (offset - first.offset) % first.length != 0 {
        offset += second.length;
    }
    CycleDescr {
        offset,
        length: first.length.lcm(&second.length),
    }
}

pub fn part_2(input: &Input) -> usize {
    let cycles = input
        .nodes
        .keys()
        .filter_map(|node| {
            if !node.ends_with('A') {
                return None;
            }
            let mut walk = input.walk_from(node);
            let offset = walk.by_ref().take_while(|n| !n.ends_with('Z')).count();
            let length = walk.take_while(|n| !n.ends_with('Z')).count() + 1;
            Some(CycleDescr { offset, length })
        })
        .collect_vec();
    cycles.into_iter().reduce(join_cycles).unwrap().offset
}
