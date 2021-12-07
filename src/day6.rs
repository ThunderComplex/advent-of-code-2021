use std::collections::HashMap;

use crate::helper::input_helper::*;

pub fn day6() {
    println!("--- DAY 6 ---");
    let lines: Vec<String> = read_input_lines(6);
    let blowfish_list: Vec<u8> = lines
        .first()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut blowfish_map: HashMap<u8, usize> = HashMap::new();

    for blowfish in blowfish_list {
        *blowfish_map.entry(blowfish).or_insert(0) += 1;
    }

    for _day in 0..256 {
        let mother_count = *blowfish_map.entry(0).or_default();

        *blowfish_map.entry(0).or_default() = *blowfish_map.entry(1).or_default();
        *blowfish_map.entry(1).or_default() = *blowfish_map.entry(2).or_default();
        *blowfish_map.entry(2).or_default() = *blowfish_map.entry(3).or_default();
        *blowfish_map.entry(3).or_default() = *blowfish_map.entry(4).or_default();
        *blowfish_map.entry(4).or_default() = *blowfish_map.entry(5).or_default();
        *blowfish_map.entry(5).or_default() = *blowfish_map.entry(6).or_default();
        *blowfish_map.entry(6).or_default() = *blowfish_map.entry(7).or_default();

        blowfish_map
            .entry(6)
            .and_modify(|e| *e += mother_count)
            .or_default();

        *blowfish_map.entry(7).or_default() = *blowfish_map.entry(8).or_default();
        *blowfish_map.entry(8).or_default() = mother_count;
    }

    let blowfish_count: usize = blowfish_map.into_values().sum();

    println!("There are {} blowfishies after 256 days!", blowfish_count);
}
