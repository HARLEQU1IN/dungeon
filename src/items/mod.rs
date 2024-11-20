pub mod key;
pub mod sword;

use {
    key::Key,
    std::fmt::{self},
    sword::Sword,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Item {
    #[default]
    None,
    Key(Key),
    Sword(Sword),
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Item::*;
        match self {
            None => write!(f, "   "),
            Key(key) => write!(f, "{}", key.display_cell()),
            Sword(sword) => write!(f, "{}", sword.display_cell()),
        }
    }
}
