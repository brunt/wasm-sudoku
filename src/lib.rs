use itertools::Itertools;
use std::iter::Iterator;
use wasm_bindgen::prelude::*;
use std::collections::BTreeSet;

pub struct Board {
    pub squares: Vec<Vec<u32>>
}

#[wasm_bindgen]
pub fn solve_from_array(input: Vec<u32>) -> Vec<u32> {
    let mut board = Board::new_with_values(input).unwrap_or_else(|_| Board::new());
    let mut tree = BTreeSet::new();
    for i in 1..10 as u32 {
        tree.insert(i);
    }
    board.find_solution(0, tree);
    board.show()
}

impl Board {
    fn new() -> Board {
        Board {
            squares: vec![vec![0; 9]; 9],
        }
    }

    fn new_with_values(input: Vec<u32>) -> Result<Board, &'static str> {
        if input.len() != 81 {
            return Err("invalid input length");
        }
        let mut board = Board::new();
        for i in input.iter().enumerate() {
            board.squares[i.0 / 9][i.0 % 9] = i.1.to_owned()
        }
        Ok(board)
    }

    fn is_filled(&self, row: usize, column: usize) -> bool {
        if self.squares[row][column] == 0 {
            return false;
        }
        true
    }

    fn get_options_for_cell(&self, row: usize, col: usize, mut options: BTreeSet<u32>) -> BTreeSet<u32> {
        self.remove_options(row, col, &mut options);
        options
    }

    fn remove_options(&self, row: usize, col: usize, options: &mut BTreeSet<u32>) {
        for i in 0..9 {
            options.remove(&self.squares[row][i]);
            options.remove(&self.squares[i][col]);
        }
        let box_row = row - row % 3 as usize;
        let box_col = col - col % 3 as usize;
        for i in 0..3 {
            for j in 0..3 {
                options.remove(&self.squares[box_row + i][box_col + j]);
            }
        }
    }

    fn clear_cell(&mut self, row: usize, col: usize) {
        self.squares[row][col] = 0 as u32;
    }

    fn find_solution(&mut self, index: usize, options: BTreeSet<u32>) -> bool {
        if index == 81 {
            return true;
        }

        let row = index / 9;
        let col = index % 9;

        if self.is_filled(row, col) {
            return self.find_solution(index + 1, options)
        }
        let ops = self.get_options_for_cell(row, col, options.clone());
        for i in ops.iter() {
            self.squares[row][col] = i.to_owned() as u32;
            if self.find_solution(index + 1, options.clone()) {
                return true;
            }
        }
        self.clear_cell(row, col);
        false
    }

    fn show(&self) -> Vec<u32> {
        self.squares.to_owned().into_iter().flatten().collect()
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
        assert_eq!(board.show(), input)
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
        let mut tree = BTreeSet::new();
        for i in 1..10 {
            tree.insert(i);
        }
        let options = board.get_options_for_cell(1, 3, tree);
        assert_eq!(options.contains(&9), true);
        assert_eq!(options.contains(&0), false);
    }

    #[test]
    fn test_find_solution() {
        let cell_vals = vec![
            2,9,6,0,0,0,5,7,4,
            5,8,4,0,0,0,6,1,3,
            7,1,3,0,0,0,2,8,9,
            6,2,5,8,9,7,3,4,1,
            9,3,1,4,2,6,8,5,7,
            4,7,8,5,3,1,9,2,6,
            1,6,7,0,5,3,4,9,8,
            8,5,9,7,0,4,1,3,2,
            3,4,2,1,8,0,7,0,5
        ];
        let solution =  vec![
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
        assert_eq!(solve_from_array(cell_vals), solution);
    }
}
