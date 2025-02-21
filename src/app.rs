mod automata;
mod lifelifecolor;
mod lifelike;
use std::error;

use automata::*;
use lifelifecolor::LifeLikeColorAutomaton;
use lifelike::LifeLikeAutomaton;
use log::info;
use ratatui::{
    layout::Rect,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Points},
        Widget,
    },
};
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// life grid
    pub automaton: Box<dyn Automaton>,
    pub marker: Marker,
}

impl App {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            running: true,
            automaton: Box::new(LifeLikeColorAutomaton::new(
                width.into(),
                height.into(),
                vec![3],
                vec![2, 3],
            )),
            marker: Marker::Block,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        info!("automaton stepped");
        self.automaton.step()
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
                self.automaton.colors().for_each(|((row, col), color)| {
                    ctx.draw(&Points {
                        coords: &[(col as f64, row as f64)],
                        color: rat_color(color),
                    });
                });
            })
    }

    pub fn draw(&mut self, draw_row: usize, draw_col: usize) {
        self.automaton.draw(draw_row, draw_col);
    }
}

fn rat_color(color: color_art::Color) -> ratatui::prelude::Color {
    ratatui::style::Color::Rgb(color.red(), color.green(), color.blue())
}
