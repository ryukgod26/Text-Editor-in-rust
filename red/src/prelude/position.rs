use super::{ColIdx, RowIdx};

#[derive(Copy,Clone,Default)]
pub struct Position{
    pub col: ColIdx,
    pub row: RowIdx,
}

impl Position{
    pub const fn saturating_sub(self, other: Self) -> Self{
        Self{
            row: self.row.saturating_sub(other.row),
            col: self.col.saturating_sub(other.col),
        }
    }
}
