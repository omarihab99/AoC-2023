use std::{fs::File, io::{BufRead, BufReader}};

pub(crate) fn read_input() -> Vec<Vec<String>> {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut block : Vec<String> = Vec::new();
    let mut blocks : Vec<Vec<String>> = Vec::new();
    for line in reader.lines() {
        if line.as_ref().unwrap().is_empty(){
            blocks.push(block);
            block = Vec::new();
            continue;
        }
        block.push(line.unwrap());
    }
    blocks.push(block);
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_input() {
        let blocks = read_input();
        for block in blocks {
            println!("{:?}", block);
        }
    }
}