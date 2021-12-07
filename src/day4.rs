use crate::helper::input_helper::*;

type NumberBoard = [[u8; 5]; 5];

#[derive(Clone, Copy, Debug)]
struct Board {
    numbers: NumberBoard,
    marked_numbers: NumberBoard,
}

impl Board {
    fn new_empty() -> Self {
        Board {
            numbers: [[0; 5]; 5],
            marked_numbers: [[0; 5]; 5],
        }
    }

    fn from_numbers(numbers: NumberBoard) -> Self {
        Board {
            numbers,
            marked_numbers: [[0; 5]; 5],
        }
    }

    fn mark_number(&mut self, number: u8) {
        for i in 0..5 {
            for j in 0..5 {
                if self.numbers[i][j] == number {
                    self.marked_numbers[i][j] = 1;
                }
            }
        }
    }

    fn has_won(self) -> bool {
        for i in 0..5 {
            if self.marked_numbers[i][0]
                & self.marked_numbers[i][1]
                & self.marked_numbers[i][2]
                & self.marked_numbers[i][3]
                & self.marked_numbers[i][4]
                == 1
            {
                return true;
            }

            if self.marked_numbers[0][i]
                & self.marked_numbers[1][i]
                & self.marked_numbers[2][i]
                & self.marked_numbers[3][i]
                & self.marked_numbers[4][i]
                == 1
            {
                return true;
            }
        }

        false
    }

    fn sum_unmarked(self) -> usize {
        let mut sum = 0usize;

        for i in 0..5 {
            for j in 0..5 {
                if self.marked_numbers[i][j] == 0 {
                    sum += self.numbers[i][j] as usize;
                }
            }
        }

        sum
    }

    fn reset_marks(&mut self) {
        self.marked_numbers = [[0; 5]; 5];
    }
}

pub fn day4() {
    println!("--- DAY 4 ---");
    let lines: Vec<String> = read_input_lines(4);
    let drawn_numbers = &lines[0];
    let mut board_line_counter = 0;
    let mut numbers = [[0; 5]; 5];
    let mut board_list: Vec<Board> = vec![];

    for line in lines.iter().skip(2) {
        if line.is_empty() {
            board_line_counter = 0;
            board_list.push(Board::from_numbers(numbers));
            continue;
        }

        let parsed_numbers: Vec<u8> = line
            .split_whitespace()
            .map(|str| str.parse::<u8>().unwrap())
            .collect();

        numbers[board_line_counter][..5].clone_from_slice(&parsed_numbers[..5]);
        board_line_counter += 1;
    }

    board_list.push(Board::from_numbers(numbers));
    let mut victorious_board = Board::new_empty();
    let mut last_drawn_number = 0;

    for drawn_number in drawn_numbers
        .split(',')
        .map(|num| num.parse::<u8>().unwrap())
    {
        let mut should_break = false;
        board_list.iter_mut().for_each(|board| {
            board.mark_number(drawn_number);

            if board.has_won() {
                victorious_board = *board;
                should_break = true;
            }
        });

        if should_break {
            last_drawn_number = drawn_number;
            break;
        }
    }

    let final_score = victorious_board.sum_unmarked() * last_drawn_number as usize;

    println!("Victory! Final score: {}", final_score);

    board_list.iter_mut().for_each(|board| board.reset_marks());

    let mut last_victorious_board = Board::new_empty();
    let mut last_drawn_number = 0;
    let mut victorious_board_count = 0;
    let board_count = board_list.len();
    let mut skip_boards: Vec<usize> = vec![];

    for drawn_number in drawn_numbers
        .split(',')
        .map(|num| num.parse::<u8>().unwrap())
    {
        let mut should_break = false;
        board_list.iter_mut().enumerate().for_each(|(i, board)| {
            board.mark_number(drawn_number);

            if !skip_boards.contains(&i) && board.has_won() {
                skip_boards.push(i);
                last_victorious_board = *board;
                victorious_board_count += 1;

                if victorious_board_count == board_count {
                    should_break = true;
                }
            }
        });

        if should_break {
            last_drawn_number = drawn_number;
            break;
        }
    }

    let final_score = last_victorious_board.sum_unmarked() * last_drawn_number as usize;
    println!("The final board has won! Final score: {}", final_score);
}
