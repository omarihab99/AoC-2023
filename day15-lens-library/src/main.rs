mod helper;

use std::collections::HashMap;

fn main() {
    let input = helper::read_input();
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &[String]) -> u32 {
    input.iter().map(|s| hash(s)).sum()
}

fn hash(s: &str) -> u32 {
    s.chars()
        .map(|c| c as u32)
        .fold(0, |h, ascii| (h + ascii) * 17 % 256)
}

fn part_two(input: &[String]) -> u32 {
    let mut focusing_power: u32 = 0;
    let mut boxes: Vec<Vec<String>> = vec![vec![]; 256];
    let mut lens_map: HashMap<String, u32> = HashMap::new();
    for s in input {
        let tuple: (String, u32, char);
        if s.contains("=") {
            let current: Vec<&str> = s.split("=").collect();
            tuple = (
                current[0].to_string(),
                current[1].parse::<u32>().unwrap(),
                '=',
            );
            lens_map.insert(tuple.0.clone(), tuple.1);
        } else {
            let current: Vec<&str> = s.split("-").collect();
            tuple = (current[0].to_string(), 0, '-');
        }
        let box_id = hash(&tuple.0);
        match tuple.2 {
            '=' => {
                if !boxes[box_id as usize].contains(&tuple.0) {
                    boxes[box_id as usize].push(tuple.0);
                }
            }
            _ => {
                if let Some(idx) = boxes[box_id as usize].iter().position(|x| *x == tuple.0) {
                    boxes[box_id as usize].remove(idx);
                }
            }
        }
    }
    for b in &boxes {
        println!("{:?}", b);
    }
    for (i, b) in boxes.iter().enumerate() {
        for (j, lens) in b.iter().enumerate() {
            focusing_power += (i as u32 + 1) * (j as u32 + 1) * lens_map.get(lens).unwrap_or(&0);
        }
    }

    focusing_power
}
