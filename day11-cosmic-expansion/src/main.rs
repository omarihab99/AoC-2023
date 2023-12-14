use std::cmp::{max,min};

mod helper;

fn main() {
    let input = helper::read_input();
    let mut indices: Vec<(usize, usize)> = Vec::new();
    let mut sum = 0;
    let part_two = true; // change to false to get part 1
    input.iter().enumerate().for_each(|(i, v)| {
        v.iter().enumerate().for_each(|(j, c)| {
            if *c == '#' {
                indices.push((i, j));
            }
        })
    });
    let (expanded_rows_indices, expanded_cols_indices) = expand(&input);
    println!("Rows: {:?}", expanded_rows_indices);
    println!("Cols: {:?}", expanded_cols_indices);
    for (i, v) in indices.iter().enumerate() {
        for j in i+1..indices.len() {
            let (x, y) = indices[j];
            sum += (x as i64 - v.0 as i64).abs()
                + (y as i64 - v.1 as i64).abs()
                + expanded_rows_indices.iter().filter(|k| min(v.0,x) <= **k && **k <= max(v.0,x)).count() as i64 * if part_two { 999999 } else { 1 }
                + expanded_cols_indices.iter().filter(|k| min(v.1,y) <= **k && **k <= max(v.1,y)).count() as i64 * if part_two { 999999 } else { 1 };
        }
    }
    println!("Part one: {}", sum);
}
// No need to add new rows or columns to the input due to the expansion and this will be obvious when solving part two in which the expansion is 1 million times larger.
// Alternatively, we could just add the indices of new rows and columns to count them in part two.
fn expand(input: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut expanded_rows_indices: Vec<usize> = Vec::new();
    let mut expanded_cols_indices: Vec<usize> = Vec::new();
    input.iter().enumerate().for_each(|(i, v)| {
        if !v.contains(&'#') {
            expanded_rows_indices.push(i);
        }
    });
    for i in 0..input[0].len() {
        let found = input.iter().any(|row| row[i] == '#');
        if !found {
            expanded_cols_indices.push(i);
        }
    }
    (expanded_rows_indices, expanded_cols_indices)
}
