use std::{fs::File, io::{BufRead, BufReader}};

pub fn read_input() -> (Vec<Vec<char>>, (usize, usize)) {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut res : Vec<Vec<char>> = Vec::new();
    let mut starting_idx : (usize, usize) = (0, 0);
    for line in reader.lines() {
        let l : Vec<char> = line.unwrap().chars().collect::<Vec<char>>();
        if l.contains(&'S') {
            starting_idx = (res.len(), l.iter().position(|c| *c == 'S').unwrap());
        }
        res.push(l);
    }
    (res, starting_idx)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input(){
        let res = read_input();
        println!("{:?}", res);
    }
}