#[derive(Copy,Copy,Default)]
pub struct Position{
    col: usize,
    row: usize,
}

impl Position{
    pub const fn saturating_sub(self, other: usize) -> Self{
        Self{
            row: self.row.saturating_sub(other.row),
            col: self.col.saturating_sub(other.col),
        }
    }
}
