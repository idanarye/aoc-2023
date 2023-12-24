use std::ops::RangeInclusive;

use itertools::Itertools;

#[derive(Debug)]
pub struct RowData {
    pos: [isize; 3],
    vel: [isize; 3],
}

pub fn generator(input: &str) -> Vec<RowData> {
    input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once('@').unwrap();
            fn parse_vec3(txt: &str) -> [isize; 3] {
                txt.split(',')
                    .map(|coord_text| coord_text.trim().parse().unwrap())
                    .collect_vec()
                    .try_into()
                    .unwrap()
            }
            RowData {
                pos: parse_vec3(pos),
                vel: parse_vec3(vel),
            }
        })
        .collect()
}

impl RowData {
    fn collision_times_xy(&self, other: &Self) -> Option<(f64, f64)> {
        // Mark:
        // - ps, po = position of self, other
        // - vs, vo = velocity of self, other
        // - ts, to = time for self, other
        // Need `to` find ts and/or `to` that satisfy:
        // ps + ts * vs = po + to * vo
        // ts * vs - to * vo = (po - ps)
        // So I need to diagonalize the matrix:
        // | vs.x  -vo.x  (po - ps).x |
        // | vs.y  -vo.y  (po - ps).y |
        // Let's mark `dp` = `po - ps`:
        // | vs.x  -vo.x  dp.x |
        // | vs.y  -vo.y  dp.y |
        // Which means subtracting the first row times `rm = (vs.y / vs.x)` (`rm` stands for "row
        // multilier") from the second row, making it:
        // | 0   -vo.y + vo.x * rm   dp.y - dp.x * rm |

        let dp = [
            (other.pos[0] - self.pos[0]) as f64,
            (other.pos[1] - self.pos[1]) as f64,
        ];
        let rm = self.vel[1] as f64 / self.vel[0] as f64;
        let time_other = (dp[1] - dp[0] * rm) / (-other.vel[1] as f64 + other.vel[0] as f64 * rm);

        // Placing this in the first formula, we get:
        // ts * vs.x = dp.x + to * vo.x
        // ts = (dp.x + to * vo.x) / vs.x

        let time_self = (dp[0] + time_other * other.vel[0] as f64) / self.vel[0] as f64;

        Some((time_self, time_other))
    }

    fn pos_at_time(&self, time: f64) -> [f64; 3] {
        [
            self.pos[0] as f64 + time * self.vel[0] as f64,
            self.pos[1] as f64 + time * self.vel[1] as f64,
            self.pos[2] as f64 + time * self.vel[2] as f64,
        ]
    }
}

pub fn part_1(input: &[RowData]) -> usize {
    const LIMIT: RangeInclusive<f64> = 200000000000000.0..=400000000000000.0;
    input
        .iter()
        .enumerate()
        .map(|(i, this)| {
            input[(i + 1)..]
                .iter()
                .filter(|that| {
                    let (time_this, time_that) = this.collision_times_xy(that).unwrap();
                    if time_this < 0.0 || time_that < 0.0 {
                        return false;
                    }
                    let collision = this.pos_at_time(time_this);
                    collision.iter().take(2).all(|coord| LIMIT.contains(coord))
                })
                .count()
        })
        .sum()
}

pub fn part_2(input: &[RowData]) -> usize {
    let _ = input;
    0
}
