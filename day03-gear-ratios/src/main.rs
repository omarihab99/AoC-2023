mod helper;

fn main() {
    let input = helper::read_input();
    if input.is_err() {
        eprintln!("Failed to read input");
    }
    let mut sum: u32 = 0;
    let input = input.unwrap();
    for (i, line) in input.iter().enumerate() {
        let mut number: u32 = 0;
        let mut valid = false;
        for (j, c) in line.chars().enumerate() {
            #[warn(unused_comparisons)]
            if c.is_digit(10){
                if i >= 1 {
                    let top = input[i - 1].chars().nth(j);
                    valid |= top.is_some() && !top.unwrap().is_digit(10) && top.unwrap() != '.';
                }
                if j >= 1 {
                    let left= input[i].chars().nth(j - 1);
                    valid |= left.is_some() && !left.unwrap().is_digit(10) && left.unwrap() != '.';
                }
                if i+1 < input.len() {
                    let bottom= input[i + 1].chars().nth(j);
                    valid |= bottom.is_some() && !bottom.unwrap().is_digit(10) && bottom.unwrap() != '.';
                }
                if j+1 < line.len() {
                    let right= input[i].chars().nth(j + 1);
                    valid |= right.is_some() && !right.unwrap().is_digit(10) && right.unwrap() != '.';
                }
                if i >= 1 && j >= 1 {
                    let left_up= input[i - 1].chars().nth(j - 1);
                    valid |= left_up.is_some() && !left_up.unwrap().is_digit(10) && left_up.unwrap() != '.';
                }
                if i >= 1 && j+1 < line.len() {
                    let left_down= input[i - 1].chars().nth(j + 1);
                    valid |= left_down.is_some() && !left_down.unwrap().is_digit(10) && left_down.unwrap() != '.';
                }
                if i+1 < input.len() && j >= 1 {
                    let right_up= input[i + 1].chars().nth(j - 1);
                    valid |= right_up.is_some() && !right_up.unwrap().is_digit(10) && right_up.unwrap() != '.';
                }
                if i+1 < input.len() && j+1 < line.len() {
                    let right_down= input[i + 1].chars().nth(j + 1);
                    valid |= right_down.is_some() && !right_down.unwrap().is_digit(10) && right_down.unwrap() != '.';
                }
                number = number * 10 + c.to_digit(10).unwrap();
            }
            else {
                if valid {
                    sum += number;
                }
                valid = false;
                number = 0;
            }
        }
        if valid {
            sum += number;
        }
    }
    println!("{}", sum);
}

