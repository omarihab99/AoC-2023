use std::{fs::File, io::{BufRead, BufReader}};
pub(crate) fn read_input() -> Vec<String> {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines().nth(0).unwrap().unwrap().split(",").map(|x| x.to_string()).collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_input() {
        let input = read_input();
        for i in input {
            println!("{}", i);
        }
    }
}