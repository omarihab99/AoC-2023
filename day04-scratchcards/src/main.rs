use std::collections::HashSet;

mod helper;

fn main() {
    let input: Vec<String> = helper::parse();
    let n: usize = input.len();
    let mut list = vec![1; n];
    let mut sum: u32 = 0;
    for (i, card) in input.iter().enumerate() {
        let all_numbers = card.split(" | ").collect::<Vec<&str>>();
        let winning_numbers = all_numbers[0];
        let my_numbers = all_numbers[1];
        let mut winning_numbers_vec = winning_numbers.split(" ").collect::<HashSet<&str>>();
        let mut my_numbers_vec = my_numbers.split(" ").collect::<Vec<&str>>();
        my_numbers_vec.retain(|&x| x != "");
        winning_numbers_vec.retain(|&x| x != "");
        my_numbers_vec.retain(|&x| winning_numbers_vec.contains(x));
        part_one(&mut sum, &my_numbers_vec);
        part_two(&my_numbers_vec, &mut list, i, n);
    }
    println!("Part one solution: {}", sum);
    println!("Part two solution: {}", list.iter().sum::<u32>());
}
fn part_one(sum: &mut u32, my_numbers_vec: &Vec<&str>) {
    if my_numbers_vec.len() != 0 {
        println!("{}", my_numbers_vec.len());
        *sum += 2_u32.pow(my_numbers_vec.len() as u32 - 1);
    }
}
fn part_two(my_numbers_vec: &Vec<&str>, list: &mut Vec<u32>, i: usize, n: usize) {
    if my_numbers_vec.len() != 0 {
        let k = my_numbers_vec.len();
        for j in i + 1..i + k + 1 {
            if j<n {
                list[j]+=list[i];
            }
        }
    }
}
