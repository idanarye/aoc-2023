use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use std::ops::Add;

#[derive(Debug)]
struct StackCell<K, C> {
    key: K,
    cost: C,
    children: VecDeque<(K, C)>,
}

#[derive(Debug)]
pub struct HashMapDfs<K, C> {
    root: Option<(K, C)>,
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
            root: Some((root.clone(), root_cost.clone())),
            stack: Vec::new(),
            visited: HashSet::new(),
        }
    }

    pub fn consider_next(&mut self) -> Option<K> {
        loop {
            if let Some((root, root_cost)) = self.root.take() {
                self.visited.insert(root.clone());
                self.stack.push(StackCell {
                    key: root.clone(),
                    cost: root_cost,
                    children: VecDeque::default(),
                });
                return Some(root);
            }
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
            "can only add edges to the currently considered node"
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
            "can only inspect the currently considered node"
        );
        top_cell.cost.clone()
    }

    pub fn path_to(&self, key: &K) -> Vec<K> {
        let top_cell = self.stack.last().expect("No path is being inspected");
        assert!(
            top_cell.key == *key,
            "can only inspect the currently considered node"
        );

        self.stack
            .iter()
            .skip(1)
            .map(|stack_cell| stack_cell.key.clone())
            .collect()
    }

    pub fn parent(&self, key: &K) -> Option<K> {
        let mut rev_it = self.stack.iter().rev();
        let top_cell = rev_it.next().expect("No path is being inspected");
        assert!(
            top_cell.key == *key,
            "can only inspect the currently considered node"
        );
        Some(rev_it.next()?.key.clone())
    }
}
