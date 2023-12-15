mod helper;
use regex::Regex;
fn main() {
    let input = helper::read_input();
    let mut valid_strings = 0;
    for line in input {
        let string_part = line.0;
        let number_part = line.1;
        let mut regex_pattern = String::from(r"^\.*");
        for number in &number_part {
            regex_pattern.push_str(&format!("#{{{}}}", number));
            regex_pattern.push_str(r"\.+");
        }
        regex_pattern.pop();
        regex_pattern.pop();
        regex_pattern.push_str(r".*$");
        let re = Regex::new(&regex_pattern).unwrap();
        backtrack(
            &mut string_part.clone(),
            &number_part,
            &mut valid_strings,
            0,
            &re,
        );
    }
    println!("Part 1: {}", valid_strings);
}
fn backtrack(
    springs: &mut Vec<char>,
    numbers: &Vec<i32>,
    count: &mut i32,
    index: usize,
    re: &Regex,
) {
    if index == springs.len() && !springs.contains(&'?') {
        if re.is_match(springs.iter().collect::<String>().as_str()) {
            *count += 1;
        }
        return;
    }
    let char = springs[index];
    if char == '?' {
        for replace in ['#', '.'].iter() {
            springs[index] = *replace;
            backtrack(springs, numbers, count, index + 1, re);
            springs[index] = '?';
        }
    } else {
        backtrack(springs, numbers, count, index + 1, re);
    }
}
