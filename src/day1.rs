use std::collections::HashMap;

pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_owned()).collect()
}

struct Solver {
    pattern_for_first: regex::Regex,
    pattern_for_last: regex::Regex,
    mapping: HashMap<String, usize>,
}

impl Solver {
    fn new(mapping: &[(&str, usize)]) -> Self {
        let pattern = mapping
            .iter()
            .map(|(txt, _)| *txt)
            .collect::<Vec<_>>()
            .join("|");
        Self {
            pattern_for_first: regex::Regex::new(&pattern).unwrap(),
            pattern_for_last: regex::Regex::new(&format!("^.*({pattern})")).unwrap(),
            mapping: mapping
                .iter()
                .map(|&(txt, num)| (txt.to_owned(), num))
                .collect(),
        }
    }

    fn solve_line(&self, line: &str) -> usize {
        if let (Some(m_first), Some(m_last)) = (
            self.pattern_for_first.find(line),
            self.pattern_for_last.captures(line),
        ) {
            self.mapping[m_first.as_str()] * 10 + self.mapping[&m_last[1]]
        } else {
            0
        }
    }

    fn solve_input(&self, input: &[String]) -> usize {
        input.iter().map(|line| self.solve_line(line)).sum()
    }
}

pub fn part_1(input: &[String]) -> usize {
    Solver::new(&[
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ])
    .solve_input(input)
}

pub fn part_2(input: &[String]) -> usize {
    Solver::new(&[
        ("0", 0),
        ("zero", 0),
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ])
    .solve_input(input)
}
