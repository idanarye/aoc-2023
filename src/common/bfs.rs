use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::hash::Hash;
use std::ops::Add;

struct BfsCell<P, C> {
    parent: Option<P>,
    cost: C,
}

pub struct LinearBfs<C> {
    cells: Vec<Option<BfsCell<usize, C>>>,
    consider: VecDeque<usize>,
}

impl<C> LinearBfs<C>
where
    for<'a> &'a C: Add<&'a C, Output = C>,
    C: PartialOrd<C>,
{
    pub fn new(size: usize) -> Self {
        Self {
            cells: std::iter::repeat_with(|| None).take(size).collect(),
            consider: VecDeque::new(),
        }
    }

    pub fn cost(&self, key: usize) -> Option<&C> {
        self.cells[key].as_ref().map(|cell| &cell.cost)
    }

    pub fn add_root(&mut self, key: usize, cost: C) {
        self.cells[key] = Some(BfsCell { parent: None, cost });
        self.consider.push_back(key)
    }

    pub fn consider_next(&mut self) -> Option<usize> {
        self.consider.pop_front()
    }

    pub fn add_edge(&mut self, parent: usize, key: usize, additional_cost: C) -> bool {
        let new_cost = self.cost(parent).unwrap() + &additional_cost;
        if let Some(existing_cell) = &self.cells[key] {
            // TODO: this may fail because I'm using VecDeque instead of a BinaryHeap
            if new_cost < existing_cell.cost {
                assert!(self.consider.contains(&key), "out of order");
            } else {
                return false;
            }
        }
        self.consider.push_back(key);
        self.cells[key] = Some(BfsCell {
            parent: Some(parent),
            cost: new_cost,
        });
        true
    }

    pub fn path_to(&self, key: usize) -> Vec<usize> {
        let mut result = Vec::new();
        let mut currentlly_at = Some(key);
        while let Some(at) = currentlly_at {
            result.push(at);
            currentlly_at = self.cells[at].as_ref().unwrap().parent;
        }
        result.reverse();
        result
    }
}

#[derive(PartialEq)]
struct HeapCell<K, C> {
    key: K,
    cost: C,
}

impl<K: PartialEq, C: PartialEq> Eq for HeapCell<K, C> {}

impl<K: PartialEq, C: PartialOrd + Eq> PartialOrd for HeapCell<K, C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl<K: PartialEq, C: Ord> Ord for HeapCell<K, C> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

pub struct HashMapBfs<K, C> {
    cells: HashMap<K, BfsCell<K, C>>,
    consider: BinaryHeap<HeapCell<K, C>>,
}

impl<K, C> HashMapBfs<K, C>
where
    K: PartialEq + Eq + Hash + Clone,
    for<'a> &'a C: Add<&'a C, Output = C>,
    C: PartialOrd<C> + Ord + Clone,
{
    pub fn cost(&self, key: &K) -> Option<&C> {
        self.cells.get(key).map(|cell| &cell.cost)
    }

    pub fn add_root(&mut self, key: K, cost: C) {
        self.cells.insert(
            key.clone(),
            BfsCell {
                parent: None,
                cost: cost.clone(),
            },
        );
        self.consider.push(HeapCell { key, cost });
    }

    pub fn consider_next(&mut self) -> Option<K> {
        self.consider.pop().map(|hc| hc.key)
    }

    pub fn add_edge(&mut self, parent: K, key: K, additional_cost: C) -> bool {
        let new_cost = self.cost(&parent).unwrap() + &additional_cost;
        if self.cells.get(&key).is_some() {
            return false;
        }
        self.consider.push(HeapCell {
            key: key.clone(),
            cost: new_cost.clone(),
        });
        self.cells.insert(
            key,
            BfsCell {
                parent: Some(parent),
                cost: new_cost,
            },
        );
        true
    }

    pub fn get_path_to(&self, key: &K) -> Option<Vec<K>> {
        let mut result = Vec::new();
        let mut currentlly_at = Some(key);
        while let Some(at) = currentlly_at {
            result.push(at.clone());
            currentlly_at = self.cells.get(at)?.parent.as_ref();
        }
        result.reverse();
        Some(result)
    }

    pub fn path_to(&self, key: &K) -> Vec<K> {
        self.get_path_to(key).expect("No path found")
    }

    pub fn all_known(&self) -> impl Iterator<Item = &K> {
        self.cells.keys()
    }
}

impl<K, C> Default for HashMapBfs<K, C>
where
    K: PartialEq + Eq + Hash + Clone,
    for<'a> &'a C: Add<&'a C, Output = C>,
    C: PartialOrd<C> + Ord,
{
    fn default() -> Self {
        Self {
            cells: HashMap::new(),
            consider: BinaryHeap::new(),
        }
    }
}
