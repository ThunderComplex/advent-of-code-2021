use core::panic;

use crate::helper::input_helper::*;

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    delta: usize,
}

pub fn day2() {
    println!("--- DAY 2 ---");
    let lines: Vec<String> = read_input_lines(2);
    let instruction_list: Vec<Instruction> = lines
        .iter()
        .map(|line| string_to_instruction(line))
        .collect();

    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    instruction_list
        .iter()
        .for_each(|instruction| match instruction.direction {
            Direction::Forward => horizontal_position += instruction.delta,
            Direction::Down => aim += instruction.delta,
            Direction::Up => aim -= instruction.delta,
        });

    println!(
        "Final horizontal position: {}, depth: {} || Result: {}",
        horizontal_position,
        depth,
        horizontal_position * depth
    );

    horizontal_position = 0;
    depth = 0;
    aim = 0;

    instruction_list
        .iter()
        .for_each(|instruction| match instruction.direction {
            Direction::Forward => {
                horizontal_position += instruction.delta;
                depth += aim * instruction.delta;
            }
            Direction::Down => aim += instruction.delta,
            Direction::Up => aim -= instruction.delta,
        });

    println!(
        "Proper calculations done! Final horizontal position: {}, depth: {} || Result: {}",
        horizontal_position,
        depth,
        horizontal_position * depth
    );
}

fn string_to_instruction(input: &str) -> Instruction {
    let split: Vec<&str> = input.split_whitespace().collect();
    let direction = match split[0] {
        "forward" => Direction::Forward,
        "down" => Direction::Down,
        "up" => Direction::Up,
        _ => panic!("Input {} contains unknown direction", input),
    };

    Instruction {
        direction,
        delta: split[1].parse().unwrap(),
    }
}
