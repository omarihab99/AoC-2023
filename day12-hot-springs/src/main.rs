mod helper;
use std::collections::HashMap;
fn main() {
    let input = helper::read_input();
    let mut valid_strings = 0;
    for mut line in input {
        line.0.push('?');
        let mut string_part= line.0.repeat(5);
        string_part.pop();
        let number_part = line.1.repeat(5);
        let mut memo: HashMap<(usize, usize, usize), u64> = HashMap::new();
        valid_strings += backtrack(&mut string_part.clone(), &number_part, (0, 0, 0), &mut memo);
    }
    println!("Part 1: {}", valid_strings);
}
fn backtrack(
    springs: &mut Vec<char>,
    lengths: &Vec<usize>,
    state: (usize, usize, usize),
    memo: &mut HashMap<(usize, usize, usize), u64>,
) -> u64 {
    let (pos, grp, len) = state;
    let hash = (pos, grp, len);

    if let Some(&result) = memo.get(&hash) {
        return result;
    }

    if pos == springs.len() {
        if len != 0 && len != lengths[grp] {
            return 0;
        }
        return if grp + (grp < lengths.len() && len == lengths[grp]) as usize == lengths.len() {
            1
        } else {
            0
        };
    }
    let mut n = 0;
    let chr = springs[pos];
    if chr != '.' && grp < lengths.len() && len < lengths[grp] {
        n += backtrack(springs, lengths, (pos + 1, grp, len + 1), memo);
    }
    if chr != '#' && (len == 0 ||len == lengths[grp]) {
        n += backtrack(
            springs,
            lengths,
            (pos + 1, grp + (grp < lengths.len() && len == lengths[grp]) as usize , 0),
            memo,
        );
    }
    memo.insert(hash, n);
    n
}
