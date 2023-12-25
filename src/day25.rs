use std::collections::{HashMap, HashSet};
use std::ops::Index;

use itertools::Itertools;

use crate::common::bfs::HashMapBfs;

#[derive(Debug, Clone)]
pub struct RowData {
    node: String,
    edges: Vec<String>,
}

pub fn generator(input: &str) -> Vec<RowData> {
    input
        .lines()
        .map(|line| {
            let (node, edges) = line.split_once(": ").unwrap();
            RowData {
                node: node.trim().to_owned(),
                edges: edges
                    .split(' ')
                    .map(|edge| edge.trim().to_owned())
                    .collect_vec(),
            }
        })
        .collect()
}

#[derive(Debug)]
struct Graph(HashMap<usize, HashSet<usize>>);

impl Index<usize> for Graph {
    type Output = HashSet<usize>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[&index]
    }
}

impl Graph {
    fn new(input: &[RowData]) -> Self {
        let mut node_to_idx = input
            .iter()
            .enumerate()
            .map(|(i, node)| (node.node.as_str(), i))
            .collect::<HashMap<&str, usize>>();
        let mut graph = HashMap::<usize, HashSet<usize>>::new();
        for RowData { node, edges } in input {
            let node = node_to_idx[node.as_str()];
            for edge in edges {
                let new_edge_index = node_to_idx.len();
                let edge = match node_to_idx.entry(edge.as_str()) {
                    std::collections::hash_map::Entry::Occupied(entry) => *entry.get(),
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        *entry.insert(new_edge_index)
                    }
                };
                graph.entry(node).or_default().insert(edge);
                graph.entry(edge).or_default().insert(node);
            }
        }
        Self(graph)
    }

    fn run_bfs(
        &self,
        root: usize,
        mut edge_filter: impl FnMut(usize, usize) -> Option<usize>,
        mut finish_pred: impl FnMut(usize) -> bool,
    ) -> HashMapBfs<usize, usize> {
        let mut bfs = HashMapBfs::default();
        bfs.add_root(root, 0);
        while let Some(node) = bfs.consider_next() {
            if finish_pred(node) {
                break;
            }
            for edge in self[node].iter() {
                if let Some(cost) = edge_filter(node, *edge) {
                    bfs.add_edge(node, *edge, cost);
                }
            }
        }
        bfs
    }

    fn find_path(
        &self,
        from: usize,
        to: usize,
        edge_filter: impl FnMut(usize, usize) -> Option<usize>,
    ) -> Option<Vec<usize>> {
        self.run_bfs(from, edge_filter, |node| node == to)
            .get_path_to(&to)
    }

    fn split_by_removing_edges(
        &self,
        edges_to_remove: &[(usize, usize)],
        max_subclusters: usize,
    ) -> Option<Vec<Vec<usize>>> {
        let edges_to_remove = edges_to_remove
            .iter()
            .flat_map(|&(f, t)| [(f, t), (t, f)])
            .collect::<HashSet<(usize, usize)>>();
        let mut result = Vec::new();
        let mut nodes_to_check = self.0.keys().copied().collect::<HashSet<usize>>();
        while let Some(&root) = nodes_to_check.iter().next() {
            if max_subclusters <= result.len() {
                return None;
            }
            let bfs = self.run_bfs(
                root,
                |f, t| (!edges_to_remove.contains(&(f, t))).then_some(1),
                |_| false,
            );
            nodes_to_check.retain(|node| bfs.cost(node).is_none());
            let subcluster = bfs.all_known().copied().collect_vec();
            result.push(subcluster);
        }
        Some(result)
    }

    fn solve(&self, from: usize, to: usize) -> Option<usize> {
        let first_path = self
            .find_path(from, to, |f, t| ((f, t) != (from, to)).then_some(1))
            .unwrap();
        let edges_in_first_path = first_path
            .iter()
            .copied()
            .tuple_windows()
            .collect::<HashSet<(usize, usize)>>();

        let second_path = self
            .find_path(from, to, |f, t| {
                if (f, t) == (from, to) {
                    None
                } else if edges_in_first_path.contains(&(f, t)) {
                    Some(2 * self.0.len())
                } else {
                    Some(1)
                }
            })
            .unwrap();

        let [first_path_edges_ud, second_path_edges_ud] = [&first_path, &second_path].map(|path| {
            path.iter()
                .copied()
                .tuple_windows()
                .map(|(a, b)| if a < b { (a, b) } else { (b, a) })
                .collect::<HashSet<(usize, usize)>>()
        });

        let common_edges = first_path_edges_ud
            .intersection(&second_path_edges_ud)
            .copied()
            .collect::<HashSet<(usize, usize)>>();

        let [first_iter, second_iter] =
            [first_path_edges_ud, second_path_edges_ud].map(|edges_ud| {
                edges_ud
                    .into_iter()
                    .filter(|edge| !common_edges.contains(edge))
            });

        for (first, second) in first_iter.cartesian_product(second_iter.collect_vec()) {
            let edges_to_remove = &[(from, to), first, second];
            if let Some(subclusters) = self.split_by_removing_edges(edges_to_remove, 2) {
                if subclusters.len() == 2 {
                    return Some(subclusters[0].len() * subclusters[1].len());
                }
            }
        }
        None
    }
}

pub fn part_1(input: &[RowData]) -> usize {
    let graph = Graph::new(input);
    graph
        .0
        .iter()
        .flat_map(|(node, edges)| {
            edges
                .iter()
                .filter(move |edge| node < edge)
                .map(move |edge| (node, edge))
        })
        .find_map(|(from, to)| graph.solve(*from, *to))
        .unwrap()
}
