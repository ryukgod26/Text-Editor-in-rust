use crate::prelude::*;
use std::collections::HashMap;
use super::{Annotation, AnnotationType, Line, SyntaxHighlighter};

#[derive(Default)]
pub struct RustSyntaxHighlighter{
    highlights: HashMap<LineIdx, Vec<Annotation>>,
}

impl RustSyntaxHighlighter{
    fn highlight_digits(line:&Line, result:&mut Vec<Annotation>){
        lines.chars().enumerate().for_each( |(idx, ch)| {
            if ch.is_ascii_digit(){
                result.push(Annotation{
                    annotation_type: AnnotationType::Digit,
                    start: idx,
                    end: idx.saturating_add(1),
                });
            }
        });
    }
}

impl SyntaxHighlighter for RustSyntaxHighlighter{
    fn highlight(&mut self, line_idx: LineIdx, line: &Line){
        let mut result = Vec::new();
        Self::highlight_digits(line, &mut result);
        self.highlights.insert(line_idx, result);
    }

    fn get_annotations(&self, line_idx: LineIdx) -> Option<&Vex<Annotation>>{
        self.highlights.get(&line_idx)
    }
}
