use std::{str::FromStr};

pub fn read_input_lines<T>(day: u8) -> std::io::Result<Vec<T>>
    where T: FromStr + Default {
    let file_string = std::fs::read_to_string(format!("src/input/day{}", day))?;
    Ok(file_string
        .lines()
        .map(|line| line.parse::<T>().unwrap_or_default())
        .collect())
}