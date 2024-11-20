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

    pub(crate) fn damage(&self) -> u32 {
        use Sword::*;
        match self {
            Dagger => 5,
            ShadowDaggers => 8,
        }
    }

    pub(crate) fn hit_chance(&self) -> f32 {
        use Sword::*;
        match self {
            Dagger => 0.75,
            ShadowDaggers => 0.65,
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
