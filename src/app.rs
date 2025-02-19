mod automata;
use automata::*;
use ratatui::{
    layout::Rect,
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
    pub automata: Box<dyn AutomataInterface>,
    pub marker: Marker,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            running: true,
            automata: todo!(),
            marker: Marker::Block,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.automata.step()
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
                let width = self.automata.width();
                let height = self.automata.height();

                for row in 0..height {
                    for col in 0..width {
                        if let Some(color) = self.automata.get_cell_color(col, row) {
                            // Assuming default color means no cell or empty
                            ctx.draw(&Points {
                                coords: &[(col as f64, row as f64)],
                                color: rat_color(color),
                            });
                        }
                    }
                }
            })
    }
}

fn rat_color(color: color_art::Color) -> ratatui::prelude::Color {
    ratatui::style::Color::Rgb(color.red(), color.green(), color.blue())
}
