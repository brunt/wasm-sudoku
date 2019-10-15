# Sudoku Solver

Solve any sudoku puzzle using javascript and webassembly.

### Building:
* have rust, wasm-pack, npm installed
* from top-level folder, `wasm-pack build` to compile rust to wasm
* from inside the www folder, run `npm run start` or `npm run build`

### TODOs:
* performance profiling (can we do better than std::collections::HashSet?)
* styling
