use std::{
    fs::File,
    io::{BufRead, BufReader},
};
pub(crate) fn read_input() -> Vec<Vec<char>> {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        let res = read_input();
        for (i, row) in res.iter().enumerate() {
            println!("Row #{}: {:?}", i, row);
        }
    }
}
