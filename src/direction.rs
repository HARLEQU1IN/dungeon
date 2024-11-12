use crate::position::{POSITIONS_COUNT, Position};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

pub(crate) const DIRECTIONS_COUNT: usize = 4;

impl Direction {
    pub(crate) fn turn_left(&mut self) {
        use Direction::*;
        *self = match self {
            Left => Down,
            Right => Up,
            Up => Left,
            Down => Right,
        };
    }

    pub(crate) fn turn_right(&mut self) {
        use Direction::*;
        *self = match self {
            Left => Up,
            Right => Down,
            Up => Right,
            Down => Left,
        };
    }

    pub(crate) fn turn_back(&mut self) {
        use Direction::*;
        *self = match self {
            Left => Right,
            Right => Left,
            Up => Down,
            Down => Up,
        };
    }

    pub(crate) fn get_forward_offset(&self) -> (isize, isize) {
        use Direction::*;
        match self {
            Right => (1, 0),
            Left => (-1, 0),
            Up => (0, -1),
            Down => (0, 1),
        }
    }

    pub(crate) fn get_render_offsets(&self) -> [(Position, isize, isize); POSITIONS_COUNT] {
        use {Direction::*, Position::*};
        const POSITIONS: [(Position, isize, isize); 13] = [
            (A, -2, -3),
            (B, 2, -3),
            (C, -1, -3),
            (D, 1, -3),
            (E, 0, -3),
            (F, -1, -2),
            (G, 1, -2),
            (H, 0, -2),
            (I, -1, -1),
            (J, 1, -1),
            (K, 0, -1),
            (L, -1, 0),
            (M, 1, 0),
        ];

        match self {
            Up => POSITIONS,
            Right => POSITIONS.map(|(p, x, y)| (p, -y, x)),
            Down => POSITIONS.map(|(p, x, y)| (p, -x, -y)),
            Left => POSITIONS.map(|(p, x, y)| (p, y, -x)),
        }
    }
}
