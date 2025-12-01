use std::fmt::{Display, Formatter, Result};

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub enum FileType{
    Rust,
    Text,
}

impl Display for FileType{
    fn fnt(&self, formatter: &mut Formatter<'_>) -> Result{
        match self{
            Self::Rust => write!(formatter, "Rust"),
            Self::Text => write!(formatter, "Text"),
        }
    }
}
