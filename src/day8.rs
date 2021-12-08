use std::collections::HashSet;

use crate::helper::input_helper::*;

pub fn day8() {
    println!("--- DAY 8 ---");
    let lines: Vec<String> = read_input_lines(8);
    let count = lines
        .iter()
        .map(|line| line.split('|').last().unwrap())
        .map(|output| output.split_whitespace().collect::<Vec<&str>>())
        .flatten()
        .filter(|s| [2, 3, 4, 7].contains(&s.len()))
        .fold(0, |acc: usize, _| acc + 1);

    println!("Digits 1, 4, 7, 8 appear {} times", count);

    let inputs: Vec<Vec<String>> = lines
        .iter()
        .map(|line| {
            line.split('|')
                .next()
                .unwrap()
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let outputs: Vec<Vec<String>> = lines
        .iter()
        .map(|line| {
            line.split('|')
                .last()
                .unwrap()
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let mut decoded_total = 0;

    for (i, input) in inputs.iter().enumerate() {
        let mut segment_map: [HashSet<char>; 10] = Default::default();

        for segment in input {
            match segment.len() {
                2 => segment_map[1] = segment.chars().into_iter().collect(),
                3 => segment_map[7] = segment.chars().into_iter().collect(),
                4 => segment_map[4] = segment.chars().into_iter().collect(),
                7 => segment_map[8] = segment.chars().into_iter().collect(),
                _ => (),
            }
        }

        for segment in input {
            if segment.len() == 5 {
                let input_set: HashSet<char> = segment.chars().into_iter().collect();

                if segment_map[1].difference(&input_set).count() == 0 {
                    segment_map[3] = input_set;
                } else if input_set.difference(&segment_map[4]).count() == 2 {
                    segment_map[5] = input_set;
                } else if input_set.difference(&segment_map[4]).count() == 3 {
                    segment_map[2] = input_set;
                }
            }
        }

        for segment in input {
            if segment.len() == 6 {
                let input_set: HashSet<char> = segment.chars().into_iter().collect();

                if segment_map[1].difference(&input_set).count() == 1 {
                    segment_map[6] = input_set;
                } else if segment_map[5].difference(&input_set).count() == 0 {
                    segment_map[9] = input_set;
                } else {
                    segment_map[0] = input_set;
                }
            }
        }

        let mut decoded_number = 0;

        for (j, output) in outputs[i].iter().rev().enumerate() {
            let output_set: HashSet<char> = output.chars().into_iter().collect();
            decoded_number +=
                segment_map.iter().position(|s| s == &output_set).unwrap() * 10usize.pow(j as u32);
        }

        decoded_total += decoded_number;
    }

    println!(
        "All segment display decoded! Total combined value: {}",
        decoded_total
    );
}
