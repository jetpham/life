use color_art::Color;

pub trait Automaton {
    fn colors<'a>(&'a self) -> Box<dyn Iterator<Item = ((usize, usize), Color)> + 'a>;
    fn size(&self) -> (usize, usize);
    fn resize(&mut self);
    fn step(&mut self);
    fn draw(&mut self, draw_row: usize, draw_col: usize);
}
