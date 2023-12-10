use std::{
    fs::File,
    io::{BufRead, BufReader},
    collections::HashMap
};

pub(crate) fn read_input() -> (Vec<u128>, Vec<HashMap<u128, Vec<u128>>>) {
    let file = File::open("src/input.txt");
    let reader = BufReader::new(file.unwrap());
    let mut seed_numbers = Vec::new();
    let mut result= Vec::new();
    let mut map : HashMap<u128, Vec<u128>> = HashMap::new();
    for line in reader.lines() {
        if line.as_ref().unwrap().is_empty() {
            continue;
        }
        else if line.as_ref().unwrap().starts_with("seeds: ") {
            let seeds = line.as_ref()
                .unwrap()
                .split(": ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            seed_numbers = seeds[1]
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.to_string().parse::<u128>().unwrap())
                .collect::<Vec<u128>>();
        }
        else if line.as_ref().unwrap().ends_with("map:") {
            if !map.is_empty() {
                result.push(map.clone());
            }
            map = HashMap::new();
        }
        else {
            let numbers = line.unwrap().split_whitespace().collect::<Vec<&str>>().iter().map(|x| x.to_string().parse::<u128>().unwrap()).collect::<Vec<u128>>();
            let source_range_start = numbers[0];
            let dest_range_start = numbers[1];
            let range_length = numbers[2];
            map.insert(source_range_start, vec![dest_range_start, range_length]);
        }
    }
    result.push(map);
    (seed_numbers, result)
}


#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn it_works() {
        let result = read_input();
        println!("{:?}", result.0);
        println!("{:?}", result.1);
    }

}