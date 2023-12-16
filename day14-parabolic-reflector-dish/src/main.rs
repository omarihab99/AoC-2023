mod helper;

fn main() {
    let mut input = helper::read_input();
    let mut res = 0;
    for (i, line) in input.clone().iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'O' {
                let mut k = i as i32 -1;
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
    input.iter().enumerate().for_each(|(i, v)| {
        let index = input.len() - i;
        let count = v.iter().filter(|c| **c == 'O').count();
        res += index as i32 * count as i32;
        println!("{}: {}", index, count);
    });
    println!("{}", res);
}

