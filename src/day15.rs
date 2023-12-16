use itertools::Itertools;

pub fn generator(input: &str) -> Vec<String> {
    input
        .lines()
        .flat_map(|line| line.split(',').map(|s| s.to_owned()))
        .collect()
}

fn calc_hash(text: &str) -> usize {
    let mut current_value = 0;
    for ch in text.chars() {
        current_value += ch as usize;
        current_value *= 17;
        current_value &= 0xFF;
    }
    current_value
}

pub fn part_1(input: &[String]) -> usize {
    input.iter().map(|step| calc_hash(step)).sum()
}

#[derive(Debug)]
enum Instruction<'a> {
    Insert { label: &'a str, focal_length: usize },
    Remove { label: &'a str },
}

impl<'a> Instruction<'a> {
    fn new(text: &'a str) -> Self {
        if let Some((label, "")) = text.rsplit_once('-') {
            Self::Remove { label }
        } else if let Some((label, focal_length)) = text.rsplit_once('=') {
            let focal_length = focal_length.parse().unwrap();
            Self::Insert {
                label,
                focal_length,
            }
        } else {
            panic!("Illegal instruction {text:?}")
        }
    }

    fn label(&'a self) -> &'a str {
        match self {
            Instruction::Insert { label, .. } => label,
            Instruction::Remove { label } => label,
        }
    }

    fn label_hash(&'a self) -> usize {
        calc_hash(self.label())
    }
}

pub fn part_2(input: &[String]) -> usize {
    let mut boxes = vec![Vec::<(&str, usize)>::new(); 256];
    for instruction_text in input.iter() {
        let instruction = Instruction::new(instruction_text);
        let b = &mut boxes[instruction.label_hash()];
        match instruction {
            Instruction::Insert {
                label,
                focal_length,
            } => {
                if let Some((_, existing)) = b.iter_mut().find(|(l, _)| *l == label) {
                    *existing = focal_length;
                } else {
                    b.push((label, focal_length));
                }
            }
            Instruction::Remove { label } => {
                if let Some((index, _)) = b.iter().find_position(|(l, _)| *l == label) {
                    b.remove(index);
                }
            }
        }
    }
    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_number, box_lenses)| {
            let box_number = box_number + 1;
            box_lenses
                .iter()
                .enumerate()
                .map(move |(slot_number, (_, focal_length))| {
                    let slot_number = slot_number + 1;
                    box_number * slot_number * focal_length
                })
        })
        .sum()
}
