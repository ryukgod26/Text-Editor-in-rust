pub type GraphemeIdx = usize;
pub type LineIdx = usize;
pub type ByteIdx = usize;
pub type ColIdx = usize;
pub type RowIdx = usize;

mod position;
mod location;
mod size;

pub use position::Position;
pub use location::Location;
pub use size::Size;
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");