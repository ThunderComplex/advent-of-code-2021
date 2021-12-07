use crate::helper::input_helper::*;
use itertools::Itertools;

pub fn day1() {
    println!("--- DAY 1 ---");
    let lines: Vec<u16> = read_input_lines(1);
    let mut increased_counter = 0;

    lines
        .iter()
        .zip(lines.iter().skip(1))
        .for_each(|(first, second)| {
            if second > first {
                increased_counter += 1
            }
        });

    println!("There are {} increased depths", increased_counter);

    let mut last_window_sum = 0;
    let mut window_increases = 0;

    for (first, second, third) in lines.iter().tuple_windows() {
        let current_window_sum = first + second + third;

        if last_window_sum > 0 && current_window_sum > last_window_sum {
            window_increases += 1;
        }

        last_window_sum = current_window_sum;
    }

    println!("There are {} 3-value window increases", window_increases);
}
