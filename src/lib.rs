
use crate::board::Board;
use wasm_bindgen::prelude::*;


mod board;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//https://github.com/jhannes-playpen/sudoku-kata/blob/commit_per_test/src/main/java/com/brodwall/kata/sudoku/SudokuSolver.java


