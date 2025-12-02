use super::Annotation;
use super::Line;
use crate::prelude::*;

pub trait SyntaxHighlighter{
    fn highlight(&mut self, line_idx: LineIdx, line: &Line);
    fn get_annotations(&self, line_idx: LineIdx) -> Option<&Vec<Annotation>>;
}
