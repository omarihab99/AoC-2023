use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};
mod helper;

fn main() {
    let (input, start) = helper::read_input();
    println!("{}", part_one(&input, start));
    println!("{}", part_two(&input, start));
}
fn part_two(input: &Vec<Vec<char>>, start: (usize, usize)) -> i32 {
    let mut stack = vec![(start.0, start.1)];
    let mut visited: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut polygon_loop = vec![(start.0 as i32, start.1 as i32)];
    visited.insert((start.0, start.1));
    while let Some((row, col)) = stack.pop() {
        let current_char = input[row][col];
        for (adj, row_d, col_d) in get_neighbour_nodes(row, col, &input) {
            match (adj, current_char, row_d, col_d) {
                ('|', '7' | 'F' | '|' | 'S', 1, 0)
                | ('|', 'L' | 'J' | '|' | 'S', -1, 0)
                | ('-', 'L' | 'F' | '-' | 'S', 0, 1)
                | ('-', '7' | 'J' | '-' | 'S', 0, -1)
                | ('L', '7' | 'F' | '|' | 'S', 1, 0)
                | ('L', '7' | 'J' | '-' | 'S', 0, -1)
                | ('J', '7' | 'F' | '|' | 'S', 1, 0)
                | ('J', 'L' | 'F' | '-' | 'S', 0, 1)
                | ('7', 'L' | 'J' | '|' | 'S', -1, 0)
                | ('7', 'L' | 'F' | '-' | 'S', 0, 1)
                | ('F', 'L' | 'J' | '|' | 'S', -1, 0)
                | ('F', '7' | 'J' | '-' | 'S', 0, -1) => {
                    let next_row = row as i32 + row_d;
                    let next_col = col as i32 + col_d;
                    if visited.insert((next_row as usize, next_col as usize)) {
                        stack.push((next_row as usize, next_col as usize));
                        polygon_loop.push((next_row, next_col));
                    }
                }
                _ => (),
            }
        }
    }
    let l = polygon_loop.len() as i32;
    let mut area = 0;
    polygon_loop.push((start.0 as i32, start.1 as i32));
    for ((x1, y1), (x2, y2)) in polygon_loop.iter().zip(polygon_loop.iter().skip(1)) {
        area += x1 * y2 - x2 * y1;
    }
    area.abs() / 2 - (l / 2 - 1)
}
fn part_one(input: &Vec<Vec<char>>, start: (usize, usize)) -> u32 {
    let mut stack = vec![(start.0, start.1)];
    let mut nodes: BTreeMap<(usize, usize), u32> = BTreeMap::new();
    nodes.insert((start.0, start.1), 0);
    while let Some((row, col)) = stack.pop() {
        let current_char = input[row][col];
        let current_dis = nodes[&(row, col)];
        // The rules of connectivity imply that some characters can be adjacent to some other characters at specific positions (for example, in '7L', '7' can not be visited from 'L').
        for (adj, row_d, col_d) in get_neighbour_nodes(row, col, &input) {
            match (adj, current_char, row_d, col_d) {
                ('|', '7' | 'F' | '|' | 'S', 1, 0)
                | ('|', 'L' | 'J' | '|' | 'S', -1, 0)
                | ('-', 'L' | 'F' | '-' | 'S', 0, 1)
                | ('-', '7' | 'J' | '-' | 'S', 0, -1)
                | ('L', '7' | 'F' | '|' | 'S', 1, 0)
                | ('L', '7' | 'J' | '-' | 'S', 0, -1)
                | ('J', '7' | 'F' | '|' | 'S', 1, 0)
                | ('J', 'L' | 'F' | '-' | 'S', 0, 1)
                | ('7', 'L' | 'J' | '|' | 'S', -1, 0)
                | ('7', 'L' | 'F' | '-' | 'S', 0, 1)
                | ('F', 'L' | 'J' | '|' | 'S', -1, 0)
                | ('F', '7' | 'J' | '-' | 'S', 0, -1) => {
                    let next_row = row as i32 + row_d;
                    let next_col = col as i32 + col_d;
                    let next_dis = current_dis + 1;
                    match nodes.entry((next_row as usize, next_col as usize)) {
                        Entry::Vacant(e) => {
                            e.insert(next_dis);
                            stack.push((next_row as usize, next_col as usize));
                        }
                        Entry::Occupied(mut e) => {
                            if *e.get_mut() > next_dis {
                                e.insert(next_dis);
                                stack.push((next_row as usize, next_col as usize));
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }
    *nodes.values().max().unwrap()
}

fn get_neighbour_nodes(
    row: usize,
    col: usize,
    input: &Vec<Vec<char>>,
) -> impl Iterator<Item = (char, i32, i32)> + '_ {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .into_iter()
        .filter(move |(row_diff, col_diff)| {
            row as i32 + row_diff >= 0
                && row as i32 + row_diff < input.len() as i32
                && col as i32 + col_diff >= 0
                && col as i32 + col_diff < input[0].len() as i32
        })
        .map(move |(row_diff, col_diff)| {
            let next_row = (row as i32 + row_diff) as usize;
            let next_col = (col as i32 + col_diff) as usize;
            let adj_char = input[next_row][next_col];
            (adj_char, row_diff, col_diff)
        })
}
