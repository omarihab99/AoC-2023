mod helper;

fn main() {
    let blocks = helper::read_input();
    let mut sum = 0;
    let part_one = false;
    let diff = if part_one { 0 } else { 1 };
    for mut block in blocks {
        if let Some(res) = reflection(&mut block, diff) {
            sum += res * 100;
        } else {
            let mut transposed = (0..block[0].len())
                .map(|i| {
                    block
                        .iter()
                        .map(|row| row.chars().nth(i).unwrap())
                        .collect()
                })
                .collect();
            if let Some(res) = reflection(&mut transposed, diff) {
                sum += res;
            }
        }
    }
    println!("Sum: {}", sum);
}
fn reflection(block: &mut Vec<String>, diff: i32) -> Option<i32> {
    for idx in 1..block.len() {
        let mut sum = 0;
        for (left, right) in block[..idx].iter().rev().zip(block[idx..].iter()) {
            sum += left
                .chars()
                .zip(right.chars())
                .filter(|(l, r)| l != r)
                .count();
        }
        if sum as i32 == diff {
            return Some(idx as i32);
        }
    }
    None
}
