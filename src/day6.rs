use std::ops::Range;

#[derive(Debug)]
pub struct Input {
    time: String,
    distance: String,
}

pub fn generator(input: &str) -> Input {
    let mut it = input.lines();

    let first_line = it.next().unwrap();
    assert!(first_line.starts_with("Time:"));
    let second_line = it.next().unwrap();
    assert!(second_line.starts_with("Distance:"));

    Input {
        time: first_line.split_once(": ").unwrap().1.to_owned(),
        distance: second_line.split_once(": ").unwrap().1.to_owned(),
    }
}

#[derive(Debug)]
pub struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn winning_range(&self) -> Range<usize> {
        // t: race time. d: distance required. p: press time
        // Need p * (t - p) > d, meaning p^2 - tp + d < 0
        // Putting in the quadric formula `(-b +- sqrt(b^2 - 4ac)) / 2a`, we get:
        // 0.5 * (t +- sqrt(t^2 - 4d))
        let discriminant = ((self.time.pow(2) - 4 * self.distance) as f64).sqrt();
        let lower_solution = 0.5 * (self.time as f64 - discriminant);
        let lower_solution = if lower_solution.fract() == 0.0 {
            lower_solution as usize + 1
        } else {
            lower_solution.ceil() as usize
        };
        let higher_solution = 0.5 * (self.time as f64 + discriminant);
        let higher_solution = if higher_solution.fract() == 0.0 {
            higher_solution as usize - 1
        } else {
            higher_solution.floor() as usize
        };
        lower_solution..(higher_solution + 1)
    }
}

pub fn part_1(input: &Input) -> usize {
    let pattern = regex::Regex::new(r#"\d+"#).unwrap();
    pattern
        .find_iter(&input.time)
        .zip(pattern.find_iter(&input.distance))
        .map(|(t, d)| {
            let race = Race {
                time: t.as_str().parse().unwrap(),
                distance: d.as_str().parse().unwrap(),
            };
            race.winning_range().len()
        })
        .product()
}

pub fn part_2(input: &Input) -> usize {
    let race = Race {
        time: input.time.replace(' ', "").parse().unwrap(),
        distance: input.distance.replace(' ', "").parse().unwrap(),
    };
    race.winning_range().len()
}
