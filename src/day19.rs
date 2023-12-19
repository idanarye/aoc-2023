use std::collections::HashMap;
use std::ops::{Index, IndexMut, Range};
use std::str::FromStr;
use std::string::ParseError;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Category {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

impl Category {
    const ALL: [Self; 4] = [
        Self::ExtremelyCoolLooking,
        Self::Musical,
        Self::Aerodynamic,
        Self::Shiny,
    ];

    fn idx(&self) -> usize {
        match self {
            Category::ExtremelyCoolLooking => 0,
            Category::Musical => 1,
            Category::Aerodynamic => 2,
            Category::Shiny => 3,
        }
    }
}

impl FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::ExtremelyCoolLooking),
            "m" => Ok(Self::Musical),
            "a" => Ok(Self::Aerodynamic),
            "s" => Ok(Self::Shiny),
            _ => Err(format!("Invalid category {s:?}")),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Verdict {
    Accept,
    Reject,
    Transfer(String),
}

impl FromStr for Verdict {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Transfer(s.to_owned()),
        })
    }
}

#[derive(Debug)]
enum Condition {
    LessThen(usize),
    AtLeast(usize),
}

impl Condition {
    fn number(&self) -> usize {
        match self {
            Condition::LessThen(number) => *number,
            Condition::AtLeast(number) => *number,
        }
    }

    fn check(&self, value: usize) -> bool {
        match self {
            Condition::LessThen(number) => value < *number,
            Condition::AtLeast(number) => *number <= value,
        }
    }
}

#[derive(Debug)]
pub struct Rule {
    condition: Option<(Category, Condition)>,
    verdict: Verdict,
}

#[derive(Debug)]
pub struct Workflow(Vec<Rule>);

#[derive(Debug)]
pub struct Part([usize; 4]);

#[derive(Debug, Clone, PartialEq)]
pub struct PartRange([Range<usize>; 4]);

impl PartRange {
    fn new(init: Range<usize>) -> Self {
        Self([(); 4].map(|_| init.clone()))
    }
    fn start(&self) -> Part {
        Part(self.0.clone().map(|range| range.start))
    }

    fn gen_advancements<'a>(&'a self, other: &'a Self) -> impl 'a + Iterator<Item = Self> {
        let mut this = self.clone();
        Category::ALL.into_iter().filter_map(move |category| {
            let mut advancement = this.clone();
            advancement[category].start = other[category].end;
            this[category].end = other[category].end;
            (!advancement.is_empty()).then_some(advancement)
        })
    }

    fn is_empty(&self) -> bool {
        self.0.iter().any(|range| range.is_empty())
    }

    fn area(&self) -> usize {
        self.0.iter().map(|range| range.len()).product()
    }
}

impl Index<Category> for Part {
    type Output = usize;

    fn index(&self, index: Category) -> &Self::Output {
        &self.0[index.idx()]
    }
}

impl Index<Category> for PartRange {
    type Output = Range<usize>;

    fn index(&self, index: Category) -> &Self::Output {
        &self.0[index.idx()]
    }
}

impl IndexMut<Category> for PartRange {
    fn index_mut(&mut self, index: Category) -> &mut Self::Output {
        &mut self.0[index.idx()]
    }
}

