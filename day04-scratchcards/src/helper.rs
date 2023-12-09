use std::fs;
use std::io::{BufReader, self, BufRead};

fn read_input() -> io::Result<Vec<String>> {
    let file = fs::File::open("src/input.txt")?;
    let reader = BufReader::new(file);
    let mut result = Vec::new();
    for line in reader.lines() {
        result.push(line?);
    }
    Ok(result)
}
pub(crate) fn parse() -> Vec<String> {
    let input = read_input().unwrap();
    let mut output:Vec<String> = Vec::new();
    for line in input {
        let card = line.split(": ").collect::<Vec<&str>>();
        let card_numbers = card[1].to_string();
        output.push(card_numbers);
    }
    output
}

#[cfg(test)]
mod tests {
    use crate::helper::{parse};

    #[test]
    fn it_works() {
        let result = parse();
        println!("{:?}", result);
    }
}