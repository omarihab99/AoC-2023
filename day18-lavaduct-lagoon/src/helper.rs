use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) fn read_input() -> Vec<(char, i64,String)> {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            let l = line.unwrap();
            let line = l.split(" ").collect::<Vec<_>>();
            (
                line[0].chars().next().unwrap(),
                line[1].parse::<i64>().unwrap(),
                line[2].to_string(),
            )
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_input() {
        let input = read_input();
        for (i, j, color) in input {
            println!("First Pair: {:?}", (i, j, color));
            println!("---------------------");
        }
    }
}
