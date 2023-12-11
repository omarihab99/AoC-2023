mod helper;

fn main() {
    let type_vec = helper::parse_input();
    let mut sum: u32 = 0;
    for v in type_vec {
        for hand in v {
            println!("{}", hand.cards);
            println!("Hand bit: {}", hand.bit);
            println!("Hand rank: {}", hand.rank.unwrap());
            sum += hand.bit * hand.rank.unwrap();
        }
    }
    println!("Part one solution: {}", sum);
}
