mod helper;

fn main() {
    let input = helper::read_input();
    let mut sum : u32 = 0;
    for s in input{
        let mut current_value : u32 = 0;
        for c in s.chars(){
            let ascii = c as u32;
            current_value+=ascii;
            current_value*=17;
            current_value%=256;
        }
        sum+=current_value;
    }
    println!("{}",sum);
}
