use std::{fs::File, io::{BufRead, BufReader}};
use std::collections::HashMap;

pub(crate) fn read_input() -> (String, HashMap<String, (String, String)>) {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut first_line = "".to_string();
    let mut map : HashMap<String, (String, String)> = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        if !line.contains("=") {
            first_line = line;
            continue;
        }
        let nodes = line.split(" = ").collect::<Vec<&str>>();
        let target_nodes = nodes[1][1..nodes[1].len()-1].split(", ").collect::<Vec<&str>>();
        let target_node1 = target_nodes[0];
        let target_node2 = target_nodes[1];
        let source_node = nodes[0];
        map.insert(source_node.to_string(), (target_node1.to_string(), target_node2.to_string()));
    }
    (first_line, map)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_input() {
        println!("{}", read_input().0);
        println!("{:?}", read_input().1);
    }
}