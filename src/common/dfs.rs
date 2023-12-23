use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use std::ops::Add;

struct StackCell<K, C> {
    key: K,
    cost: C,
    children: VecDeque<(K, C)>,
}

pub struct HashMapDfs<K, C> {
    stack: Vec<StackCell<K, C>>,
    visited: HashSet<K>,
}

impl<K, C> HashMapDfs<K, C>
where
    K: PartialEq + Eq + Hash + Clone,
    for<'a> &'a C: Add<&'a C, Output = C>,
    C: Clone,
{
    pub fn new(root: K, root_cost: C) -> Self {
        Self {
            stack: vec![StackCell {
                key: root.clone(),
                cost: root_cost.clone(),
                children: [(root, root_cost)].into_iter().collect(),
            }],
            visited: HashSet::new(),
        }
    }

    pub fn consider_next(&mut self) -> Option<K> {
        loop {
            let top_cell = self.stack.last_mut()?;
            if let Some((key, cost)) = top_cell.children.pop_front() {
                self.visited.insert(key.clone());
                self.stack.push(StackCell {
                    key: key.clone(),
                    cost,
                    children: VecDeque::default(),
                });
                return Some(key);
            } else {
                self.visited.remove(&top_cell.key);
                self.stack.pop();
            }
        }
    }

    pub fn add_edge(&mut self, parent: &K, key: K, additional_cost: C) -> bool {
        let top_cell = self.stack.last_mut().expect("No path is being inspected");
        assert!(
            top_cell.key == *parent,
            "add_edge can only add edges to the currently considered node"
        );
        if self.visited.contains(&key) {
            return false;
        }
        let new_cost = &top_cell.cost + &additional_cost;
        top_cell.children.push_back((key, new_cost));
        true
    }

    pub fn was_in_current_path(&self, key: &K) -> bool {
        let top_cell = self.stack.last().expect("No path is being inspected");
        if top_cell.key == *key {
            return false;
        }
        self.visited.contains(key)
    }

    pub fn cost(&self, key: &K) -> C {
        let top_cell = self.stack.last().expect("No path is being inspected");
        assert!(
            top_cell.key == *key,
            "cost can only inspect the currently considered node"
        );
        top_cell.cost.clone()
    }
}
