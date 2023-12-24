use std::ops::RangeInclusive;

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Trajectory {
    pos: [isize; 3],
    vel: [isize; 3],
}

pub fn generator(input: &str) -> Vec<Trajectory> {
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
            Trajectory {
                pos: parse_vec3(pos),
                vel: parse_vec3(vel),
            }
        })
        .collect()
}

impl Trajectory {
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

    fn subtract_velocity(&self, velocity: [isize; 3]) -> Trajectory {
        Trajectory {
            pos: self.pos,
            vel: [
                self.vel[0] - velocity[0],
                self.vel[1] - velocity[1],
                self.vel[2] - velocity[2],
            ],
        }
    }
}

pub fn part_1(input: &[Trajectory]) -> usize {
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

pub fn part_2(input: &[Trajectory]) -> isize {
    (0..)
        .flat_map(|md| {
            (-md..=md).flat_map(move |x: isize| {
                let y = md - x.abs();
                [[x, y]].into_iter().chain((y != 0).then_some([x, -y]))
            })
        })
        .find_map(|[vx, vy]| {
            let rock_vel = [vx, vy, 0];
            let mut it = input[1..].iter().map(|other| {
                let (t1, _) = input[0]
                    .subtract_velocity(rock_vel)
                    .collision_times_xy(&other.subtract_velocity(rock_vel))
                    .unwrap();
                t1
            });
            let t0 = it.next().unwrap();
            if !it.all(|time| (t0 - time).abs() < 0.1) {
                return None;
            }

            let [px, py, pz0] = input[0].subtract_velocity(rock_vel).pos_at_time(t0);

            let mut it = input[1..].iter().map(|hailstone| {
                let hailstone = hailstone.subtract_velocity(rock_vel);
                let t = (px - hailstone.pos[0] as f64) / hailstone.vel[0] as f64;
                let [hpx, hpy, hpz] = hailstone.pos_at_time(t);
                assert!((hpx - px).abs() < 0.1);
                assert!((hpy - py).abs() < 0.1);
                (t, hpz)
            });

            let (t1, pz1) = it.next().unwrap();

            let vz = (pz1 - pz0) / (t1 - t0);

            for (t, pz) in it {
                if 0.1 < (vz - (pz - pz0) / (t - t0)) {
                    return None;
                }
            }

            let rock_vel = [vx, vy, vz as isize];
            Some(
                input[0]
                    .pos_at_time(t0)
                    .iter()
                    .zip(rock_vel)
                    .map(|(p0, rock_vel)| p0 - (rock_vel as f64 * t0))
                    .sum::<f64>() as isize,
            )
        })
        .unwrap()
}
