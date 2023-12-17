
mod helper;

fn main() {
    let mut input = helper::read_input();
    println!("{}", part_two(&mut input));
}

fn part_one(input: &mut Vec<Vec<char>>) -> i32{
    let mut res = 0;
    tilt_vertically(input,0);
    input.iter().enumerate().for_each(|(i, v)| {
        let index = input.len() - i;
        let count = v.iter().filter(|c| **c == 'O').count();
        res += index as i32 * count as i32;
    });
    res
}
fn part_two(input: &mut Vec<Vec<char>>) -> i32{
    let mut res = 0;
    let row_size = input.len();
    let output = process(input);
    for i in 0..row_size{
        let index = row_size - i;
        for j in 0..row_size{
            let c = output.chars().nth(i * row_size + j).unwrap();
            if c == 'O' {
                res += index as i32;

            }
        }
    }
    res
}
fn process(input: &mut Vec<Vec<char>>) -> String{
    let mut past: Vec<String> = Vec::new();
    let limit = 1000000000;
    let mut i=0;
    while i < limit {
        cycle(input);
        let state = input.iter().map(|v| v.iter().collect::<String>()).collect::<Vec<String>>().join("");
        if past.contains(&state) {
            if let Some(idx) = past.iter().position(|s| s == &state) {
                let cycle_size = i - idx;
                let diff = (limit - idx) % cycle_size + idx;
                return past[diff-1].clone();
            }
        }
        past.push(state);
        i+=1;
    }
    input.iter().map(|v| v.iter().collect::<String>()).collect::<Vec<String>>().join("")
}

fn cycle(input: &mut Vec<Vec<char>>){
    tilt_vertically(input,-1);
    tilt_horizontally(input,-1);
    tilt_vertically(input,1);
    tilt_horizontally(input,1);
}
fn tilt_vertically(input: &mut Vec<Vec<char>>, dir:i32)  {
    if dir > 0 {
        input.reverse();
    }
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 'O' {
                let mut k = i as i32 - 1;
                while k >= 0 {
                    let ch = input[k as usize][j];
                    if ch == 'O' || ch == '#' {
                        k += 1;
                        break;
                    }
                    k -= 1;
                }
                k = k.max(0);
                input[i][j] = '.';
                input[k as usize][j] = 'O';
            }
        }
    }
    if dir > 0 {
        input.reverse();
    }
}

fn tilt_horizontally(input: &mut Vec<Vec<char>>, dir:i32)  {
    for i in 0..input.len(){
        if dir > 0{
            input[i].reverse();
        }
        for j in 0..input[i].len(){
            if input[i][j] == 'O' {
                let mut k = j as i32 - 1;
                while k >= 0 {
                    let ch = input[i][k as usize];
                    if ch == 'O' || ch == '#' {
                        k += 1;
                        break;
                    }
                    k -= 1;
                }
                k = k.max(0);
                input[i][j] = '.';
                input[i][k as usize] = 'O';
            }
        }
        if dir > 0{
            input[i].reverse();
        }
    }
}

