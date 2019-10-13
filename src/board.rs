use itertools::Itertools;
use std::iter::Iterator;
use wasm_bindgen::prelude::*;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Board {
    pub squares: Vec<Vec<usize>>
}

//#[wasm_bindgen]
pub fn show_board(board: &Board) -> Vec<usize> {
    let mut output = Vec::with_capacity(81);
    for v in &board.squares {
        for s in v {
            output.push(s.clone());
        }
    }
    output
}

#[wasm_bindgen]
pub fn solve_board(board: &mut Board) -> Vec<usize> {
    //recursion?
    show_board(board)
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: vec![vec![0; 9]; 9],
        }
    }

//    #[wasm_bindgen]
    pub fn new_with_values(input: Vec<usize>) -> Result<Board, &'static str> {
        if input.len() != 81 {
            return Err("invalid input length");
        }
        let mut board = Board::new();
        for i in input.iter().enumerate() {
            board.squares[i.0 / 9][i.0 % 9] = i.1.to_owned()
        }
        Ok(board)
    }

    pub fn is_filled(&self, row: usize, column: usize) -> bool {
        if self.squares[row][column] == 0 {
            return false;
        }
        true
    }

    pub fn get_options_for_cell(&self, row: usize, col: usize) -> HashSet<usize> {
        let mut options = HashSet::with_capacity(9);
        for i in 1..10 as usize {
            options.insert(i);
        }
        self.remove_column_options(col, &mut options);
        self.remove_row_options(row, &mut options);
        self.remove_box_options(row, col, &mut options);
        options
    }

    pub fn remove_column_options(&self, col: usize, options: &mut HashSet<usize>) {
        for i in 0..9 {
            options.remove(&self.squares[i][col]);
        }
    }

    pub fn remove_row_options(&self, row: usize, options: &mut HashSet<usize>) {
        for i in 0..9 {
            options.remove(&self.squares[row][i]);
        }
    }

    pub fn remove_box_options(&self, row: usize, col: usize, options: &mut HashSet<usize>) {
        let box_row = row - row % 3 as usize;
        let box_col = col - col % 3 as usize;
        for i in 0..3 {
            for j in 0..3 {
                options.remove(&self.squares[box_row + i][box_col + j]);
            }
        }
    }

    fn clear_cell(&mut self, row: usize, col: usize) {
        self.squares[row][col] = 0 as usize;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let x = Board::new();
        assert_eq!(x.squares.capacity(), 9)
    }

    #[test]
    fn test_is_filled() {
        let board = Board::new();
        assert_eq!(board.is_filled(0, 0), false);
        let mut board2 = Board::new();
        board2.squares[1][4] = 3;
        assert_eq!(board2.is_filled(1,4), true)
    }

    #[test]
    fn test_new_with_values() {
        let mut input = Vec::with_capacity(81);
        for i in 0..81 {
            input.push(i);
        }
        let board = Board::new_with_values(input).unwrap();
        assert_eq!(board.squares[5][4], 49);

        let cell_vals = vec![
            2,9,6,3,1,8,5,7,4,
            5,8,4,9,7,2,6,1,3,
            7,1,3,6,4,5,2,8,9,
            6,2,5,8,9,7,3,4,1,
            9,3,1,4,2,6,8,5,7,
            4,7,8,5,3,1,9,2,6,
            1,6,7,2,5,3,4,9,8,
            8,5,9,7,6,4,1,3,2,
            3,4,2,1,8,9,7,6,5
        ];
        let board = Board::new_with_values(cell_vals).unwrap();
        assert_eq!(board.squares[0][1], 9);
    }

    #[test]
    fn test_show_board() {
        let mut input = Vec::with_capacity(81);
        for i in 0..81 {
            input.push(i);
        }
        let board = Board::new_with_values(input.clone()).unwrap();
        assert_eq!(show_board(&board), input)
    }

    #[test]
    fn test_get_option_for_cell() {
        let cell_vals = vec![
            2,9,6,3,1,8,5,7,4,
            5,8,4,0,7,2,6,1,3,
            7,1,3,6,4,5,2,8,9,
            6,2,5,8,9,7,3,4,1,
            9,3,1,4,2,6,8,5,7,
            4,7,8,5,3,1,9,2,6,
            1,6,7,2,5,3,4,9,8,
            8,5,9,7,6,4,1,3,2,
            3,4,2,1,8,9,7,6,5
        ];
        let board = Board::new_with_values(cell_vals).unwrap();
        let options = board.get_options_for_cell(1, 3);
        assert_eq!(options.contains(&9), true);
        assert_eq!(options.contains(&0), false);
    }
}
