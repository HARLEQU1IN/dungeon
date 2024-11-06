#[derive(Debug)]
pub(crate) struct PlayerStatus {
    current: Status,
    previous: Vec<Status>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Status {
    Game,
    Inventory,
}

impl PlayerStatus {
    pub(crate) fn new() -> Self {
        Self {
            current: Status::Game,
            previous: vec![],
        }
    }

    pub(crate) fn get_current(&self) -> Status { self.current }

    pub(crate) fn go_back(&mut self) -> bool {
        let Some(previous) = self.previous.pop() else {
            return false;
        };
        self.current = previous;
        true
    }

    pub(crate) fn go_to(&mut self, tab: Status) {
        self.previous
            .push(std::mem::replace(&mut self.current, tab));
    }
}
