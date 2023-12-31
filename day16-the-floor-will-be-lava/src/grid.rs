use std::collections::HashSet;
#[derive(Clone, Eq, Hash, PartialEq)]
pub(crate) enum Direction {
    Left,
    Right,
    Down,
    Up,
}
impl Direction {
    fn new(prev_direction: Direction, state: State) -> Vec<Self> {
        match state {
            State::EmptySpace => vec![prev_direction],
            State::HorizontalSplitter => match prev_direction {
                Direction::Left | Direction::Right => vec![prev_direction],
                Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
            },
            State::VerticalSplitter => match prev_direction {
                Direction::Up | Direction::Down => vec![prev_direction],
                Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
            },
            State::ForwardSlashMirror => match prev_direction {
                Direction::Right => vec![Direction::Up],
                Direction::Up => vec![Direction::Right],
                Direction::Left => vec![Direction::Down],
                Direction::Down => vec![Direction::Left],
            },
            State::BackslashMirror => match prev_direction {
                Direction::Right => vec![Direction::Down],
                Direction::Up => vec![Direction::Left],
                Direction::Left => vec![Direction::Up],
                Direction::Down => vec![Direction::Right],
            },
        }
    }
}
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub(crate) enum State {
    BackslashMirror,
    ForwardSlashMirror,
    EmptySpace,
    HorizontalSplitter,
    VerticalSplitter,
}

impl State {
    fn new(value: char) -> Self {
        match value {
            '|' => State::VerticalSplitter,
            '-' => State::HorizontalSplitter,
            '.' => State::EmptySpace,
            '\\' => State::BackslashMirror,
            '/' => State::ForwardSlashMirror,
            _ => unimplemented!(),
        }
    }
}
#[derive(Eq, Hash, PartialEq, Clone)]
pub(crate) struct Node {
    state: State,
    x: usize,
    y: usize,
    next_dir: Vec<Direction>,
}
pub(crate) fn get_result(grid: &Vec<Vec<char>>, i: usize, j: usize, initial_dir: Direction) -> i32 {
    let mut result: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];
    let mut visited: HashSet<Node> = HashSet::new();
    let current_node_state = State::new(grid[i][j]);
    let current_node = Node {
        state: current_node_state,
        x: i,
        y: j,
        next_dir: Direction::new(initial_dir, current_node_state),
    };
    visited.insert(current_node.clone());
    dfs(grid, &mut visited, &current_node, &mut result);
    result.iter().flatten().map(|e| *e.min(&1)).sum::<i32>()
}

fn dfs(
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<Node>,
    current_node: &Node,
    result: &mut Vec<Vec<i32>>,
) {
    let x = current_node.x;
    let y = current_node.y;
    if x >= grid.len() || y >= grid[0].len() {
        return;
    }
    result[x][y] += 1;
    for dir in &current_node.next_dir {
        let (new_x, new_y) = determine_index(x as i32, y as i32, dir.clone());
        if new_x < 0 || new_y < 0 || new_x >= grid.len() as i32 || new_y >= grid[0].len() as i32 {
            continue;
        }
        let new_state = State::new(grid[new_x as usize][new_y as usize]);
        let new_node = Node {
            state: new_state,
            x: new_x as usize,
            y: new_y as usize,
            next_dir: Direction::new(dir.clone(), new_state),
        };
        if visited.contains(&new_node) {
            continue;
        }
        visited.insert(new_node.clone());
        dfs(grid, visited, &new_node, result);
    }
}

fn determine_index(x: i32, y: i32, direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Left => (x, y - 1),
        Direction::Right => (x, y + 1),
        Direction::Down => (x + 1, y),
        Direction::Up => (x - 1, y),
    }
}
