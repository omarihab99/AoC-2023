mod helper;
use pathfinding::{
    matrix::{directions, Matrix},
    prelude::astar,
};
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct State {
    position: (usize, usize),
    direction: (isize, isize),
    distance: i32,
}
fn main() {
    let grid = helper::read_input();
    println!("{}", part_one(&grid, 1, 3));
}
fn part_one(grid: &Matrix<i32>, min: i32, max: i32) -> i32 {
    let start_state = State {
        position: (0, 0),
        direction: (0, 0),
        distance: 0,
    };
    let end_state_position = (grid.rows - 1, grid.columns - 1);
    let path = astar(
        &start_state,
        |state| match state.distance >= min || state.direction == (0, 0) {
            true => get_neighbours(&grid, state, &start_state, max),
            false => get_next(grid, state),
        },
        |state| {
            (end_state_position.0.abs_diff(state.position.0)
                + end_state_position.1.abs_diff(state.position.1)) as i32
        },
        |state| state.position == end_state_position,
    );
    path.unwrap().1
}
fn get_neighbours(
    grid: &Matrix<i32>,
    state: &State,
    start_state: &State,
    max: i32,
) -> Vec<(State, i32)> {
    [directions::N, directions::S, directions::E, directions::W]
        .iter()
        .flat_map(|direction| {
            grid.move_in_direction(state.position, *direction)
                .map(|point| (point, *direction, *grid.get(point).unwrap()))
        })
        .filter(|(position, direction, _)| {
            let inverse = state.direction.0 == -direction.0 && state.direction.1 == -direction.1;
            !inverse && *position != start_state.position
        })
        .flat_map(|(position, direction, weight)| {
            let distance = match state.direction == direction {
                true => state.distance + 1,
                false => 1,
            };
            match distance <= max {
                true => Some((
                    State {
                        position,
                        direction,
                        distance,
                    },
                    weight,
                )),
                false => None,
            }
        })
        .collect::<Vec<_>>()
}
fn get_next(grid: &Matrix<i32>, state: &State) -> Vec<(State, i32)> {
    match grid.move_in_direction(state.position, state.direction) {
        Some(point) => {
            let weight = *grid.get(point).unwrap();
            let next_state = State{
                position: point,
                direction: state.direction,
                distance: state.distance+1,
            };
            vec![(next_state, weight)]
        }
        None => Vec::with_capacity(0),
    }
}