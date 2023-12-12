mod helper;

fn main() {
    let input = helper::read_input();
    let mut sum: i64 = 0;
    let part_two = true;
    for line in input {
        let mut res: Vec<Vec<i64>> = Vec::new();
        res.push(line.clone());
        get_diff(&line, &mut res);
        sum += if part_two {
            get_number_two(&res)
        } else {
            get_number_one(&res)
        };
        println!("{}", sum);
    }
    println!("solution: {}", sum);
}
fn get_diff(line: &Vec<i64>, res: &mut Vec<Vec<i64>>) {
    res.push(
        line.iter()
            .zip(line.iter().skip(1))
            .map(|(x, y)| y - x)
            .collect::<Vec<i64>>(),
    );
    let mut not_zero = false;
    for num in res.last().unwrap() {
        if num != &0 {
            not_zero = true;
            break;
        }
    }
    if not_zero {
        let r = res.clone();
        get_diff(r.last().unwrap(), res);
    }
}

fn get_number_one(res: &Vec<Vec<i64>>) -> i64 {
    let c = 0;
    res.iter().rev().fold(c, |acc, x| acc + x.last().unwrap())
}
fn get_number_two(res: &Vec<Vec<i64>>) -> i64 {
    let c = 0;
    res.iter().rev().fold(c, |acc, x| x.first().unwrap() - acc)
}