#[derive(Debug)]
pub struct Input {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

pub fn generator(input: &str) -> Input {
    let mut it = input.lines();

    let workflow_pattern = Regex::new(r#"^(\w+)\{(.*)\}$"#).unwrap();
    let rule_pattern = Regex::new(r#"^(\w)([<>])(\d+):(\w+)$"#).unwrap();
    let workflows = it
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let m = workflow_pattern.captures(line).unwrap();
            let workflow_name = m[1].to_owned();
            let rules = m[2]
                .split(',')
                .map(|rule_text| {
                    if let Some(m) = rule_pattern.captures(rule_text) {
                        let category = m[1].parse().unwrap();
                        let threshold = m[3].parse().unwrap();
                        let condition = match &m[2] {
                            "<" => Condition::LessThen(threshold),
                            ">" => Condition::AtLeast(threshold + 1),
                            invalid => panic!("Bad condition {invalid:?}"),
                        };
                        let verdict = m[4].parse().unwrap();
                        Rule {
                            condition: Some((category, condition)),
                            verdict,
                        }
                    } else {
                        Rule {
                            condition: None,
                            verdict: rule_text.parse().unwrap(),
                        }
                    }
                })
                .collect_vec();
            (workflow_name, Workflow(rules))
        })
        .collect();

    let part_pattern = Regex::new(r#"^\{(.*)\}$"#).unwrap();
    let parts = it
        .map(|part_text| {
            let mut result = [0; 4];
            for property_text in part_pattern.captures(part_text).unwrap()[1].split(',') {
                let (category_text, value_text) = property_text.split_once('=').unwrap();
                let category: Category = category_text.parse().unwrap();
                let value: usize = value_text.parse().unwrap();
                result[category.idx()] = value;
            }
            Part(result)
        })
        .collect_vec();

    Input { workflows, parts }
}

impl Rule {
    fn check_part(&self, part: &Part) -> Option<Verdict> {
        if let Some((category, condition)) = &self.condition {
            let value = part[*category];
            if !condition.check(value) {
                return None;
            }
        }
        Some(self.verdict.clone())
    }

    fn check_part_range(&self, part: &PartRange) -> (PartRange, Option<Verdict>) {
        let mut part = part.clone();
        if let Some((category, condition)) = &self.condition {
            let relevant_range = &mut part[*category];
            let split_at_number = condition.number();
            if relevant_range.start < split_at_number && split_at_number < relevant_range.end {
                relevant_range.end = split_at_number;
            }
        }
        let result = self.check_part(&part.start());
        (part, result)
    }
}

impl Workflow {
    fn check_part(&self, part: &Part) -> Verdict {
        self.0
            .iter()
            .find_map(|rule| rule.check_part(part))
            .expect("Finished workflow without verdict")
    }

    fn check_part_range(&self, part: &PartRange) -> (PartRange, Verdict) {
        let mut part = part.clone();
        for rule in self.0.iter() {
            let (reduced_part, check_result) = rule.check_part_range(&part);
            if let Some(verdict) = check_result {
                return (reduced_part, verdict);
            }
            part = reduced_part;
        }
        panic!("Finished workflow without verdict");
    }
}

impl Input {
    fn check_part(&self, part: &Part) -> bool {
        let mut workflow = &self.workflows["in"];
        loop {
            match workflow.check_part(part) {
                Verdict::Accept => return true,
                Verdict::Reject => return false,
                Verdict::Transfer(new_workflow) => {
                    workflow = &self.workflows[&new_workflow];
                }
            }
        }
    }

    fn check_part_range(&self, part: &PartRange) -> (PartRange, bool) {
        let mut part = part.clone();
        let mut workflow = &self.workflows["in"];
        loop {
            let (reduced_part, check_result) = workflow.check_part_range(&part);
            match check_result {
                Verdict::Accept => return (reduced_part, true),
                Verdict::Reject => return (reduced_part, false),
                Verdict::Transfer(new_workflow) => {
                    part = reduced_part;
                    workflow = &self.workflows[&new_workflow];
                }
            }
        }
    }
}

impl Part {
    fn rating(&self) -> usize {
        self.0.iter().sum()
    }
}

pub fn part_1(input: &Input) -> usize {
    input
        .parts
        .iter()
        .filter(|part| input.check_part(part))
        .map(|part| part.rating())
        .sum()
}

pub fn part_2(input: &Input) -> usize {
    let mut to_check = vec![PartRange::new(1..4001)];
    std::iter::from_fn(|| {
        let part_range = to_check.pop()?;
        let (covered_range, is_accepted) = input.check_part_range(&part_range);
        for new_range in part_range.gen_advancements(&covered_range) {
            to_check.push(new_range);
        }
        if is_accepted {
            Some(covered_range.area())
        } else {
            Some(0)
        }
    })
    .sum()
}
