use color_art::Color;
use grid::*;
use std::fmt;
use std::fmt::Debug;

pub struct Automata<T: Default + Clone + Debug + WithColor> {
    pub grid: Grid<T>,
    pub step: Box<dyn Fn(&Grid<T>) -> Grid<T>>,
}

impl<T: Default + Clone + Debug + WithColor> AutomataInterface for Automata<T> {
    fn step(&mut self) {
        let new_grid = (self.step)(&self.grid);
        self.grid = new_grid;
    }

    fn width(&self) -> usize {
        self.grid.size().1
    }

    fn height(&self) -> usize {
        self.grid.size().0
    }

    fn get_cell_color(&self, x: usize, y: usize) -> Option<Color> {
        self.grid.get(x, y).map(|x| x.get_color()).flatten()
    }
}

impl<T: Default + Clone + Debug + WithColor> Automata<T> {
    pub fn default(width: usize, height: usize) -> Automata<T> {
        Automata {
            grid: Grid::init(height, width, T::default()),
            step: Box::new(|grid| grid.clone()),
        }
    }
}

impl<T: Default + Clone + Debug + WithColor> Debug for Automata<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Automata")
            .field("grid", &self.grid)
            .finish()
    }
}

pub trait AutomataInterface: Debug {
    fn step(&mut self);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get_cell_color(&self, x: usize, y: usize) -> Option<Color>;
}

pub trait WithColor: Debug {
    fn get_color(&self) -> Option<Color>;
}
