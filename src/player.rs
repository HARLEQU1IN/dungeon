use crate::{
    action::Action,
    blocks::Block,
    direction::Direction,
    frame::{Frame, GAME_HEIGHT, GAME_WIDTH, INVENTORY_HEIGHT, INVENTORY_WIDTH},
    inventory::Inventory,
    items::Item,
    map::Map,
    status::{Interface, State},
};

/// a
const MAX_HEALTH_POINTS: usize = 3;
const COUNT_STEPS: usize = 50;

/// Структура игрока
pub(crate) struct Player {
    map: Map,
    x: usize,
    y: usize,
    direction: Direction,
    /// Количество жизней игрока
    pub health_points: usize,
    /// Инвентарь игрока
    pub inventory: Inventory,
    interface: Interface,
    /// Подсчёт шагов
    pub steps: usize,
}

impl Player {
    pub(crate) fn new(map: Map) -> Self {
        Self {
            map,
            x: 1,
            y: 1,
            direction: Default::default(),
            inventory: Default::default(),
            interface: Interface::new(),
            health_points: MAX_HEALTH_POINTS,
            steps: 0,
        }
    }

    pub(crate) fn get_current_status(&self) -> State { self.interface.get_current() }

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

                self.steps += 1;

                if self.steps > COUNT_STEPS {
                    self.steps = 0;
                    self.health_points -= 1;
                    if self.health_points == 0 {
                        return Action::Loss;
                    }
                }

                Action::MoveForward
            },
            Block::Wall => Action::HitWall,
            Block::Door(..) => Action::HitDoor,
            Block::Case(..) => Action::HitCase,
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
        self.interface.go_to(State::Inventory);
        Action::OpenedInventory
    }

    pub(crate) fn close_inventory(&mut self) -> Action {
        self.interface.go_back();
        Action::ClosedInventory
    }

    pub(crate) fn render(&self) -> String {
        let pattern = match self.interface.get_current() {
            State::Game => format!("{}", self.render_game()),
            State::Inventory => format!("{}", self.render_inventory()),
        };
        format!("{}\n{}", pattern, self.render_hp())
    }

    fn render_game(&self) -> Frame<GAME_WIDTH, GAME_HEIGHT> {
        let offsets = self.direction.get_render_offsets();
        let mut frame = Frame::default();

        // Отрисовка окружения
        for (position, dx, dy) in offsets {
            let (.., block) = self.map.get_neighbour_block(self.x, self.y, dx, dy);
            let pattern = block.get_pattern(position, self.direction);
            frame.add_layer(pattern);
        }
        frame
    }

    fn render_hp(&self) -> String {
        let hearts = "❤️ ".repeat(self.health_points);
        let empty_hearts = "🤍".repeat(MAX_HEALTH_POINTS - self.health_points);
        format!("HP: {}{}", hearts, empty_hearts)
    }

    fn render_inventory(&self) -> Frame<INVENTORY_WIDTH, INVENTORY_HEIGHT> {
        let mut frame = Frame::default();
        let pattern = self.inventory.get_pattern();
        frame.add_layer(pattern.as_str());
        frame
    }
}
