mod read_file;
use read_file::read_input;
// This is the main function that does the following:
// 1. gets the input as vector of strings.
// 2. creates a vector of numbers
// 3. iterates through the vector and stores only digit numbers in a new vector
// 4. store the final number in the created vector then prints the sum.
fn main() {
    let input = read_input().unwrap();
    let mut nums: Vec<i32> = Vec::new();
    for line in input{
        let mut number : Vec<i32> = Vec::new();
        for c in line.chars() {
            if c.is_digit(10) {
                number.push(c.to_digit(10).unwrap() as i32);
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

