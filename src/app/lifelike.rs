use color_art::Color;
use grid::Grid;
use log::info;
use rand::{rng, Rng};
use std::fmt::Debug;
use std::vec::Vec;

use super::Automaton;

#[derive(Debug)]
pub struct LifeLikeAutomaton {
    grid: Grid<bool>,
    birth: Vec<u8>,
    survival: Vec<u8>,
}

impl LifeLikeAutomaton {
    pub fn new(width: usize, height: usize, birth: Vec<u8>, survival: Vec<u8>) -> Self {
        let mut rng = rng();
        let random_grid = Grid::from_vec(
            (0..width * height).map(|_| rng.random_bool(0.5)).collect(),
            width,
        );
        info!(
            "grid initialized\nsize:({}, {})\nbirth: {:?}\nsurvival: {:?}\ndensity: {}",
            width,
            height,
            birth,
            survival,
            random_grid.iter().filter(|x| **x).count()
        );
        LifeLikeAutomaton {
            grid: random_grid,
            birth,
            survival,
        }
    }
}

impl Automaton for LifeLikeAutomaton {
    fn step(&mut self) {
        let grid = &self.grid;
        let (rows, cols) = grid.size();
        let mut new_grid = Grid::init(rows, cols, false);

        for y in 0..rows {
            for x in 0..cols {
                let mut alive_neighbors = 0;

                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        let neighbor_x = x as i32 + i;
                        let neighbor_y = y as i32 + j;

                        if neighbor_x >= 0
                            && neighbor_x < cols as i32
                            && neighbor_y >= 0
                            && neighbor_y < rows as i32
                        {
                            if let Some(neighbor) =
                                grid.get(neighbor_y as usize, neighbor_x as usize)
                            {
                                if *neighbor {
                                    alive_neighbors += 1;
                                }
                            }
                        }
                    }
                }

                let current_cell_alive = grid[(y, x)];

                if current_cell_alive {
                    new_grid[(y, x)] = self.survival.contains(&alive_neighbors);
                } else {
                    new_grid[(y, x)] = self.birth.contains(&alive_neighbors)
                };
            }
        }
        self.grid = new_grid
    }

    fn colors<'a>(&'a self) -> Box<dyn Iterator<Item = ((usize, usize), Color)> + 'a> {
        Box::new(
            self.grid
                .indexed_iter()
                .map(|(x, y)| (x, bool_to_color(*y))),
        )
    }

    fn size(&self) -> (usize, usize) {
        self.grid.size()
    }

    fn resize(&mut self) {
        todo!()
    }

    fn draw(&mut self, draw_row: usize, draw_col: usize) {
        if let Some(state) = self.grid.get_mut(draw_row, draw_col) {
            *state = !*state;
            info!("Cell at ({}, {}) toggled", draw_row, draw_col);
        } else {
            info!(
                "Draw missed: coordinates ({}, {}) out of bounds",
                draw_row, draw_col
            );
        }
    }
}
fn bool_to_color(value: bool) -> Color {
    if value {
        Color::from_name("white").unwrap()
    } else {
        Color::from_name("black").unwrap()
    }
}
