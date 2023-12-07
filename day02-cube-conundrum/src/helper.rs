use std::{fs::File, io::{self, BufReader}};
use std::collections::HashMap;
use std::io::BufRead;

fn read_input() -> io::Result<Vec<String>> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    Ok(lines)
}
pub fn get_games() -> HashMap<u8, Vec<String>> {
    let mut games : HashMap<u8,Vec<String>> = HashMap::new();
    let lines = read_input().unwrap();
    for line in lines {
        let game = line.split(": ").collect::<Vec<&str>>();
        let id:u8 = game[0].split_whitespace().collect::<Vec<&str>>()[1].parse::<u8>().unwrap();
        let sets = game[1].split("; ").collect::<Vec<&str>>();
        games.insert(id, sets.iter().map(|s| s.to_string()).collect());
    }
    games
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_games() {
        let games = get_games();
        println!("{:?}", games);
    }
}