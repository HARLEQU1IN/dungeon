use {
    crate::{blocks::Block, direction::Direction, items::key::Key},
    std::ops::Index,
};

pub(crate) const WIDTH: usize = 11;
pub(crate) const HEIGHT: usize = 11;

pub(crate) struct Map([[Block; WIDTH]; HEIGHT]);

impl Map {
    pub(crate) fn new() -> Self {
        use {Block::*, Direction::*, Key::*};
        Self([
            [
                Wall,
                Door(Down, Brozen),
                Wall,
                Wall,
                Wall,
                Wall,
                Wall,
                Wall,
                Wall,
                Wall,
                Wall,
            ],
            [
                Door(Right, Silver),
                Air,
                Air,
                Wall,
                Air,
                Air,
                Wall,
                Air,
                Air,
                Wall,
                Wall,
            ],
            [
                Case(Right),
                Air,
                Wall,
                Wall,
                Air,
                Air,
                Air,
                Air,
                Wall,
                Wall,
                Wall,
            ],
            [Wall, Air, Air, Air, Air, Air, Air, Air, Air, Air, Wall],
            [
                Wall, Air, Air, Wall, Wall, Wall, Wall, Air, Air, Air, Wall,
            ],
            [Wall, Air, Air, Air, Wall, Air, Wall, Air, Air, Air, Wall],
            [Wall, Air, Air, Air, Wall, Air, Air, Air, Air, Air, Wall],
            [Wall, Air, Air, Air, Air, Air, Wall, Air, Air, Air, Wall],
            [Wall, Air, Air, Air, Air, Air, Wall, Air, Air, Air, Wall],
            [Wall, Air, Air, Air, Air, Air, Wall, Air, Air, Air, Wall],
            [
                Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall,
            ],
        ])
    }

    pub(crate) fn get_neighbour_block(
        &self,
        x: usize,
        y: usize,
        dx: isize,
        dy: isize,
    ) -> (usize, usize, Block) {
        match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
            (Some(x), Some(y)) if x < WIDTH && y < HEIGHT => (x, y, self[(x, y)]),
            _ => (0, 0, Block::Wall),
        }
    }
}

impl Index<(usize, usize)> for Map {
    type Output = Block;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        if x < WIDTH && y < HEIGHT {
            &self.0[y][x]
        } else {
            panic!("Out of bounds")
        }
    }
}
