use std::{fs::File, io::BufRead, io::BufReader};

pub(crate) fn read_input() -> Vec<(Vec<char>, Vec<usize>)> {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|l| {
        let l = l.unwrap();
        let line = l.split(" ").collect::<Vec<&str>>();
        (line[0].chars().collect(), line[1].split(",").map(|s| s.parse::<usize>().unwrap()).collect())
    }).collect()
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_read_input() {
        let input = read_input();
        for i in input {
            println!("{:?}", i);
        }
    }
}