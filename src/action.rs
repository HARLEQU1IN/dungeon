use crate::{blocks::Block, items::key::Key, player::Player, status::Status};

#[derive(Debug, Default)]
pub enum Action {
    #[default]
    None,
    MoveForward,
    TurnLeft,
    TurnRight,
    TurnBack,
    HitWall,
    HitDoor,
    HitCase,
    Closed(Key),
    NonOpen(Block),
    Win,
    UnknowCommand,
    OpenedInventory,
    ClosedInventory,
}

impl Action {
    pub fn update(&self) -> bool {
        use Action::*;
        match self {
            None => {},
            MoveForward => {},
            TurnLeft => {},
            TurnRight => {},
            TurnBack => {},
            HitWall => {
                println!("Вы упёрлись в стену!");
            },
            HitDoor => {
                println!("Вы упёрлись в дверь!");
            },
            HitCase => {
                println!("Вы нашли сундук!")
            },
            Closed(key) => {
                println!("У вас нет [{key}] для открытия двери.");
            },
            NonOpen(block) => {
                println!("Нельзя открыть [{block}]");
            },
            OpenedInventory => {},
            ClosedInventory => {},
            Win => {
                println!("Вы победили!");
                return false;
            },
            UnknowCommand => {
                println!("Такой команды нету!");
            },
        }
        true
    }

    pub fn process_input(&mut self, input: String, player: &mut Player) {
        *self = match player.get_current_status() {
            Status::Game => {
                match input.as_str() {
                    "a" | "left" | "turn left" => player.tunr_left(),
                    "d" | "right" | "turn right" => player.turn_right(),
                    "s" | "back" | "turn back" => player.turn_back(),
                    "w" | "forward" => player.move_forward(),
                    "o" | "open door" => player.open_door(),
                    "i" | "inventory" | "open inventory" => player.open_inventory(),
                    _ => Action::UnknowCommand,
                }
            },
            Status::Inventory => {
                match input.as_str() {
                    "q" | "go back" => player.close_inventory(),
                    _ => Action::UnknowCommand,
                }
            },
        }
    }
}
