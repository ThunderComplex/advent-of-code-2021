use itertools::Itertools;

use crate::helper::input_helper::*;

macro_rules! safe_get {
    ($heightmap:expr, $default:expr, $i:expr, $j:expr) => {
        if $i < 0 || $j < 0 {
            &10
        } else {
            $heightmap
                .get($i as usize)
                .unwrap_or($default)
                .get($j as usize)
                .unwrap_or(&10)
        }
    };
}

type Point = (i32, i32, u32);

pub fn day9() {
    println!("--- DAY 9 ---");
    let lines: Vec<String> = read_input_lines(9);
    let heightmap: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .map(|char_list| char_list.iter().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let width = heightmap[0].len();
    let height = heightmap.len();
    let mut lowest_points: Vec<Point> = vec![];

    let default_vec: Vec<u32> = vec![];
    for i in 0..height as isize {
        for j in 0..width as isize {
            let top_left = safe_get!(heightmap, &default_vec, i - 1, j - 1);
            let top = safe_get!(heightmap, &default_vec, i - 1, j);
            let top_right = safe_get!(heightmap, &default_vec, i - 1, j + 1);
            let left = safe_get!(heightmap, &default_vec, i, j - 1);
            let center = safe_get!(heightmap, &default_vec, i, j);
            let right = safe_get!(heightmap, &default_vec, i, j + 1);
            let bottom_left = safe_get!(heightmap, &default_vec, i + 1, j - 1);
            let bottom = safe_get!(heightmap, &default_vec, i + 1, j);
            let bottom_right = safe_get!(heightmap, &default_vec, i + 1, j + 1);

            if center
                .min(top_left)
                .min(top)
                .min(top_right)
                .min(left)
                .min(right)
                .min(bottom_left)
                .min(bottom)
                .min(bottom_right)
                == center
            {
                lowest_points.push((i.try_into().unwrap(), j.try_into().unwrap(), *center));
            }
        }
    }

    let risk_sum: u32 = lowest_points.iter().map(|p| p.2 + 1).sum();
    println!("The risk level sum is {}", risk_sum);

    let mut basins: Vec<Vec<Point>> = vec![];
    for point in lowest_points {
        basins.push(get_basin(&heightmap, point.0, point.1, point.2));
    }

    basins = dedup_basin_points(&basins);

    let basin_result: usize = basins
        .iter()
        .map(|v| v.len() + 1)
        .sorted()
        .rev()
        .take(3)
        .product();

    println!(
        "The 3 largest basins multiplied together result in {}",
        basin_result
    );
}

fn dedup_basin_points(basin_list: &[Vec<Point>]) -> Vec<Vec<Point>> {
    let mut deduped_list: Vec<Vec<Point>> = vec![];

    for point_list in basin_list {
        let mut deduped_point_list: Vec<Point> = vec![];

        for point in point_list {
            if !deduped_point_list.contains(point) {
                deduped_point_list.push(*point);
            }
        }
        deduped_list.push(deduped_point_list);
    }

    deduped_list
}

fn get_basin(heightmap: &[Vec<u32>], i: i32, j: i32, val: u32) -> Vec<Point> {
    if val >= 9 {
        return vec![];
    }

    let default_vec: Vec<u32> = vec![];

    let top = safe_get!(heightmap, &default_vec, i - 1, j);
    let left = safe_get!(heightmap, &default_vec, i, j - 1);
    let right = safe_get!(heightmap, &default_vec, i, j + 1);
    let bottom = safe_get!(heightmap, &default_vec, i + 1, j);

    let mut basin = vec![];

    if *top < 9 && *top > val {
        basin.append(&mut get_basin(heightmap, i - 1, j, *top));
        basin.push((i - 1, j, *top));
    }

    if *left < 9 && *left > val {
        basin.append(&mut get_basin(heightmap, i, j - 1, *left));
        basin.push((i, j - 1, *left));
    }

    if *right < 9 && *right > val {
        basin.append(&mut get_basin(heightmap, i, j + 1, *right));
        basin.push((i, j + 1, *right));
    }

    if *bottom < 9 && *bottom > val {
        basin.append(&mut get_basin(heightmap, i + 1, j, *bottom));
        basin.push((i + 1, j, *bottom));
    }

    basin
}
