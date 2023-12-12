use std::{
    fs::File,
    io::{BufRead, BufReader},
};
pub(crate) fn read_input() -> Vec<Vec<i64>> {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut res: Vec<Vec<i64>> = Vec::new();
    for line in reader.lines() {
        res.push(
            line.unwrap()
                .split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
        );
    }
    res
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        println!("{:?}", read_input());
    }
}