use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use num::Integer;
use regex::Regex;

use crate::common::bfs::HashMapBfs;

#[derive(Debug, Clone)]
pub struct ModuleSpec {
    name: String,
    module_type: ModuleType,
    outputs: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModuleType {
    Interface,
    FlipFlop,
    Conjunction,
}

pub fn generator(input: &str) -> Vec<ModuleSpec> {
    let pattern = Regex::new(r#"^([%&])?(\w+) -> (.*)$"#).unwrap();
    input
        .lines()
        .map(|line| {
            let m = pattern.captures(line).unwrap();
            ModuleSpec {
                name: m[2].to_owned(),
                module_type: match m.get(1).map(|c| c.as_str()) {
                    None => ModuleType::Interface,
                    Some("%") => ModuleType::FlipFlop,
                    Some("&") => ModuleType::Conjunction,
                    Some(t) => panic!("Unknown module type prefix {t:?}"),
                },
                outputs: m[3].split(", ").map(|name| name.to_owned()).collect(),
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PulseType {
    Low,
    High,
}

impl PulseType {
    fn idx(&self) -> usize {
        match self {
            PulseType::Low => 0,
            PulseType::High => 1,
        }
    }
}

type Topology = HashMap<String, ModuleSpec>;

#[derive(Debug)]
struct State {
    topology: Topology,
    flip_flops: HashMap<String, bool>,
    conjunctions: HashMap<String, HashMap<String, PulseType>>,
}

impl State {
    fn new(input: &[ModuleSpec]) -> Self {
        let topology: Topology = input.iter().map(|m| (m.name.clone(), m.clone())).collect();

        let flip_flops = topology
            .values()
            .filter(|m| m.module_type == ModuleType::FlipFlop)
            .map(|m| (m.name.clone(), false))
            .collect();
        let mut conjunctions: HashMap<String, HashMap<String, PulseType>> = topology
            .values()
            .filter(|m| m.module_type == ModuleType::Conjunction)
            .map(|m| (m.name.clone(), HashMap::new()))
            .collect();
        for module in topology.values() {
            for output in module.outputs.iter() {
                if let Some(conjuction_memory) = conjunctions.get_mut(output) {
                    conjuction_memory.insert(module.name.clone(), PulseType::Low);
                }
            }
        }
        Self {
            topology,
            flip_flops,
            conjunctions,
        }
    }

    fn pulse_from(&self, name: &str) -> PulseType {
        let module = self.topology.get(name).unwrap();
        match module.module_type {
            ModuleType::Interface => PulseType::Low,
            ModuleType::FlipFlop => {
                if self.flip_flops[name] {
                    PulseType::High
                } else {
                    PulseType::Low
                }
            }
            ModuleType::Conjunction => {
                if self.conjunctions[name]
                    .values()
                    .all(|p| *p == PulseType::High)
                {
                    PulseType::Low
                } else {
                    PulseType::High
                }
            }
        }
    }

    fn send_pulse(
        &mut self,
        target: &str,
        pulse_type: PulseType,
        mut listener: impl FnMut(&str, PulseType, &str),
    ) -> [usize; 2] {
        let mut pulse_queue = VecDeque::<(&str, PulseType, &str)>::new();
        pulse_queue.push_back(("", pulse_type, target));

        let mut counts = [0, 0];

        while let Some((source, pulse_type, target)) = pulse_queue.pop_front() {
            listener(source, pulse_type, target);
            counts[pulse_type.idx()] += 1;
            let Some(module) = self.topology.get(target) else {
                continue;
            };
            let out_pulse = match module.module_type {
                ModuleType::Interface => pulse_type, // just resend the pulse
                ModuleType::FlipFlop => {
                    match pulse_type {
                        PulseType::Low => {
                            let flip_flop = self.flip_flops.get_mut(target).unwrap();
                            *flip_flop = !*flip_flop;
                            self.pulse_from(target)
                        }
                        PulseType::High => {
                            // Flip flops ignore high pulses and do not retransmit anything
                            continue;
                        }
                    }
                }
                ModuleType::Conjunction => {
                    let conjunction = self.conjunctions.get_mut(target).unwrap();
                    *conjunction.get_mut(source).unwrap() = pulse_type;
                    self.pulse_from(target)
                }
            };
            for output in module.outputs.iter() {
                pulse_queue.push_back((target, out_pulse, output));
            }
        }

        counts
    }
}

pub fn part_1(input: &[ModuleSpec]) -> usize {
    let mut state = State::new(input);
    let mut total_counts = [0; 2];
    for _ in 0..1000 {
        let counts = state.send_pulse("broadcaster", PulseType::Low, |_, _, _| {});
        for (t, c) in total_counts.iter_mut().zip(counts) {
            *t += c;
        }
    }
    total_counts.into_iter().product()
}

pub fn part_2(input: &[ModuleSpec]) -> usize {
    let all_outputs = input
        .iter()
        .flat_map(|m| m.outputs.iter())
        .cloned()
        .collect::<HashSet<String>>();
    let mut reverse_graph = HashMap::<String, Vec<String>>::new();
    for module in input.iter() {
        for output in module.outputs.iter() {
            reverse_graph
                .entry(output.clone())
                .or_default()
                .push(module.name.clone());
        }
    }
    let mut clusters = HashMap::<String, HashSet<String>>::new();
    for ending_point in all_outputs.iter() {
        let mut bfs = HashMapBfs::default();
        bfs.add_root(ending_point, 0);
        while let Some(name) = bfs.consider_next() {
            if let Some(outputs) = reverse_graph.get(name) {
                for output in outputs.iter() {
                    bfs.add_edge(name, output, 1);
                }
            }
        }
        clusters.insert(
            ending_point.clone(),
            bfs.all_known().copied().cloned().collect(),
        );
    }

    let mut state = State::new(input);

    let mut deps = reverse_graph["rx"].clone();
    for _i in 0.. {
        let new_deps = deps
            .iter()
            .flat_map(|d| reverse_graph[d].iter().cloned())
            .collect_vec();
        if 4 < new_deps.len() {
            break;
        }
        deps = new_deps.clone();
    }

    // NOTE: the assumption is that such relevant subclusters need to all send high signal. This is
    // just how the input is structured.
    let relevant_subclusters = deps
        .iter()
        .map(|d| {
            let mut c = clusters[d].iter().cloned().collect_vec();
            c.sort();
            c
        })
        .collect_vec();

    let mut visited_states = relevant_subclusters
        .iter()
        .map(|_| HashMap::<Vec<PulseType>, usize>::new())
        .collect_vec();
    let mut detected_cycles: Vec<Option<usize>> =
        relevant_subclusters.iter().map(|_| None).collect_vec();
    for i in 0.. {
        let mut result = Some(());
        state.send_pulse(
            "broadcaster",
            PulseType::Low,
            |_source, pulse_type, target| {
                if target == "rx" && pulse_type == PulseType::Low {
                    result = None;
                }
            },
        );

        for (cluster, (visited, detected)) in relevant_subclusters
            .iter()
            .zip(visited_states.iter_mut().zip(detected_cycles.iter_mut()))
        {
            let cluster_state = cluster.iter().map(|m| state.pulse_from(m)).collect_vec();
            match visited.entry(cluster_state) {
                std::collections::hash_map::Entry::Occupied(entry) => {
                    if detected.is_none() {
                        assert_eq!(*entry.get(), 0);
                        *detected = Some(i);
                    }
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(i);
                }
            }
        }
        if detected_cycles.iter().all(|c| c.is_some()) {
            break;
        }
    }
    let mut result = 1;
    for cycle in detected_cycles {
        // I don't know why it's always at the end of the cycle, but for some reason it does.
        result = result.lcm(&cycle.unwrap());
    }
    result
}
