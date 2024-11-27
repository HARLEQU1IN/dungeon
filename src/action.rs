use crate::{
    blocks::Block,
    interface::State,
    items::{key::Key, sword::Sword},
    player::Player,
};

#[derive(Debug, Default, PartialEq, Eq)]
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
    Loss,
    UnknowCommand,
    OpenedInventory,
    ClosedInventory,
    Attack,
    EnemyDefeated,
    HitEnemy,
    NoAttack,
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
                println!("Вы нашли сундук!");
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
            Loss => {
                println!("Вы проиграли");
                return false;
            },
            UnknowCommand => {
                println!("Такой команды нету!");
            },
            HitEnemy => {
                println!("Вы встретили врага!")
            },
            Attack => {
                let sword = Sword::Dagger;
                println!(
                    "{} наносит урон: {} и имеет шанс попадания: {:.0}%",
                    sword,
                    sword.damage(),
                    sword.hit_chance() * 100.0
                );
                let dual_sword = Sword::ShadowDaggers;
                println!(
                    "{} наносит урон: {} и имеет шанс попадания: {:.0}%",
                    dual_sword,
                    dual_sword.damage(),
                    dual_sword.hit_chance() * 100.0
                );
            },
            EnemyDefeated => {
                println!("Вы победили врага!");
            },
            NoAttack => {
                println!("Это не враг, нельзя атаковать");
            },
        }
        true
    }

    pub fn process_input(&mut self, input: String, player: &mut Player) {
        *self = match player.get_current_status() {
            State::Game => {
                match input.as_str() {
                    "a" | "left" | "turn left" => player.turn_left(),
                    "d" | "right" | "turn right" => player.turn_right(),
                    "s" | "back" | "turn back" => player.turn_back(),
                    "w" | "forward" => player.move_forward(),
                    "o" | "open door" => player.open_door(),
                    "i" | "inventory" | "open inventory" => player.open_inventory(),
                    "h" | "attack" => player.attack_enemy(),
                    _ => Action::UnknowCommand,
                }
            },
            State::Inventory => {
                match input.as_str() {
                    "q" | "go back" => player.close_inventory(),
                    _ => Action::UnknowCommand,
                }
            },
            State::Battle(battle) => todo!("{battle}"),
        }
    }
}
