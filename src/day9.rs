use itertools::Itertools;

pub fn generator(input: &str) -> Vec<Vec<isize>> {
    let pattern = regex::Regex::new(r#"[-\d]+"#).unwrap();
    input
        .lines()
        .map(|line| {
            pattern
                .find_iter(line)
                .map(|m| m.as_str().parse().unwrap())
                .collect()
        })
        .collect()
}

fn derive_once(sequence: &[isize]) -> Vec<isize> {
    sequence
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect()
}

fn next_item(sequence: &[isize]) -> isize {
    let derived = derive_once(sequence);
    let last_item = sequence.last().copied().unwrap_or(0);
    if derived.iter().all(|d| *d == 0) {
        last_item
    } else {
        last_item + next_item(&derived)
    }
}

pub fn part_1(input: &[Vec<isize>]) -> isize {
    input.iter().map(|sequence| next_item(sequence)).sum()
}

pub fn part_2(input: &[Vec<isize>]) -> isize {
    input
        .iter()
        .map(|sequence| {
            let mut sequence = sequence.clone();
            sequence.reverse();
            next_item(&sequence)
        })
        .sum()
}
