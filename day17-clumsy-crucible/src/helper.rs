use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use pathfinding::matrix::Matrix;
pub(crate) fn read_input() -> Matrix<i32> {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Matrix<i32>>()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_input() {
        let input = read_input();
        for row in input {
            println!("{:?}", row);
        }
    }
}
