mod helper;

fn main() {
    let input = helper::read_input();
    let mut sum : i64 = 0;
    for line in input {
        let mut res : Vec<Vec<i64>> = Vec::new();
        res.push(line.clone());
        get_diff(&line, &mut res);
        sum+=get_number(&res);
    }
    println!("Part one solution: {}", sum);
}
fn get_diff(line: &Vec<i64>, res: &mut Vec<Vec<i64>>)  {
    res.push(line.iter().zip(line.iter().skip(1)).map(|(x, y)| y - x).collect::<Vec<i64>>());
    let mut not_zero = false;
    for num in res.last().unwrap(){
        if num != &0{
            not_zero = true;
            break;
        }
    }
    if not_zero {
        let r = res.clone();
        get_diff(r.last().unwrap(),  res);
    }
}

fn get_number(res: &Vec<Vec<i64>>) -> i64 {
    let c = 0;
    res.iter().fold(c, |acc, x| {
        acc + x.last().unwrap()
    })
}