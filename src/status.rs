#[derive(Debug)]
pub(crate) struct Interface {
    current: State,
    previous: Vec<State>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum State {
    Game,
    Inventory,
}

impl Interface {
    pub(crate) fn new() -> Self {
        Self {
            current: State::Game,
            previous: vec![],
        }
    }

    pub(crate) fn get_current(&self) -> State { self.current }

    pub(crate) fn go_back(&mut self) -> bool {
        let Some(previous) = self.previous.pop() else {
            return false;
        };
        self.current = previous;
        true
    }

    pub(crate) fn go_to(&mut self, tab: State) {
        self.previous
            .push(std::mem::replace(&mut self.current, tab));
    }
}
