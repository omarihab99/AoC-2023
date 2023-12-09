use std::fs::File;
use std::io::{BufReader, self, BufRead};

pub(crate) fn read_input() -> io::Result<Vec<String>> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);
    let mut result = Vec::new();
    for line in reader.lines() {
        result.push(line?);
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    use crate::helper::read_input;

    #[test]
    fn it_works() {
        let result = read_input().unwrap();
        println!("{:?}", result);
    }
}