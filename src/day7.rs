use crate::helper::input_helper::*;

pub fn day7() {
    println!("--- DAY 7 ---");
    let lines: Vec<String> = read_input_lines(7);
    let mut crab_positions: Vec<usize> = lines
        .first()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    crab_positions.sort_unstable();

    let median;

    if crab_positions.len() % 2 == 0 {
        median = (crab_positions[(crab_positions.len() / 2) - 1]
            + crab_positions[crab_positions.len() / 2])
            / 2;
    } else {
        median = crab_positions[(crab_positions.len() as f32 / 2f32).ceil() as usize]
    }

    let mut fuel_cost = 0;

    for crab_position in crab_positions.iter() {
        fuel_cost += match crab_position.cmp(&median) {
            std::cmp::Ordering::Greater => crab_position - median,
            std::cmp::Ordering::Less => median - crab_position,
            std::cmp::Ordering::Equal => 0,
        }
    }

    println!("Median is {}. Calculated fuel cost: {}", median, fuel_cost);

    let sum: usize = crab_positions.iter().sum();
    let mean = sum / crab_positions.len();

    fuel_cost = 0;

    for crab_position in crab_positions.iter() {
        fuel_cost += match crab_position.cmp(&mean) {
            std::cmp::Ordering::Greater => {
                ((crab_position - mean).pow(2) + (crab_position - mean)) / 2
            }
            std::cmp::Ordering::Less => {
                ((mean - crab_position).pow(2) + (mean - crab_position)) / 2
            }
            std::cmp::Ordering::Equal => 0,
        }
    }

    println!(
        "Fixed fuel calculation: Mean is {}. Calculated fuel cost: {}",
        mean, fuel_cost
    );
}
