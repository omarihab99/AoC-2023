mod helper;

fn main() {
    let (directions, map) = helper::read_input();
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
