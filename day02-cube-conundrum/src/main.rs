use std::collections::HashMap;

mod helper;
// This is the main function that takes each game with its sets of various colors
// and checks if the game is possible which occurs when all colors' number in each set is less
// the original number of that color.
fn main() {
    let games = helper::get_games();
    let mut config: HashMap<String, u8> = HashMap::new();
    let mut sum: u32 = 0;
    let mut power_sum: u32 = 0;
    config.insert("red".to_string(),12);
    config.insert("green".to_string(),13);
    config.insert("blue".to_string(),14);
    for (id, sets) in games {
        let mut possible = true;
        let mut min_green : u8 = 0;
        let mut min_blue : u8 = 0;
        let mut min_red : u8 = 0;
        for set in sets {
            let parts = set.split(", ").collect::<Vec<&str>>();
            for part in parts {
                let part = part.split_whitespace().collect::<Vec<&str>>();
                let count = part[0].parse::<u8>().unwrap();
                let color = part[1];
                if config.contains_key(color) && config[color] < count {
                    possible = false;
                }
                if color == "green"{
                    min_green = std::cmp::max(min_green, count);
                }
                if color == "blue"{
                    min_blue = std::cmp::max(min_blue, count);
                }
                if color == "red"{
                    min_red = std::cmp::max(min_red, count);
                }
            }
        }
        if possible {
            sum += id as u32;
        }
        power_sum += min_green as u32 * min_blue as u32 * min_red as u32;
    }
    println!("{}", sum);
    println!("{}", power_sum);
}
