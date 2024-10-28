use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::utils::Module;
#[allow(unused_variables)]
pub(crate) fn read_input() -> (HashMap<String, Module>, HashMap<String, Vec<String>>) {
    let mut modules = HashMap::new();
    let mut receivers: HashMap<String, Vec<String>> = HashMap::new();
    let file = File::open("src/input.txt");
    if let Ok(file) = file {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let line = line.split(" -> ").collect::<Vec<&str>>();
            let left = line[0].trim();
            let right = line[1].trim();
            let right = right.split(", ").collect::<Vec<&str>>();
            if left.starts_with("%") {
                modules.insert(left[1..].to_string(), Module::FLIPFLOP);
                let left = left[1..].to_string();
            } else if left.starts_with("&") {
                modules.insert(left[1..].to_string(), Module::CONJUNCTION);
                let left = left[1..].to_string();
            } else {
                modules.insert(left.to_string(), Module::BROADCASTER);
                let left = left.to_string();
            }
            receivers.insert(
                left.to_string(),
                right.iter().map(|x| x.to_string()).collect(),
            );
        }
    }
    (modules, receivers)
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_read_input() {
        let (modules, receivers) = super::read_input();
        for (key, value) in modules {
            println!("{:?} {:?}", key, value);
        }
        for (key, value) in receivers {
            println!("{:?} {:?}", key, value);
        }        
    }
}
