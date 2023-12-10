use std::collections::HashMap;
mod helper;

fn main() {
    let result = helper::read_input();
    let seed_numbers = result.0;
    let maps = result.1;
    let seed_to_soil_map = &maps[0];
    let soil_to_fertilizer_map = &maps[1];
    let fertilizer_to_water_map = &maps[2];
    let water_to_light_map = &maps[3];
    let light_to_temp_map = &maps[4];
    let temp_to_humidity_map = &maps[5];
    let humidity_to_location_map = &maps[6];
    let mut res1 = u128::MAX;
    let mut res2 = u128::MAX;
    for &seed_number in &seed_numbers {
        let location_number = find_number(
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temp_map,
            temp_to_humidity_map,
            humidity_to_location_map,
            seed_number,
        );
        res1 = res1.min(location_number);
    }
    println!("Part one solution: {}", res1);
    // res2 = res1;
    for i in (0..seed_numbers.len()).step_by(2) {
        let seed_number = seed_numbers[i];
        let range = seed_numbers[i + 1];
        if range > 0 {
            for s_sum in seed_number..seed_number + range {
                let location_number = find_number(
                    seed_to_soil_map,
                    soil_to_fertilizer_map,
                    fertilizer_to_water_map,
                    water_to_light_map,
                    light_to_temp_map,
                    temp_to_humidity_map,
                    humidity_to_location_map,
                    s_sum,
                );
                res2 = res2.min(location_number);
            }
        }
    }

    println!("Part two solution: {}", res2);
}
fn find_number(
    seed_to_soil_map: &HashMap<u128, Vec<u128>>,
    soil_to_fertilizer_map: &HashMap<u128, Vec<u128>>,
    fertilizer_to_water_map: &HashMap<u128, Vec<u128>>,
    water_to_light_map: &HashMap<u128, Vec<u128>>,
    light_to_temp_map: &HashMap<u128, Vec<u128>>,
    temp_to_humidity_map: &HashMap<u128, Vec<u128>>,
    humidity_to_location_map: &HashMap<u128, Vec<u128>>,
    prev_number: u128,
) -> u128 {
    let mut mapping_number = prev_number;
    fn get_mapping_number(map: &HashMap<u128, Vec<u128>>, prev_number: u128) -> u128 {
        if let Some(value) = map.iter().find_map(|(key, value)| {
            let source_range_start = value[0];
            let range_length = value[1];
            let source_range_end = source_range_start + range_length;
            if prev_number >= source_range_start && prev_number <= source_range_end {
                Some(key + (prev_number - source_range_start))
            } else {
                None
            }
        }) {
            value
        } else {
            prev_number
        }
    }
    mapping_number = get_mapping_number(seed_to_soil_map, prev_number);
    mapping_number = get_mapping_number(soil_to_fertilizer_map, mapping_number);
    mapping_number = get_mapping_number(fertilizer_to_water_map, mapping_number);
    mapping_number = get_mapping_number(water_to_light_map, mapping_number);
    mapping_number = get_mapping_number(light_to_temp_map, mapping_number);
    mapping_number = get_mapping_number(temp_to_humidity_map, mapping_number);
    get_mapping_number(humidity_to_location_map, mapping_number)
}
