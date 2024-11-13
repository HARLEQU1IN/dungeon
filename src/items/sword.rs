use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Sword {
    ShadowDaggers,
    Dagger,
}
impl Sword {
    pub(crate) fn display_cell(&self) -> &str {
        use Sword::*;
        match self {
            Dagger => "🗡\u{3000}",
            ShadowDaggers => "⚔\u{3000}",
        }
    }
}

impl fmt::Display for Sword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Sword::*;
        match self {
            Dagger => write!(f, "Кинжал"),
            ShadowDaggers => write!(f, "Два кинжала"),
        }
    }
}
