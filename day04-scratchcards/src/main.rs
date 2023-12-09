use std::collections::HashSet;

mod helper;

fn main() {
    let input = helper::parse();
    let mut sum: u32 = 0;
    for card in input {
        let all_numbers = card.split(" | ").collect::<Vec<&str>>();
        let winning_numbers = all_numbers[0];
        let my_numbers = all_numbers[1];
        let mut winning_numbers_vec = winning_numbers.split(" ").collect::<HashSet<&str>>();
        let mut my_numbers_vec = my_numbers.split(" ").collect::<Vec<&str>>();
        my_numbers_vec.retain(|&x| x != "");
        winning_numbers_vec.retain(|&x| x != "");
        my_numbers_vec.retain(|&x| winning_numbers_vec.contains(x));
        part_one(&mut sum, &my_numbers_vec);

    }
    println!("Part one solution: {}", sum);
}
fn part_one(sum : &mut u32, my_numbers_vec : &Vec<&str>) {
    if my_numbers_vec.len() != 0 {
        *sum += 2_u32.pow(my_numbers_vec.len() as u32 - 1);
    }
}