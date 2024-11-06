use std::fmt;

pub(crate) const GAME_WIDTH: usize = 26;
pub(crate) const GAME_HEIGHT: usize = 14;
pub(crate) const INVENTORY_WIDTH: usize = 15;
pub(crate) const INVENTORY_HEIGHT: usize = 9;
pub(crate) const ALPHA: char = '`';

pub(crate) struct Frame<const WIDTH: usize, const HEIGHT: usize>([[char; WIDTH]; HEIGHT]);

impl<const WIDTH: usize, const HEIGHT: usize> Frame<WIDTH, HEIGHT> {
    pub(crate) fn add_layer(&mut self, pattern: &str) {
        for (j, line) in pattern.lines().enumerate() {
            for (k, char) in line.chars().enumerate() {
                if char != ALPHA {
                    self.0[j][k] = char;
                }
            }
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for Frame<WIDTH, HEIGHT> {
    fn default() -> Self { Self([[' '; WIDTH]; HEIGHT]) }
}

impl<const WIDTH: usize, const HEIGHT: usize> fmt::Display for Frame<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1Bc")?;
        for line in self.0 {
            for char in line {
                write!(f, "{}", char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
