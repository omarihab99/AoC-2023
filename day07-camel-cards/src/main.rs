mod helper;

fn main() {
    let type_vec = helper::parse_input(false); // Change the value to true if you want part one.
    let mut sum: u32 = 0;
    for v in type_vec {
        for hand in v {
            println!("{}", hand.cards);
            println!("Hand bit: {}", hand.bit);
            println!("Hand rank: {}", hand.rank.unwrap());
            sum += hand.bit * hand.rank.unwrap();
        }
    }
    println!("Solution: {}", sum);
}
