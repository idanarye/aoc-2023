use std::collections::{BTreeMap, HashMap};
use std::ops::Range;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct Input {
    seeds: Vec<usize>,
    mappings: HashMap<String, Mapping>,
}

#[derive(Debug)]
struct Mapping {
    destination: String,
    ranges: BTreeMap<usize, MappingRangeDestination>,
}

#[derive(Debug)]
struct MappingRangeDestination {
    destination: usize,
    length: usize,
}

pub fn generator(input: &str) -> Input {
    let seeds_pattern = Regex::new(r#"^seeds: ([\d ]*)$"#).unwrap();
    let mapping_header_pattern = Regex::new(r#"^(\w+)-to-(\w+) map:$"#).unwrap();
    let mapping_pattern = Regex::new(r#"^(\d+) (\d+) (\d+)$"#).unwrap();

    let mut seeds = Vec::new();
    let mut mappings = HashMap::new();
    let mut current_mapping = None;

    for line in input.lines() {
        if let Some(m) = seeds_pattern.captures(line) {
            seeds.extend(m[1].split(' ').map(|seed| seed.parse::<usize>().unwrap()));
        } else if let Some(m) = mapping_header_pattern.captures(line) {
            mappings.insert(
                m[1].to_owned(),
                Mapping {
                    destination: m[2].to_owned(),
                    ranges: Default::default(),
                },
            );
            current_mapping = mappings.get_mut(&m[1]);
        } else if let Some(m) = mapping_pattern.captures(line) {
            let mapping = current_mapping.as_mut().unwrap();
            mapping.ranges.insert(
                m[2].parse().unwrap(),
                MappingRangeDestination {
                    destination: m[1].parse().unwrap(),
                    length: m[3].parse().unwrap(),
                },
            );
        } else {
            assert!(line.is_empty());
        }
    }

    Input { seeds, mappings }
}

impl Mapping {
    fn resolve(&self, source: usize) -> usize {
        if let Some((source_start, mapping_range)) = self.ranges.range(..=source).last() {
            let offset = source - source_start;
            if offset < mapping_range.length {
                return mapping_range.destination + offset;
            }
        }
        source
    }

    fn resolve_range_longest(&self, source: Range<usize>) -> Range<usize> {
        if let Some((source_start, mapping_range)) = self.ranges.range(..=source.start).last() {
            let offset = source.start - source_start;
            if offset < mapping_range.length {
                let dest_start = mapping_range.destination + offset;
                let dest_end = mapping_range.destination + mapping_range.length;
                return dest_start..(dest_end.min(dest_start + source.len()));
            }
        }
        if let Some((next_start, _)) = self.ranges.range(source.start..).next() {
            return source.start..*next_start;
        }
        source
    }
}

impl Input {
    fn chain_resolve(
        &self,
        source_name: &str,
        source_index: usize,
        destination_name: &str,
    ) -> usize {
        let mut source_name = source_name;
        let mut source_index = source_index;
        while source_name != destination_name {
            let mapping = &self.mappings[source_name];
            source_name = &mapping.destination;
            source_index = mapping.resolve(source_index);
        }
        source_index
    }

    fn chain_resolve_range_longest(
        &self,
        source_name: &str,
        source_range: Range<usize>,
        destination_name: &str,
    ) -> Range<usize> {
        let mut source_name = source_name;
        let mut source_range = source_range;
        while source_name != destination_name {
            let mapping = &self.mappings[source_name];
            source_name = &mapping.destination;
            source_range = mapping.resolve_range_longest(source_range);
        }
        source_range
    }

    fn chain_resolve_range<'a>(
        &'a self,
        source_name: &'a str,
        source_range: Range<usize>,
        destination_name: &'a str,
    ) -> impl 'a + Iterator<Item = Range<usize>> {
        let mut source_range = source_range;
        std::iter::from_fn(move || {
            if source_range.is_empty() {
                None
            } else {
                let result = self.chain_resolve_range_longest(
                    source_name,
                    source_range.clone(),
                    destination_name,
                );
                assert!(!result.is_empty());
                source_range.start += result.len();
                Some(result)
            }
        })
    }
}

pub fn part_1(input: &Input) -> usize {
    input
        .seeds
        .iter()
        .map(|seed| input.chain_resolve("seed", *seed, "location"))
        .min()
        .unwrap()
}

pub fn part_2(input: &Input) -> usize {
    input
        .seeds
        .iter()
        .copied()
        .tuples()
        .flat_map(|(from, length)| {
            let range = from..(from + length);
            input
                .chain_resolve_range("seed", range, "location")
                .map(|range| range.start)
        })
        .min()
        .unwrap()
}
