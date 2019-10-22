# Sudoku Solver

Solve any sudoku puzzle using javascript and webassembly.

[DEMO!](https://brunt.github.io)

### Building:
* have [rust](https://www.rust-lang.org/), [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/), [npm](https://www.npmjs.com/) installed
* from top-level folder, `wasm-pack build` to compile rust to wasm
* from inside the www folder, run `npm install`, then `npm run start` or `npm run build`

### TODOs:
* performance profiling (can we do better than std::collections::BTreeSet?)
* styling
* alternative algorithms
