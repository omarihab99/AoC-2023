// Time:        57     72     69     92
// Distance:   291   1172   1176   2026
fn main() {
    let time : Vec<u32> = vec![57, 72, 69, 92];
    let mut res : u32 = 1;
    let distance : Vec<u32> = vec![291, 1172, 1176, 2026];
    for i in 0..time.len(){
        let t = time[i];
        let d = distance[i];
        let mut sum = 0;
        for b in 0..t+1{
            let curr_d = b * (t-b);
            if curr_d > d{
                sum += 1;
            }
        }
        res *= sum;
    }
    println!("{}", res);
}
