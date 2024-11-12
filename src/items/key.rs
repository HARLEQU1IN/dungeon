use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Key {
    Brozen,
    Silver,
    Gold,
}

impl Key {
    pub(crate) fn display_cell(&self) -> &str {
        use Key::*;
        match self {
            Brozen => "🔑 ",
            Silver => "🗝️\u{3000}",
            Gold => "🔐 ",
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Key::*;
        match self {
            Brozen => write!(f, "Бронзовый ключ"),
            Silver => write!(f, "Серебряный ключ"),
            Gold => write!(f, "Золотой ключ"),
        }
    }
}
