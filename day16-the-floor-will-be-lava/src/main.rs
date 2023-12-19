mod grid;
mod helper;

use grid::{get_result, Direction};

fn main() {
    let grid = helper::read_input();
    println!("Part one: {}", get_result(&grid, 0, 0, Direction::Right));
    println!("Part two: {}", calculate_part_two(&grid));
}

fn calculate_part_two(grid: &Vec<Vec<char>>) -> i32 {
    let mut v = Vec::new();
    for i in 0..grid.len() {
        v.push(get_result(grid, i, 0, Direction::Right));
        v.push(get_result(grid, i, grid[0].len() - 1, Direction::Left));
    }

    for j in 0..grid[0].len() {
        v.push(get_result(grid, 0, j, Direction::Down));
        v.push(get_result(grid, grid.len() - 1, j, Direction::Up));
    }
    *v.iter().max().unwrap_or(&0)
}
