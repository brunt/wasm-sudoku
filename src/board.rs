use itertools::Itertools;
use std::iter::Iterator;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub struct Board {
    pub squares: Vec<Vec<usize>>
}

#[wasm_bindgen]
pub fn show_board(board: &Board) -> Vec<usize> {
    let mut output = Vec::with_capacity(81);
    for v in &board.squares {
        for s in v {
            output.push(s.clone());
        }
    }
    output}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: vec![vec![0; 9]; 9],
        }
    }

    pub fn is_filled(&self, row: usize, column: usize) -> bool {
        if self.squares[row][column] == 0 {
            return false;
        }
        true
    }

//    #[wasm_bindgen]
    pub fn new_with_values(input: Vec<usize>) -> Result<Board, &'static str> {
        if input.len() != 81 {
            return Err("invalid input length");
        }
        let mut board = Board::new();
//        for i in 0..input.len() {
        for i in input.iter().enumerate() {
            board.squares[i.0 / 9][i.0 % 9] = i.1.to_owned()
        }
        Ok(board)
    }

    pub fn get_options_for_cell(&mut self, row: usize, col: usize) -> Vec<usize> {
        let mut options = vec![1,2,3,4,5,6,7,8,9];
        self.remove_column_options(col, &mut options);
        self.remove_row_options(row, &mut options);
        options
    }

    pub fn remove_column_options(&mut self, col: usize, options: &mut Vec<usize>) {
        for i in 1..10 as usize {
            if options.contains(&self.squares[i][col]){
                options.remove(i);
            }
        }
    }

    pub fn remove_row_options(&mut self, row: usize, options: &mut Vec<usize>) {
        for i in 1..10 as usize {
            if options.contains(&self.squares[row][i]){
                options.remove(i);
            }
        }
    }

    pub fn remove_box_options(&mut self, row: usize, col: usize, options: &mut Vec<usize>) {
        let box_row = row - row % 3 as usize;
        let box_col = col - col % 3 as usize;
        for i in 0..3 {
            for j in 0..3 {
                options.remove(self.squares[box_row + i][box_col + j]);
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
        assert_eq!(board.squares[5][4], 49)
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


}
