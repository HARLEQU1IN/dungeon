use {
    crate::{
        direction::{Direction, DIRECTIONS_COUNT},
        items::key::Key,
        position::{Position, POSITIONS_COUNT},
    }, air::PATTERN, exit::PATTERNS, std::fmt
};

mod air;
mod exit;
mod wall;


const PATTERNS_COUNT: usize = POSITIONS_COUNT * DIRECTIONS_COUNT;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Block {
    Air,
    Wall,
    Door(Direction, Key),
}

impl Block {
    pub fn get_pattern(&self, position: Position, direction: Direction) -> &str {
        use Block::*;
        let index = position as usize * 4;
        match self {
            Air => air::PATTERN,
            Wall => wall::PATTERNS[index + direction as usize],
            Door(exit_direction, _) => {
                let direction = (direction as isize - *exit_direction as isize).rem_euclid(4);
                exit::PATTERNS[index + direction as usize]
            },
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Block::*;
        match self {
            Air => write!(f, "Воздух"),
            Wall => write!(f, "Стена"),
            Door(..) => write!(f, "Дверь"),
        }
    }
}
