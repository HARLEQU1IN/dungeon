mod action;
mod blocks;
mod direction;
mod frame;
mod inventory;
mod items;
mod map;
mod player;
mod position;
mod status;

use {action::Action, input_macro::input, map::Map, player::Player};

fn main() {
    let map = Map::new();
    let mut player = Player::new(map);
    let mut action = Action::default();
    player
        .inventory
        .add_item(items::Item::Key(items::key::Key::Brozen));
    player
        .inventory
        .add_item(items::Item::Key(items::key::Key::Silver));
    player
        .inventory
        .add_item(items::Item::Key(items::key::Key::Gold));
    player
        .inventory
        .add_item(items::Item::Key(items::key::Key::Gold));
    player
        .inventory
        .remove_item(items::Item::Key(items::key::Key::Gold));

    loop {
        println!("{frame}", frame = player.render());

        if !action.update() {
            break;
        }

        let input = input!(":> ").to_lowercase();
        action.process_input(input, &mut player);
    }
}
