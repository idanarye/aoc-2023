use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Sub;

use itertools::Itertools;

#[derive(Debug)]
#[allow(unused)]
pub struct NumWarp<N> {
    pub mapper: HashMap<N, usize>,
    pub expands: Vec<WarpingExpansion<N>>,
    pub virtual_size: usize,
}

#[derive(Debug)]
pub enum WarpingExpansion<N> {
    Single(N),
    Range { prev: N, next: N },
}

impl<N> WarpingExpansion<N>
where
    N: Sub + Clone + PartialOrd,
    usize: TryFrom<<N as Sub>::Output>,
    <usize as TryFrom<<N as Sub>::Output>>::Error: Debug,
{
    pub fn is_empty(&self) -> bool {
        match self {
            WarpingExpansion::Single(_) => false,
            WarpingExpansion::Range { prev, next } => next <= prev,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            WarpingExpansion::Single(_) => 1,
            WarpingExpansion::Range { prev, next } => {
                let plus_one: usize = (next.clone() - prev.clone()).try_into().unwrap();
                plus_one - 1
            }
        }
    }
}

impl<N> NumWarp<N>
where
    N: Sub + Hash + Eq + Ord + Clone,
    usize: TryFrom<<N as Sub>::Output>,
    <usize as TryFrom<<N as Sub>::Output>>::Error: Debug,
{
    pub fn new(points: impl Iterator<Item = N>) -> Self {
        let mut points = points.collect_vec();
        points.sort();

        let mut mapper = HashMap::new();
        let mut expands = Vec::new();

        let mut points = points.into_iter();

        let first = points.next().unwrap();
        mapper.insert(first.clone(), 0);
        expands.push(WarpingExpansion::Single(first.clone()));
        let mut prev = first;

        for point in points {
            if point <= prev {
                continue;
            }
            let dist: usize = (point.clone() - prev.clone()).try_into().unwrap();
            if 1 < dist {
                expands.push(WarpingExpansion::Range {
                    prev,
                    next: point.clone(),
                });
            }
            prev = point.clone();

            mapper.insert(point.clone(), expands.len());
            expands.push(WarpingExpansion::Single(point));
        }

        let virtual_size = mapper.values().max().unwrap() + 1;

        Self {
            mapper,
            expands,
            virtual_size,
        }
    }
}
