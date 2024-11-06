pub mod key;

use {key::Key, std::fmt};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Item {
    #[default]
    None,
    Key(Key),
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Item::*;
        match self {
            None => write!(f, "   "),
            Key(key) => write!(f, "{}", key.display_cell()),
        }
    }
}
