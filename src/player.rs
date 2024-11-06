use {
    crate::{
        action::Action,
        blocks::Block,
        direction::Direction,
        frame::{Frame, GAME_HEIGHT, GAME_WIDTH, INVENTORY_HEIGHT, INVENTORY_WIDTH},
        inventory::Inventory,
        items::Item,
        map::Map,
        status::{PlayerStatus, Status},
    },
    std::fmt,
};

pub(crate) struct Player {
    map: Map,
    x: usize,
    y: usize,
    direction: Direction,
    pub inventory: Inventory,
    status: PlayerStatus,
}

impl Player {
    pub(crate) fn new(map: Map) -> Self {
        Self {
            map,
            x: 1,
            y: 1,
            direction: Default::default(),
            inventory: Default::default(),
            status: PlayerStatus::new(),
        }
    }

    pub(crate) fn get_current_status(&self) -> Status { self.status.get_current() }

    pub(crate) fn tunr_left(&mut self) -> Action {
        self.direction.turn_left();
        Action::TurnLeft
    }

    pub(crate) fn turn_right(&mut self) -> Action {
        self.direction.turn_right();
        Action::TurnRight
    }

    pub(crate) fn turn_back(&mut self) -> Action {
        self.direction.turn_back();
        Action::TurnBack
    }

    pub(crate) fn move_forward(&mut self) -> Action {
        let (dx, dy) = self.direction.get_forward_offset();
        let (x, y, block) = self.map.get_neighbour_block(self.x, self.y, dx, dy);
        match block {
            Block::Air => {
                self.x = x;
                self.y = y;
                Action::MoveForward
            },
            Block::Wall => Action::HitWall,

            Block::Door(..) => Action::HitDoor,
        }
    }

    pub(crate) fn open_door(&mut self) -> Action {
        let (dx, dy) = self.direction.get_forward_offset();
        let (.., block) = self.map.get_neighbour_block(self.x, self.y, dx, dy);
        match block {
            Block::Door(exit_direction, exit_key) => {
                let direction = (self.direction as isize - exit_direction as isize).rem_euclid(4);
                match direction {
                    0 | 2 => {
                        if self.inventory.has_item(Item::Key(exit_key)) {
                            Action::Win
                        } else {
                            Action::Closed(exit_key)
                        }
                    },
                    1 | 3 => Action::HitWall,
                    _ => unreachable!("Impossible because module by 4"),
                }
            },
            block => Action::NonOpen(block),
        }
    }

    pub(crate) fn open_inventory(&mut self) -> Action {
        self.status.go_to(Status::Inventory);
        Action::OpenedInventory
    }

    pub(crate) fn close_inventory(&mut self) -> Action {
        self.status.go_back();
        Action::ClosedInventory
    }

    pub(crate) fn render(&self) -> Box<dyn fmt::Display> {
        match self.status.get_current() {
            Status::Game => Box::new(self.render_game()),
            Status::Inventory => Box::new(self.render_inventory()),
        }
    }

    fn render_game(&self) -> Frame<GAME_WIDTH, GAME_HEIGHT> {
        let offsets = self.direction.get_render_offsets();
        let mut frame = Frame::default();
        for (position, dx, dy) in offsets {
            let (.., block) = self.map.get_neighbour_block(self.x, self.y, dx, dy);
            let pattern = block.get_pattern(position, self.direction);
            frame.add_layer(pattern);
        }
        frame
    }

    fn render_inventory(&self) -> Frame<INVENTORY_WIDTH, INVENTORY_HEIGHT> {
        let mut frame = Frame::default();
        let pattern = self.inventory.get_pattern();
        frame.add_layer(pattern.as_str());
        frame
    }
}
