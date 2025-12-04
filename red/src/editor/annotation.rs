use super::AnnotationType;
use crate::prelude::*;

#[derive(Copy,Clone,Debug)]
#[allow(clippy::struct_field_names)]
pub struct Annotation{
    pub annotation_type: AnnotationType,
    pub start: usize,
    pub end: usize,
}

impl Annotation{
    pub fn shift(&mut self, offset: ByteIdx) {
        self.start = self.start.saturating_add(offset);
        self.end = self.end.saturating_add(offset);
    }
}
