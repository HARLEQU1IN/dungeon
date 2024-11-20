use {
    crate::{blocks::Block, enemy::Enemy},
    rand::{
        Rng,
        seq::{IteratorRandom, SliceRandom},
    },
    std::ops::{Index, IndexMut},
};

pub(crate) const WIDTH: usize = 11;
pub(crate) const HEIGHT: usize = 11;

pub(crate) struct Map([[Block; WIDTH]; HEIGHT]);

impl Map {
    pub(crate) fn new() -> Self {
        let mut map = Self::empty_map();
        map.generate_maze();
        map.place_doors_and_cases_and_enemy();
        map
    }

    fn empty_map() -> Self {
        use std::array::from_fn;
        Self(from_fn(|_| from_fn(|_| Block::Wall)))
    }

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

    fn place_doors_and_cases_and_enemy(&mut self) {
        let mut rng = rand::thread_rng();
        let mut door_count = 0;
        let mut chest_count = 0;
        let mut enemy_count = 0;

        while door_count < 3 || chest_count < 3 || enemy_count < 5 {
            let x = rng.gen_range(0..WIDTH);
            let y = rng.gen_range(0..HEIGHT);

            if self.0[y][x] == Block::Air {
                let mut directions = vec![
                    crate::direction::Direction::Up,
                    crate::direction::Direction::Down,
                    crate::direction::Direction::Left,
                    crate::direction::Direction::Right,
                ];
                directions.shuffle(&mut rng);

                let mut valid_direction = None;
                let mut wall_count = 0;
                for direction in directions.clone() {
                    let (dx, dy) = direction.get_forward_offset();
                    let (.., block) = self.get_neighbour_block(x, y, dx, dy);
                    if block == &Block::Wall {
                        wall_count += 1;
                    }
                    if block == &Block::Air {
                        valid_direction = Some(direction);
                    }
                }

                if valid_direction.is_some() && wall_count < 2 {
                    if door_count < 3 && rng.gen_bool(0.5) {
                        if let Some(direction) = valid_direction {
                            self.0[y][x] = Block::Door(direction, crate::items::key::Key::Brozen);
                            door_count += 1;
                        }
                    } else if chest_count < 3 && valid_direction.is_some() {
                        if let Some(direction) = valid_direction {
                            self.0[y][x] = Block::Case(direction);
                            chest_count += 1;
                        }
                    } else if enemy_count < 5 {
                        if let Some(direction) = valid_direction {
                            let name = format!(
                                "Enemy #{}",
                                (0..10000000000usize).choose(&mut rng).unwrap()
                            );
                            let health = (10..=25).choose(&mut rng).unwrap();
                            self.0[y][x] = Block::Enemy(Enemy::new(name, health, direction));
                            enemy_count += 1;
                        }
                    }
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
    ) -> (usize, usize, &Block) {
        match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
            (Some(x), Some(y)) if x < WIDTH && y < HEIGHT => (x, y, &self[(x, y)]),
            _ => (0, 0, &Block::Wall),
        }
    }

    pub(crate) fn get_neighbour_block_mut(
        &mut self,
        x: usize,
        y: usize,
        dx: isize,
        dy: isize,
    ) -> Option<(usize, usize, &mut Block)> {
        match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
            (Some(x), Some(y)) if x < WIDTH && y < HEIGHT => Some((x, y, &mut self[(x, y)])),
            _ => None,
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

impl IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        if x < WIDTH && y < HEIGHT {
            &mut self.0[y][x]
        } else {
            panic!("Out of bounds")
        }
    }
}
