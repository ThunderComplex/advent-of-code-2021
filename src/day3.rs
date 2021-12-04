use crate::helper::input_helper::*;

pub fn day3() {
    let lines: Vec<String> = read_input_lines(3);
    let bits = lines[0].len();
    let mut gamma_rate = 2usize.pow((bits).try_into().unwrap()) - 1;
    let mut epsilon_rate = gamma_rate.clone();

    let mut continue_oxygen_filtering = true;
    let mut continue_co2_filtering = true;

    let mut oxygen_filtered = lines.clone();
    let mut co2_filtered = lines.clone();

    for bit in 0..bits {
        let mut zero_count = 0;
        let mut one_count = 0;
        let mut zero_count_oxy = 0;
        let mut one_count_oxy = 0;
        let mut zero_count_co2 = 0;
        let mut one_count_co2 = 0;
        let bitflag: usize = 0b1 << (bits - bit - 1);

        lines.iter().for_each(|line| {
            let lineint: usize = usize::from_str_radix(line, 2).unwrap();

            if lineint & bitflag == 0 {
                zero_count += 1;
            } else {
                one_count += 1;
            }
        });

        oxygen_filtered.iter().for_each(|line| {
            let lineint: usize = usize::from_str_radix(line, 2).unwrap();

            if lineint & bitflag == 0 {
                zero_count_oxy += 1;
            } else {
                one_count_oxy += 1;
            }
        });

        co2_filtered.iter().for_each(|line| {
            let lineint: usize = usize::from_str_radix(line, 2).unwrap();

            if lineint & bitflag == 0 {
                zero_count_co2 += 1;
            } else {
                one_count_co2 += 1;
            }
        });

        if zero_count > one_count {
            gamma_rate ^= bitflag;
        } else {
            epsilon_rate ^= bitflag;
        }

        if zero_count_oxy > one_count_oxy {
            if continue_oxygen_filtering {
                oxygen_filtered = oxygen_filtered
                    .iter()
                    .filter(|line| usize::from_str_radix(line, 2).unwrap() & bitflag == 0)
                    .map(String::from)
                    .collect();
            }
        } else if continue_oxygen_filtering {
            oxygen_filtered = oxygen_filtered
                .iter()
                .filter(|line| usize::from_str_radix(line, 2).unwrap() & bitflag != 0)
                .map(String::from)
                .collect();
        }

        if zero_count_co2 > one_count_co2 {
            if continue_co2_filtering {
                co2_filtered = co2_filtered
                    .iter()
                    .filter(|line| usize::from_str_radix(line, 2).unwrap() & bitflag != 0)
                    .map(String::from)
                    .collect();
            }
        } else if continue_co2_filtering {
            co2_filtered = co2_filtered
                .iter()
                .filter(|line| usize::from_str_radix(line, 2).unwrap() & bitflag == 0)
                .map(String::from)
                .collect();
        }

        continue_oxygen_filtering = oxygen_filtered.len() > 1;
        continue_co2_filtering = co2_filtered.len() > 1;
    }

    println!(
        "γ-rate: {}, ε-rate: {}, power consumption: {}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );

    let oxygen_generator_rating = usize::from_str_radix(&oxygen_filtered[0], 2).unwrap();
    let co2_scrubber_rating = usize::from_str_radix(&co2_filtered[0], 2).unwrap();
    let life_support_rating = oxygen_generator_rating * co2_scrubber_rating;

    println!(
        "Oxygen generator rating: {}, CO2 scrubber rating: {}, life support rating: {}",
        oxygen_generator_rating, co2_scrubber_rating, life_support_rating
    );
}
