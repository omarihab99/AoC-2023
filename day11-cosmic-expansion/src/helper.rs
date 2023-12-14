use std::{fs::File, io::{BufRead, BufReader}};
pub(crate) fn read_input() -> Vec<Vec<char>> {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut res : Vec<Vec<char>> = Vec::new();
    for line in reader.lines(){
        let l = line.unwrap();
        res.push(l.chars().collect());
        if !l.contains('#') {
            res.push(l.chars().collect());
        }
    }
    let mut col: usize = 0;
    while col < res[0].len(){
        let mut found = false;
        for row in 0..res.len(){
            if res[row][col] == '#' {
                found = true;
                break;
            }
        }
        if !found {
            res.iter_mut().for_each(| v | {
                v.insert(col, '.');
            });
            col+=1;
        }
        col+=1
    }
    res
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input(){
        let res = read_input();
        for (i,row) in res.iter().enumerate() {
            println!("Row #{}: {:?}", i, row);
        }
    }
}