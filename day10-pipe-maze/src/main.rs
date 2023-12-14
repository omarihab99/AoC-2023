use std::{collections::{BTreeMap, btree_map::Entry}};
mod helper;

fn main() {
    let (input, start) = helper::read_input();
    let mut stack  = vec![(start.0, start.1)];
    let mut nodes: BTreeMap<(usize, usize), u32> = BTreeMap::new();
    nodes.insert((start.0, start.1), 0);
    while let Some((row, col)) = stack.pop() {
        let current_char = input[row][col];
        let current_dis = nodes[&(row, col)];
        let m = [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .into_iter()
            .filter( |(row_diff, col_diff )| {
                row as i32 + row_diff >= 0
                    && row as i32 + row_diff < input.len() as i32
                    && col as i32 + col_diff >= 0
                    && col as i32 + col_diff < input[0].len() as i32
            })
            .map( |(row_diff, col_diff)| {
                let next_row =( row as i32 + row_diff) as usize;
                let next_col = (col as i32 + col_diff) as usize;
                let adj_char = input[next_row][next_col];
                (adj_char, row_diff, col_diff)
            });

        // The rules of connectivity imply that some characters can be adjacent to some other characters at specific positions (for example, in '7L', '7' can not be visited from 'L').

        for (adj, row_d, col_d) in m {
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
                },
                _ => (),
            }
        }
    }
    println!("{}", nodes.values().max().unwrap());
}

