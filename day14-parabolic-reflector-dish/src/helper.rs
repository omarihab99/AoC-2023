use std::{fs::File, io::{BufRead, BufReader}};

pub(crate) fn read_input() -> Vec<Vec<char>>{
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap().chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_input() {
        let input = read_input();
        input.iter().for_each(|v| {println!("{:?}", v);});
    }
}