use std::collections::HashMap;

mod helper;

fn main() {
    let result = helper::read_input();
    let seed_numbers = result.0;
    let maps = result.1;
    let seed_to_soil_map = maps[0].clone();
    let soil_to_fertilizer_map = maps[1].clone();
    let fertilizer_to_water_map = maps[2].clone();
    let water_to_light_map = maps[3].clone();
    let light_to_temp_map = maps[4].clone();
    let temp_to_humidity_map = maps[5].clone();
    let humidity_to_location_map = maps[6].clone();
    let mut res = u128::MAX;
    for seed_number in seed_numbers {
        let soil_number = find_number(seed_to_soil_map.clone(), seed_number);
        let  fertilizer_number = find_number(soil_to_fertilizer_map.clone(), soil_number);
        let  water_number = find_number(fertilizer_to_water_map.clone(), fertilizer_number);
        let  light_number = find_number(water_to_light_map.clone(), water_number);
        let  temp_number = find_number(light_to_temp_map.clone(), light_number);
        let  humidity_number = find_number(temp_to_humidity_map.clone(), temp_number);
        let  location_number = find_number(humidity_to_location_map.clone(), humidity_number);
        res = res.min(location_number);
    }
    println!("Part one solution: {}", res);
}
fn find_number(map : HashMap<u128, Vec<u128>>, prev_number : u128) -> u128 {
    let mut mapping_number = prev_number;
    for (key, value) in map {
        let source_range_start = value[0];
        let range_length = value[1];
        let source_range_end = source_range_start + range_length;
        if prev_number >= source_range_start && prev_number <= source_range_end {
            mapping_number = key + (prev_number - source_range_start);
            break;
        }
    }
    mapping_number
}