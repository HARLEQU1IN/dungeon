use input_macro::input;

mod exit;
mod wall;

#[derive(Clone, Copy)]
pub enum Block {
    Wall,
    Air,
    Door,
}

use Block::*;

pub const HEIGHT: usize = 11;
pub const WIDTH: usize = 11;
pub static MAP: [[Block; 11]; 11] = [
    [
        Wall, Door, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall,
    ],
    [Door, Air, Air, Wall, Air, Air, Wall, Air, Air, Wall, Wall],
    [Door, Air, Wall, Wall, Air, Air, Air, Air, Wall, Wall, Wall],
    [Wall, Air, Air, Air, Air, Wall, Air, Air, Air, Air, Wall],
    [
        Wall, Wall, Wall, Wall, Wall, Wall, Wall, Air, Air, Air, Wall,
    ],
    [Wall, Air, Air, Air, Wall, Air, Wall, Air, Air, Air, Wall],
    [Wall, Air, Air, Air, Wall, Air, Air, Air, Air, Air, Wall],
    [Wall, Air, Air, Air, Air, Air, Wall, Air, Air, Air, Wall],
    [Wall, Air, Air, Air, Air, Air, Wall, Air, Air, Air, Wall],
    [Wall, Air, Air, Air, Air, Air, Wall, Air, Air, Air, Wall],
    [
        Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall, Wall,
    ],
];

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
enum DoorDirection {
    Up,
    Down,
    Left,
    Right,
}

struct Player {
    direction: Direction,
    x: usize,
    y: usize,
}

enum Action {
    None,
    Moved,
    Rotated,
    HitDoor,
    HitWall,
    UnknowCommand,
}

fn main() {
    let mut player = Player {
        direction: Direction::Down,
        x: 1,
        y: 1,
    };
    let mut last_action = Action::None;
    loop {
        let d = match player.direction {
            Direction::Left => [
                (-3, 2),
                (-3, -2),
                (-3, 1),
                (-3, -1),
                (-3, 0),
                (-2, 1),
                (-2, -1),
                (-2, 0),
                (-1, 1),
                (-1, -1),
                (-1, 0),
                (0, 1),
                (0, -1),
            ],
            Direction::Right => [
                (3, -2),
                (3, 2),
                (3, -1),
                (3, 1),
                (3, 0),
                (2, -1),
                (2, 1),
                (2, 0),
                (1, -1),
                (1, 1),
                (1, 0),
                (0, -1),
                (0, 1),
            ],
            Direction::Up => [
                (-2, -3),
                (2, -3),
                (-1, -3),
                (1, -3),
                (0, -3),
                (-1, -2),
                (1, -2),
                (0, -2),
                (-1, -1),
                (1, -1),
                (0, -1),
                (-1, 0),
                (1, 0),
            ],
            Direction::Down => [
                (2, 3),
                (-2, 3),
                (1, 3),
                (-1, 3),
                (0, 3),
                (1, 2),
                (-1, 2),
                (0, 2),
                (1, 1),
                (-1, 1),
                (0, 1),
                (1, 0),
                (-1, 0),
            ],
        };

        let mut frame = [[' '; 26]; 14];
        for (i, (dx, dy)) in d.iter().enumerate() {
            let x = player.x.checked_add_signed(*dx);
            let y = player.y.checked_add_signed(*dy);
            let block = match (x, y) {
                (Some(x), Some(y)) if x < WIDTH && y < HEIGHT => MAP[y][x],
                _ => Block::Wall,
            };
            let pattern = match block {
                Block::Wall => wall::PATTERNS[i],
                Block::Door => {
                    exit::PATTERNS[i * 4 + (player.direction as usize)]
                },
                _ => continue,
            };
            for (i, line) in pattern.lines().enumerate() {
                for (j, char) in line.chars().enumerate() {
                    if char != '`' {
                        frame[i][j] = char;
                    }
                }
            }
        }
        print!("\x1Bc");
        for line in frame {
            for char in line {
                print!("{}", char);
            }
            println!();
        }

        match last_action {
            Action::None => {}
            Action::Rotated => {}
            Action::Moved => {}
            Action::HitDoor => println!("U WINNER!!!"),
            Action::HitWall => println!("You hit a wall!"),
            Action::UnknowCommand => println!("Unknown command. Try again!"),
        }

        let n: String = input!(":> ").to_lowercase();
        last_action = match n.as_str() {
            "d" | "right" => {
                player.direction = match player.direction {
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                };
                Action::Rotated }
            "a" | "left" => {
                player.direction = match player.direction {
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                };
                Action::Rotated
            }
            "w" | "forward" => {
                let (dx, dy) = match player.direction {
                    Direction::Right => (1, 0),
                    Direction::Left => (-1, 0),
                    Direction::Up => (0, -1),
                    Direction::Down => (0, 1),
                };
                let x = player.x.checked_add_signed(dx);
                let y = player.y.checked_add_signed(dy);
                let block = match (x, y) {
                    (Some(x), Some(y)) if x < WIDTH && y < HEIGHT => MAP[y][x],
                    _ => Block::Wall,
                };
                match block {
                    Block::Air => {
                        player.x = x.unwrap();
                        player.y = y.unwrap();
                        Action::Moved
                    }
                    Block::Wall => Action::HitWall,
                    Block::Door => Action::HitDoor,
                }
            }
            _ => Action::UnknowCommand,
        }
    }
}
