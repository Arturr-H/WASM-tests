/*- Global allowings -*/
#![allow(unused_variables, unused_imports)]

/*- Imports -*/
use js_sys::Date;
use std::cmp;
use wasm_bindgen::prelude::*;

/*- Constants & Statics -*/
const WIDTH: i32 = 50i32;
const HEIGHT: i32 = 50i32;

/*- The grid cell -*/
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

/*- The map / grid -*/
#[wasm_bindgen]
pub struct Universe {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
    ms: f64,
}

/*- All grid implementations go here -*/
#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: i32, column: i32) -> usize {
        (row * self.width + column) as usize
    }

    /*- Get the amount of living neighbours -*/
    fn get_neighbour_count(&self, row: i32, column: i32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 { continue; };
                let idx = self.get_index((row + delta_row) % self.height, (column + delta_col) % self.width);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    /*- The update function -*/
    pub fn tick(&mut self) -> f64 {
        let mut next = self.cells.clone();
        let start: f64 = Date::now();

        /*- Iterate over all cells -*/
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.get_neighbour_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation. //? UNDERPOPULATION
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation. //? CONTINUE
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation. //? OVERPOPULATION
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction. //? REPRODUCTION
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
        self.ms = Date::now() - start;
        self.ms
    }

    /*- The 'init' function -*/
    pub fn new() -> Universe {
        let cells = (0..WIDTH * HEIGHT)
            .map(|i| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                }else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width: WIDTH,
            height: HEIGHT,
            cells,
            ms: 0f64,
        }
    }

    /*- Returning pointer-variables, so that we
    won't need to clone the variables in frontend -*/
    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn height(&self) -> i32 {
        self.height
    }
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}
