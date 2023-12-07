use std::collections::HashMap;

mod helper;
// This is the main function that takes each game with its sets of various colors
// and checks if the game is possible which occurs when all colors' number in each set is less
// the original number of that color.
fn main() {
    let games = helper::get_games();
    let mut sum: i32 = 0;
    let mut config: HashMap<String, u8> = HashMap::new();
    config.insert("red".to_string(),12);
    config.insert("green".to_string(),13);
    config.insert("blue".to_string(),14);
    for (id, sets) in games {
        let mut possible = true;
        for set in sets {
            let parts = set.split(", ").collect::<Vec<&str>>();
            for part in parts {
                let part = part.split_whitespace().collect::<Vec<&str>>();
                let count = part[0].parse::<u8>().unwrap();
                let color = part[1];
                if config.contains_key(color) && config[color] < count {
                    possible = false;
                    break;
                }
            }
        }
        if possible {
            sum += id as i32;
        }
    }
    println!("{}", sum);
}
