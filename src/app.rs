use color_art::Color;
use grid::*;
use rand::prelude::*;
use ratatui::{
    layout::Rect,
    style::Color as OtherColor,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Points},
        Widget,
    },
};
use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// life grid
    pub grid: Grid<Option<Cell>>,
    pub width: u16,
    pub height: u16,
    pub marker: Marker,
}

#[derive(Debug, Default, Clone)]
pub struct Cell {
    age: usize,
    color: Option<Color>,
}

impl Cell {
    fn get_color(&self) -> Option<OtherColor> {
        if let Some(color) = self.color {
            Some(OtherColor::Rgb(color.red(), color.green(), color.blue()))
        } else {
            None
        }
    }
}

/// Function to generate a grid of Option<Color> with given width, height, and density
fn generate_color_grid(width: usize, height: usize, density: u8) -> Grid<Option<Cell>> {
    let mut grid = Grid::new(height, width);

    for row in 0..height {
        for col in 0..width {
            grid[(row, col)] = generate_random_option_cell(density);
        }
    }

    grid
}

fn generate_random_option_cell(density: u8) -> Option<Cell> {
    let mut rng = rand::rng();

    if rng.random_range(1..=100) <= density {
        let r: f64 = rng.random_range(0.0..=360.0);
        return Some(Cell {
            color: Color::from_hsv(r, 1.0, 1.0).ok(),
            age: 0,
        });
    }

    None
}
fn generate_random_cell() -> Cell {
    let mut rng = rand::rng();

    let r: f64 = rng.random_range(0.0..=360.0);
    return Cell {
        color: Color::from_hsv(r, 1.0, 1.0).ok(),
        age: 0,
    };
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            running: true,
            grid: generate_color_grid(width.into(), height.into(), 50),
            marker: Marker::Block,
            width,
            height,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.step()
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
    pub fn life_canvas(&self, area: Rect) -> impl Widget + '_ {
        let left = 0.0;
        let right = f64::from(area.width);
        let bottom = 0.0;
        let top = f64::from(area.height);
        Canvas::default()
            .marker(self.marker)
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .paint(|ctx| {
                self.grid.indexed_iter().for_each(|((row, col), cell)| {
                    if let Some(cell) = cell {
                        ctx.draw(&Points {
                            coords: &[(col as f64, row as f64)],
                            color: match cell.get_color() {
                                Some(color) => color,
                                None => OtherColor::Black,
                            },
                        })
                    }
                })
            })
    }

    pub fn step(&mut self) {
        let current_grid = self.grid.clone();

        for row in 0..self.grid.rows() {
            for col in 0..self.grid.cols() {
                let live_neighbors = Self::count_live_neighbors(&current_grid, row, col);

                let current_state = current_grid.get(row, col).unwrap_or(&None);

                let next_state = match (current_state, live_neighbors) {
                    (Some(_), n) if !(2..=3).contains(&n) => None,
                    (Some(color), n) if n == 2 || n == 3 => Some(color.clone()),
                    (None, 3) => Some(generate_random_cell()),
                    _ => None,
                };

                *self.grid.get_mut(row, col).unwrap() = next_state;
            }
        }
    }

    pub fn reset(&mut self) {
        self.grid = generate_color_grid(self.width.into(), self.height.into(), 50);
    }

    fn count_live_neighbors(grid: &Grid<Option<Cell>>, row: usize, col: usize) -> usize {
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        directions
            .iter()
            .filter_map(|(dr, dc)| {
                let neighbor_row = row.wrapping_add(*dr as usize);
                let neighbor_col = col.wrapping_add(*dc as usize);

                if neighbor_row < grid.rows() && neighbor_col < grid.cols() {
                    grid.get(neighbor_row, neighbor_col)
                } else {
                    None
                }
            })
            .filter(|neighbor| neighbor.is_some())
            .count()
    }
}
