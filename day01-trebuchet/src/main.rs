mod read_file;

use std::collections::HashMap;
use read_file::read_input;
// This is the main function that does the following:
// 1. gets the input as vector of strings.
// 2. creates a vector of numbers
// 3. iterates through the vector and stores only digit numbers in a new vector
// 4. store the final number in the created vector then prints the sum.
fn main() {
    let mut numbers_map : HashMap<String, i32> = HashMap::new();
    numbers_map.insert("one".to_string(), 1);
    numbers_map.insert("two".to_string(), 2);
    numbers_map.insert("three".to_string(), 3);
    numbers_map.insert("four".to_string(), 4);
    numbers_map.insert("five".to_string(), 5);
    numbers_map.insert("six".to_string(), 6);
    numbers_map.insert("seven".to_string(), 7);
    numbers_map.insert("eight".to_string(), 8);
    numbers_map.insert("nine".to_string(), 9);
    let input = read_input().unwrap();
    let mut nums: Vec<i32> = Vec::new();
    for line in input{
        let mut number : Vec<i32> = Vec::new();
        for (mut i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                number.push(c.to_digit(10).unwrap() as i32);
            }
            else {
                let mut s = String::from("");
                while i < line.len() {
                    if line.chars().nth(i).unwrap().is_alphabetic(){
                        s+= &line.chars().nth(i).unwrap().to_string();
                    }
                    else {
                        break;
                    }
                    if numbers_map.contains_key(&s){
                        number.push(numbers_map.get(s.as_str()).unwrap().clone());
                        break;
                    }
                    i += 1;
                }
            }
        }
        if number.len() == 0 {
            nums.push(0);
        }
        else {
            nums.push((number[0] * 10) + number.last().unwrap());
        }

    }
    println!("{}", nums.iter().sum::<i32>());
}

