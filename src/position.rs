/// [A][C][E][D][B]
///    [F][H][G]
///    [I][K][J]
///    [L][^][M]
#[derive(Debug, Clone, Copy)]
pub(crate) enum Position {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
}

pub(crate) const POSITIONS_COUNT: usize = 13;
