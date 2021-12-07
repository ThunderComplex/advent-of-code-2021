#![allow(clippy::if_same_then_else)]
use crate::helper::input_helper::*;

#[derive(Clone, Copy, Debug)]
struct Line {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
}

pub fn day5() {
    println!("--- DAY 5 ---");
    let lines: Vec<String> = read_input_lines(5);
    let line_list: Vec<Line> = lines
        .iter()
        .map(|line| {
            let points: Vec<isize> = line
                .split(" -> ")
                .map(|s| {
                    s.split(',')
                        .map(|p| p.parse::<isize>().unwrap())
                        .collect::<Vec<isize>>()
                })
                .flatten()
                .collect();
            Line {
                x1: points[0],
                y1: points[1],
                x2: points[2],
                y2: points[3],
            }
        })
        .collect();

    let mut grid = vec![vec![0u8; 1000]; 1000];
    let mut overlap_count = 0;

    for y in 0..1000 {
        for x in 0..1000 {
            line_list.iter().for_each(|line| {
                let dxc = x - line.x1;
                let dyc = y - line.y1;

                let dx = line.x2 - line.x1;
                let dy = line.y2 - line.y1;

                let cross = (dxc * dy) - (dyc * dx);

                if cross == 0 {
                    if dx.abs() >= dy.abs() {
                        if dx > 0 && (line.x1 <= x && x <= line.x2) {
                            grid[y as usize][x as usize] += 1;
                        } else if line.x2 <= x && x <= line.x1 {
                            grid[y as usize][x as usize] += 1;
                        }
                    } else if dy > 0 && (line.y1 <= y && y <= line.y2) {
                        grid[y as usize][x as usize] += 1;
                    } else if line.y2 <= y && y <= line.y1 {
                        grid[y as usize][x as usize] += 1;
                    }
                }
            });
        }
    }

    for row in grid {
        for col in row {
            if col > 1 {
                overlap_count += 1;
            }
        }
    }

    println!("There are {} overlapping points!", overlap_count);
}
