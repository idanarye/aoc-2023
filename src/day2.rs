type Pull = [usize; 3];

#[derive(Debug)]
pub struct Game {
    #[allow(dead_code)]
    game_id: usize,
    pulls: Vec<Pull>,
}

#[derive(Copy, Clone, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn idx(&self) -> usize {
        match self {
            Color::Red => 0,
            Color::Green => 1,
            Color::Blue => 2,
        }
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => panic!("Bad color {value}"),
        }
    }
}

pub fn generator(input: &str) -> Vec<Game> {
    let pattern = regex::Regex::new(r#"(\d+) (\w+)"#).unwrap();
    input
        .lines()
        .map(|line| {
            let (game_descr, game_parts) = line.split_once(": ").unwrap();
            let game_id = game_descr.split_once(' ').unwrap().1.parse().unwrap();
            let pulls = game_parts
                .split("; ")
                .map(|part| {
                    let mut result = Pull::default();
                    for c in pattern.captures_iter(part) {
                        let amount = c[1].parse::<usize>().unwrap();
                        let color: Color = c[2].into();
                        result[color.idx()] = amount;
                    }
                    result
                })
                .collect();
            Game { game_id, pulls }
        })
        .collect()
}

fn is_possible(bag: &Pull, pull: &Pull) -> bool {
    bag.iter().zip(pull).all(|(b, p)| p <= b)
}

impl Game {
    fn is_possible(&self, bag: &Pull) -> bool {
        self.pulls.iter().all(|pull| is_possible(bag, pull))
    }
}

pub fn part_1(input: &[Game]) -> usize {
    input
        .iter()
        .filter(|game| game.is_possible(&[12, 13, 14]))
        .map(|game| game.game_id)
        .sum()
}

fn pull_max(a: &Pull, b: &Pull) -> Pull {
    [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])]
}

impl Game {
    fn min_bag(&self) -> Pull {
        let mut result = Pull::default();
        for pull in self.pulls.iter() {
            result = pull_max(&result, pull);
        }
        result
    }
}

fn pull_power(pull: &Pull) -> usize {
    pull.iter().product()
}

pub fn part_2(input: &[Game]) -> usize {
    input.iter().map(|game| pull_power(&game.min_bag())).sum()
}
