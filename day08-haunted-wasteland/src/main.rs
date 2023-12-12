use std::collections::HashMap;
use num::integer::lcm;
mod helper;

fn main() {
    let (directions, map) = helper::read_input();
    part_one(&directions, &map);
    part_two(&directions, &map);
}
fn part_one(directions: &str, map: &HashMap<String, (String, String)>) {
    let mut count: u32 = 0;
    let mut i = 0;
    let mut curr = "AAA".to_string();
    let dir_len = directions.len();
    loop {
        count += 1;
        let next = map.get(&curr).unwrap();
        curr = if directions.chars().nth(i % dir_len).unwrap() == 'R' {
            next.1.to_string()
        } else {
            next.0.to_string()
        };
        if curr == "ZZZ" {
            break;
        }
        i += 1;
    }
    println!("{}", count);
}
fn part_two(directions: &str, map: &HashMap<String, (String, String)>) {
    let mut steps : Vec<u32> = Vec::new();
    let mut starting = map.keys().filter(|k| k.ends_with("A")).collect::<Vec<&String>>();
    let dir_len = directions.len();
    for k in starting {
        let mut curr = k.to_string();
        let mut i = 0;
        let mut count: u32 = 0;
        loop {
            count += 1;
            let next = map.get(&curr).unwrap();
            curr = if directions.chars().nth(i % dir_len).unwrap() == 'R' {
                next.1.to_string()
            } else {
                next.0.to_string()
            };
            if curr .ends_with("Z") {
                break;
            }
            i += 1;
        }
        steps.push(count);
    }
    let mut prev : u64 = steps[0] as u64;
    for i in 1..steps.len() {
        prev = lcm(prev, steps[i] as u64);
    }
    println!("Part two solution: {}", prev);
}
