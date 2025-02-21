use color_art::Color;
use grid::Grid;
use log::info;
use rand::{rng, Rng};
use std::fmt::Debug;
use std::vec::Vec;

use super::Automaton;

#[derive(Debug, Clone)]
enum Cell {
    Alive(Color),
    Dead,
}

impl Cell {
    fn from_neighbors(
        &self,
        neighbors: Vec<&Cell>,
        survival: &Vec<usize>,
        birth: &Vec<usize>,
    ) -> Cell {
        let neighbor_count = neighbors
            .iter()
            .filter(|cell| match cell {
                Cell::Alive(_) => true,
                Cell::Dead => false,
            })
            .count();
        match self {
            Cell::Alive(_) => {
                if survival.contains(&neighbor_count) {
                    self.clone()
                } else {
                    Cell::Dead
                }
            }
            Cell::Dead => {
                if birth.contains(&neighbor_count) {
                    let neighbor_colors: Vec<&Color> = neighbors
                        .iter()
                        .filter_map(|cell| match cell {
                            Cell::Alive(color) => Some(color),
                            Cell::Dead => None,
                        })
                        .collect();
                    Cell::Alive(mix_colors(neighbor_colors))
                } else {
                    self.clone()
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct LifeLikeColorAutomaton {
    grid: Grid<Cell>,
    birth: Vec<usize>,
    survival: Vec<usize>,
}

fn random_color() -> Color {
    let mut rng = rng();
    Color::from_hsv(rng.random_range(0.0..=360.0), 1.0, 1.0).unwrap()
}
impl LifeLikeColorAutomaton {
    pub fn new(width: usize, height: usize, birth: Vec<usize>, survival: Vec<usize>) -> Self {
        let mut rng = rng();
        let random_grid = Grid::from_vec(
            (0..width * height)
                .map(|_| -> Cell {
                    match rng.random_bool(0.5) {
                        true => Cell::Alive(random_color()),
                        false => Cell::Dead,
                    }
                })
                .collect(),
            width,
        );
        info!(
            "grid initialized\nsize:({}, {})\nbirth: {:?}\nsurvival: {:?}\ndensity: {}",
            width,
            height,
            birth,
            survival,
            random_grid
                .iter()
                .filter(|x| matches!(x, Cell::Alive(_)))
                .count()
        );
        LifeLikeColorAutomaton {
            grid: random_grid,
            birth,
            survival,
        }
    }

    fn get_neighbors(&self, row: usize, col: usize) -> Vec<&Cell> {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .fold(Vec::new(), |mut neighbors, (moore_row, moore_col)| {
            let row = match add_and_cast(row, *moore_row) {
                Some(row) => row,
                None => return neighbors,
            };
            let col = match add_and_cast(col, *moore_col) {
                Some(col) => col,
                None => return neighbors,
            };
            if let Some(neighbor) = self.grid.get(row, col) {
                neighbors.push(neighbor);
            }
            neighbors
        })
    }
}

fn add_and_cast(a: usize, b: i8) -> Option<usize> {
    let b_as_isize = b as isize;
    a.checked_add_signed(b_as_isize)
}

fn mix_colors(colors: Vec<&Color>) -> Color {
    let (sum_sin, sum_cos) = colors
        .iter()
        .map(|color| color.hue())
        .fold((0.0, 0.0), |acc, h| {
            let rad = h * std::f64::consts::PI / 180.0;
            (acc.0 + rad.sin(), acc.1 + rad.cos())
        });
    let mut avg_hue = sum_sin.atan2(sum_cos) * 180.0 / std::f64::consts::PI;
    if avg_hue < 0.0 {
        avg_hue += 360.0
    }
    Color::from_hsv(avg_hue, 1.0, 1.0).unwrap()
}

impl Automaton for LifeLikeColorAutomaton {
    fn step(&mut self) {
        let grid = &self.grid;
        let mut new_grid = grid.clone();

        new_grid.indexed_iter_mut().for_each(|((row, col), cell)| {
            let neighbors = self.get_neighbors(row, col);
            *cell = cell.from_neighbors(neighbors, &self.survival, &self.birth);
        });
        self.grid = new_grid
    }

    fn colors<'a>(&'a self) -> Box<dyn Iterator<Item = ((usize, usize), Color)> + 'a> {
        Box::new(self.grid.indexed_iter().filter_map(|(x, y)| match y {
            Cell::Alive(color) => Some((x, *color)),
            Cell::Dead => None,
        }))
    }

    fn size(&self) -> (usize, usize) {
        self.grid.size()
    }

    fn resize(&mut self) {
        todo!()
    }

    fn draw(&mut self, draw_row: usize, draw_col: usize) {
        if let Some(state) = self.grid.get_mut(draw_row, draw_col) {
            *state = Cell::Alive(random_color());
            info!("Cell at ({}, {}) toggled", draw_row, draw_col);
        } else {
            info!(
                "Draw missed: coordinates ({}, {}) out of bounds",
                draw_row, draw_col
            );
        }
    }
}
