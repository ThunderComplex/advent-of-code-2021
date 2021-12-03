use crate::helper::input_helper::*;

/// yikes! This code is a mess!
/// Seriously! This is scuffed!!!
/// It was a headache-and-a-half to even get this code working (talk about 'fighting the borrow-checker')
/// I'm not gonna optimize it, but if you want to have fun give it a shot!
pub fn day3() {
    let lines: Vec<String> = read_input_lines(3);
    let bits = lines[0].len();
    let mut most_common_bit_list: Vec<u8> = vec![];
    let mut least_common_bit_list: Vec<u8> = vec![];

    for bit in 0..bits {
        let mut column_bit_list: Vec<char> = vec![];
        lines
            .iter()
            .for_each(|line| column_bit_list.push(line.chars().nth(bit).expect("Line is broken")));

        let zero_bit_count = column_bit_list.iter().filter(|x| **x == '0').count();
        let one_bit_count = column_bit_list.iter().filter(|x| **x == '1').count();

        if zero_bit_count > one_bit_count {
            most_common_bit_list.push(0);
            least_common_bit_list.push(1);
        } else {
            most_common_bit_list.push(1);
            least_common_bit_list.push(0);
        }
    }

    let gamma_rate = usize::from_str_radix(
        most_common_bit_list
            .into_iter()
            .map(|bit| bit.to_string())
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap();

    let epsilon_rate = usize::from_str_radix(
        least_common_bit_list
            .into_iter()
            .map(|bit| bit.to_string())
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap();

    println!(
        "γ-rate: {}, ε-rate: {}, power consumption: {}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );

    let mut oxygen_filtered = lines.clone();
    let mut co2_filtered = lines;

    let mut continue_oxygen_filtering = true;
    let mut continue_co2_filtering = true;

    for bit in 0..bits {
        let zero_bit_count_oxy = oxygen_filtered
            .iter()
            .map(|line| line.chars().nth(bit).expect("Line is broken"))
            .filter(|x| *x == '0')
            .count();
        let one_bit_count_oxy = oxygen_filtered
            .iter()
            .map(|line| line.chars().nth(bit).expect("Line is broken"))
            .filter(|x| *x == '1')
            .count();

        let zero_bit_count_co2 = co2_filtered
            .iter()
            .map(|line| line.chars().nth(bit).expect("Line is broken"))
            .filter(|x| *x == '0')
            .count();
        let one_bit_count_co2 = co2_filtered
            .iter()
            .map(|line| line.chars().nth(bit).expect("Line is broken"))
            .filter(|x| *x == '1')
            .count();
        let mut most_common_bit = '0';
        let mut least_common_bit = '0';

        if one_bit_count_oxy >= zero_bit_count_oxy {
            most_common_bit = '1';
        }

        if one_bit_count_co2 < zero_bit_count_co2 {
            least_common_bit = '1';
        }

        if continue_oxygen_filtering {
            oxygen_filtered = oxygen_filtered
                .iter_mut()
                .filter(|line| {
                    line.chars()
                        .nth(bit)
                        .expect("Line is broken")
                        .eq(&most_common_bit)
                })
                .map(|e| String::from(&*e))
                .collect();
        }

        if continue_co2_filtering {
            co2_filtered = co2_filtered
                .iter_mut()
                .filter(|line| {
                    line.chars()
                        .nth(bit)
                        .expect("Line is broken")
                        .eq(&least_common_bit)
                })
                .map(|e| String::from(&*e))
                .collect();
        }

        continue_oxygen_filtering = oxygen_filtered.len() > 1;
        continue_co2_filtering = co2_filtered.len() > 1;
    }

    let oxygen_generator_rating = usize::from_str_radix(&oxygen_filtered[0], 2).unwrap();
    let co2_scrubber_rating = usize::from_str_radix(&co2_filtered[0], 2).unwrap();
    let life_support_rating = oxygen_generator_rating * co2_scrubber_rating;

    println!(
        "Oxygen generator rating: {}, CO2 scrubber rating: {}, life support rating: {}",
        oxygen_generator_rating, co2_scrubber_rating, life_support_rating
    );
}
