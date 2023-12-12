use std::collections::HashMap;

#[derive(Debug)]
pub struct RowData {
    individuals: Vec<SpringCondition>,
    damaged_groups: Vec<usize>,
}

#[derive(Debug, Copy, Clone)]
enum SpringCondition {
    Operational,
    Damaged,
    Unknown,
}

pub fn generator(input: &str) -> Vec<RowData> {
    input
        .lines()
        .map(|line| {
            let (individuals, damaged_groups) = line.split_once(' ').unwrap();
            RowData {
                individuals: individuals
                    .chars()
                    .map(|ch| match ch {
                        '.' => SpringCondition::Operational,
                        '#' => SpringCondition::Damaged,
                        '?' => SpringCondition::Unknown,
                        _ => panic!("Bad input {ch:?}"),
                    })
                    .collect(),
                damaged_groups: damaged_groups
                    .split(',')
                    .map(|g| g.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

impl RowData {
    fn num_arrangements_with(
        &self,
        individuals_offset: usize,
        damaged_groups_offset: usize,
        memoization: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        let memoization_key = (individuals_offset, damaged_groups_offset);
        if let Some(memoized_result) = memoization.get(&memoization_key) {
            return *memoized_result;
        }
        let individuals = &self.individuals[individuals_offset..];
        let Some(&group_to_match) = self.damaged_groups.get(damaged_groups_offset) else {
            if individuals
                .iter()
                .any(|c| matches!(c, SpringCondition::Damaged))
            {
                // Damaged ahead, cannot match
                return 0;
            } else {
                // No groups to match - only one arrangement (all unknown are operational)
                return 1;
            }
        };

        let mut total = 0;
        for (i, spring_condition) in individuals.iter().enumerate() {
            if individuals.len() < i + group_to_match {
                break;
            }
            if matches!(spring_condition, SpringCondition::Operational) {
                continue;
            }
            let streak_can_end;
            let continue_using;
            if let Some(spring_after) = individuals.get(i + group_to_match) {
                streak_can_end = matches!(
                    spring_after,
                    SpringCondition::Operational | SpringCondition::Unknown
                );
                continue_using = individuals_offset + i + group_to_match + 1;
            } else {
                streak_can_end = true;
                continue_using = individuals_offset + i + group_to_match;
            }
            if streak_can_end {
                let streak = &individuals[i..][..group_to_match];
                if streak
                    .iter()
                    .all(|c| matches!(c, SpringCondition::Damaged | SpringCondition::Unknown))
                {
                    total += self.num_arrangements_with(
                        continue_using,
                        damaged_groups_offset + 1,
                        memoization,
                    );
                }
            }
            if matches!(spring_condition, SpringCondition::Damaged) {
                // No other options
                break;
            }
        }
        memoization.insert(memoization_key, total);
        total
    }

    fn num_arrangements(&self) -> usize {
        self.num_arrangements_with(0, 0, &mut HashMap::new())
    }
}

pub fn part_1(input: &[RowData]) -> usize {
    input.iter().map(|inp| inp.num_arrangements()).sum()
}

pub fn part_2(input: &[RowData]) -> usize {
    input
        .iter()
        .map(|inp| {
            let mut individuals = Vec::new();
            let mut damaged_groups = Vec::new();
            for _ in 0..5 {
                if !individuals.is_empty() {
                    individuals.push(SpringCondition::Unknown);
                }
                individuals.extend_from_slice(&inp.individuals);
                damaged_groups.extend_from_slice(&inp.damaged_groups);
            }
            RowData {
                individuals,
                damaged_groups,
            }
            .num_arrangements()
        })
        .sum()
}
