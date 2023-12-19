use crate::common::bfs::HashMapBfs;
use crate::common::direction::Direction;
use crate::common::vmatrix::VMatrix;

pub fn generator(input: &str) -> VMatrix<usize> {
    VMatrix::from_chars(input, |ch| {
        ch.is_ascii_digit().then_some(ch as usize).unwrap() - '0' as usize
    })
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    coord: [usize; 2],
    steps: usize,
    direction: Direction,
}

fn solve(
    input: &VMatrix<usize>,
    can_go_straight: impl Fn(usize) -> bool,
    can_turn: impl Fn(usize) -> bool,
) -> usize {
    let mut bfs = HashMapBfs::default();
    for direction in [Direction::East, Direction::South] {
        bfs.add_root(
            State {
                coord: [0, 0],
                steps: 100, // to force a turn
                direction,
            },
            0,
        );
    }

    while let Some(state) = bfs.consider_next() {
        if state.coord == [input.rows - 1, input.cols - 1] && can_turn(state.steps + 1) {
            return *bfs.cost(&state).unwrap();
        }

        let straight_option =
            can_go_straight(state.steps + 1).then_some((state.direction, state.steps + 1));
        let turn_options = can_turn(state.steps + 1).then_some([
            (state.direction.clockwise(), 0),
            (state.direction.counter_clockwise(), 0),
        ]);

        for (direction, steps) in straight_option
            .into_iter()
            .chain(turn_options.into_iter().flatten())
        {
            if let Some(coord) = input.motion(state.coord, direction.motion()) {
                bfs.add_edge(
                    state.clone(),
                    State {
                        coord,
                        steps,
                        direction,
                    },
                    input[coord],
                );
            }
        }
    }

    panic!("No path found");
}

pub fn part_1(input: &VMatrix<usize>) -> usize {
    solve(input, |steps| steps < 3, |_| true)
}

pub fn part_2(input: &VMatrix<usize>) -> usize {
    solve(input, |steps| steps < 10, |steps| 4 <= steps)
}
