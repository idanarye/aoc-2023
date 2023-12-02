#[derive(Debug)]
pub struct RowData {
}

pub fn generator(input: &str) -> Vec<RowData> {
    input.lines().map(|line| {
        RowData {
        }
    }).collect()
}

pub fn part_1(input: &[RowData]) -> usize {
    println!("{:?}", input);
    0
}

pub fn part_2(input: &[RowData]) -> usize {
    let _ = input;
    0
}
