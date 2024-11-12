use {
    crate::{blocks::Block},
    rand::{Rng, seq::SliceRandom},
    std::ops::Index,
};

pub(crate) const WIDTH: usize = 11;
pub(crate) const HEIGHT: usize = 11;

pub(crate) struct Map([[Block; WIDTH]; HEIGHT]);

impl Map {
    pub(crate) fn new() -> Self {
        let mut map = Self::empty_map();
        map.generate_maze();
        map
    }

    fn empty_map() -> Self { Self([[Block::Wall; WIDTH]; HEIGHT]) }

    fn generate_maze(&mut self) {
        let mut rng = rand::thread_rng();
        let start_x = rng.gen_range(1..WIDTH - 1);
        let start_y = rng.gen_range(1..HEIGHT - 1);

        self.0[start_y][start_x] = Block::Air;

        let mut visited = vec![vec![false; WIDTH]; HEIGHT];
        visited[start_y][start_x] = true;

        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        let mut queue = vec![(start_x, start_y)];

        while let Some((x, y)) = queue.pop() {
            let mut shuffled_dirs = directions.to_vec();
            shuffled_dirs.shuffle(&mut rng);

            for (dx, dy) in shuffled_dirs {
                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;

                if nx < WIDTH && ny < HEIGHT && !visited[ny][nx] {
                    self.0[ny][nx] = Block::Air;
                    visited[ny][nx] = true;
                    queue.push((nx, ny));
                }
            }
        }

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.0[y][x] == Block::Air && rng.gen_bool(0.2) {
                    self.0[y][x] = Block::Wall;
                }
            }
        }
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
