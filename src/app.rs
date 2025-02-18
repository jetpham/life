use grid::*;
use ratatui::{
    layout::Rect,
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Points},
        Block, Widget,
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
    /// counter
    pub counter: u8,
    /// life grid
    pub grid: Grid<Option<Color>>,
    pub marker: Marker,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            running: true,
            counter: 0,
            grid: Grid::init(height.into(), width.into(), None),
            marker: Marker::Block,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
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
                self.grid.indexed_iter().for_each(|((row, col), color)| {
                    if let Some(color) = color {
                        ctx.draw(&Points {
                            coords: &[(col as f64, row as f64)],
                            color: *color,
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
                    (Some(color), n) if n == 2 || n == 3 => Some(*color),
                    (None, 3) => Some(Color::Black),
                    _ => None,
                };

                *self.grid.get_mut(row, col).unwrap() = next_state;
            }
        }
    }

    fn count_live_neighbors(grid: &Grid<Option<Color>>, row: usize, col: usize) -> usize {
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
