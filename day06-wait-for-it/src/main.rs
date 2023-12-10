// Time:        57     72     69     92
// Distance:   291   1172   1176   2026
fn main() {
    let time: Vec<u32> = vec![57, 72, 69, 92];
    let mut res: u32 = 1;
    let distance: Vec<u32> = vec![291, 1172, 1176, 2026];
    for i in 0..time.len() {
        let t = time[i];
        let d = distance[i];
        let sum = calculate(t as u128, d as u128);
        res *= sum as u32;
    }
    // Part one
    println!("{}", res);
    println!("{}", calculate(57726992, 291117211762026));
}
fn calculate(time: u128, distance: u128) -> u128 {
    let mut sum: u128 = 0;
    for b in 0..time + 1 {
        let curr_d: u128 = b * (time - b);
        if curr_d > distance {
            sum += 1;
        }
    }
    sum
}
