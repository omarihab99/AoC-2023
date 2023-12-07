use std::{fs::File, io::{self, BufRead}};

pub fn read_input() -> io::Result<Vec<String>> {
    let file = File::open("src/input.txt")?;
    let reader = io::BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    Ok(lines)
}
#[cfg(test)]
mod tests {
    use crate::read_file::read_input;
    #[test]
    fn it_works() {
        read_input().expect("failed to read input");
    }
}