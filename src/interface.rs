use crate::battle::Battle;

#[derive(Debug, Default)]
pub(crate) enum State {
    #[default]
    Game,
    Inventory,
    Battle(Battle),
}

#[derive(Debug, Default)]
pub(crate) struct Interface {
    current: State,
    previous: Vec<State>,
}

impl Interface {
    pub(crate) fn get_current(&self) -> &State { &self.current }

    pub(crate) fn go_to(&mut self, state: State) {
        self.previous
            .push(std::mem::replace(&mut self.current, state));
    }

    pub(crate) fn go_back(&mut self) -> bool {
        self.previous
            .pop()
            .map(|state| self.current = state)
            .is_some()
    }
}
